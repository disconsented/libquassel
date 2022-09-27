{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, advisory-db, fenix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        inherit (pkgs) lib;

        rustToolchain = with fenix.packages.${system};
          combine [
            stable.defaultToolchain
            (stable.withComponents [ "rust-src" ])

            rust-analyzer
          ];

        craneLib = crane.lib.${system}.overrideToolchain rustToolchain;

        src = lib.cleanSourceWith {
          # Apply the default source cleaning from nixpkgs
          src = lib.cleanSource ./.;

          # Then add our own filter on top
          filter = orig_path: type:
            let
              path = (toString orig_path);
              base = baseNameOf path;
              parentDir = baseNameOf (dirOf path);

              matchesSuffix = lib.any (suffix: lib.hasSuffix suffix base) [
                ".rs"
                ".toml"
                ".md"
              ];

              # Cargo.toml already captured above
              isCargoFile = base == "Cargo.lock";

              # .cargo/config.toml already captured above
              isCargoConfig = parentDir == ".cargo" && base == "config";
            in type == "directory" || matchesSuffix || isCargoFile
            || isCargoConfig;
        };

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        cargoArtifacts = craneLib.buildDepsOnly { inherit src; };

        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        libquassel = craneLib.buildPackage {
          inherit cargoArtifacts src;

          # nativeBuildInputs = [ ./README.md ];
        };
      in {
        checks = {
          # Build the crate as part of `nix flake check` for convenience
          inherit libquassel;

          # Run clippy (and deny all warnings) on the crate source,
          # again, resuing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          libquassel-clippy = craneLib.cargoClippy {
            inherit cargoArtifacts src;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          };

          libquassel-doc = craneLib.cargoDoc { inherit cargoArtifacts src; };

          # Check formatting
          libquassel-fmt = craneLib.cargoFmt { inherit src; };

          # Audit dependencies
          libquassel-audit = craneLib.cargoAudit { inherit src advisory-db; };

          # Run tests with cargo-nextest
          # Consider setting `doCheck = false` on `libquassel` if you do not want
          # the tests to run twice
          libquassel-nextest = craneLib.cargoNextest {
            inherit cargoArtifacts src;
            partitions = 1;
            partitionType = "count";
          };
        } // lib.optionalAttrs (system == "x86_64-linux") {
          # NB: cargo-tarpaulin only supports x86_64 systems
          # Check code coverage (note: this will not upload coverage anywhere)
          libquassel-coverage =
            craneLib.cargoTarpaulin { inherit cargoArtifacts src; };
        };

        packages.default = libquassel;

        # apps.default = flake-utils.lib.mkApp {
        #   drv = libquassel;
        # };

        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks;

          # Extra inputs can be added here
          nativeBuildInputs = with pkgs; [ rustToolchain ];
        };
      });
}
