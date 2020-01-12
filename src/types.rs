#[allow(dead_code)]
pub mod basic;
pub mod variant;
pub mod handshake;

pub use basic::*;

pub trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

pub trait Deserialize {
    fn parse(&mut self, b: &[u8]) -> usize;
}


//       78 da 6c 54 6d 6b d3 50 14 7e 44 41 d1 09
// 7e 50 10 19 a3 14 91 89 2f db fa b2 75 32 05 cd
// 2c 16 3b 19 6b d9 f7 98 dc a6 61 69 12 93 db ce
// f8 6f fc a3 ea 73 ce 4d ca 64 23 dc e4 bc 3c e7
// 39 6f b7 05 ee fc 06 70 9f 67 03 22 78 49 6c 52
// 7b ec 5b 53 9b f1 18 7b e8 e3 00 03 7d 1f a0 8b
// 1e e5 1a ff d0 e1 cf 4d 51 c6 59 5a 87 dc da c0
// 0a bb 78 cb c0 1e de 20 47 01 83 16 b6 af d9 5e
// 51 1b 60 9f be 08 31 2c ed 47 f0 a9 cd d5 3b c3
// 7b b4 29 5b 3e 39 4a bc c3 0e 1f 87 9c 63 89 ef
// 64 0b 90 61 41 eb 0f ea 3e 31 25 e3 92 1b 74 87
// 5b 68 ec 0e 9b e8 31 73 87 0c 7b 08 99 7f 40 3d
// 60 be 0e e5 be 7a a4 49 83 43 3e 5d ea e2 e9 10
// d9 a7 d4 65 17 33 fa 76 a9 1f f2 2b 83 69 e3 c3
// 35 ce 23 e6 f1 69 7f d9 8c ea c1 d0 f8 76 59 98
// 71 5c 5a 51 c5 f6 84 e7 35 26 a8 90 b2 00 69 3b
// a3 14 e3 17 89 43 9c 90 a0 c0 05 e5 02 63 5a 53
// c8 52 9e 13 2f ad 25 f8 c8 26 65 14 86 1e 4b 7f
// 40 bb 7c 85 03 78 ba c6 7d c6 4f da 85 25 a5 25
// a1 6f 1b 5f 88 0b 69 1b a9 2d d0 b8 15 f5 6f 3c
// 16 97 e4 90 cc 25 b1 9b 38 ad 47 e9 ac 21 3c e6
// f4 19 17 69 3d 9b d4 7d 2e 48 22 23 22 84 e9 6a
// 1d 5b 38 d7 dc 31 c7 56 b1 26 91 57 fa 9e f0 19
// 13 d1 22 c3 92 fc 56 97 74 a6 d1 46 3b 76 0b 93
// 2a 5e b0 db 4b 7a 2a 0c b5 8a 85 a2 a6 8a 31 1a
// eb 53 ca b5 9e 9b e7 22 51 a5 ce ef 13 fd 33 5d
// 62 41 6c d3 bb 64 aa d6 bb 10 9c a7 31 86 36 37
// 2b 99 59 44 de a4 fe da 7a 3e 13 cd 14 2a df 69
// 7d 75 63 ce dc a8 bf c5 8e 0c 6b cb b4 ab 63 7a
// 4a bd 8e a9 ee 53 b2 0b a6 d9 91 e3 09 d9 a5 d1
// aa 97 ca 27 3c 8f 38 91 4c a7 de 74 2d 3b 3e d3
// fe e6 bc 2b 46 77 e4 eb 56 04 df 66 9f b2 d9 0b
// d6 9b d1 3a 24 32 a9 ef c1 94 5d e6 ca b0 c5 cc
// 01 f3 49 a4 a7 3e cb ea 73 7c a5 5c 29 cf b3 75
// de ff 73 8c 18 e5 ba 77 13 33 7a 33 dc c4 e4 56
// cd 28 d7 57 ff 5e 7d f5 85 ee 36 cf 9f bf 8d e7
// ee 49 19 4d ab fc ca 9f 8d a7 e3 75 ab 1b e9 8f
// c1 fe 13
