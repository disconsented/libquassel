use libquassel_derive::Network;

#[derive(Debug, Clone, PartialEq, Network)]
#[network(repr = "list")]
pub struct CertManager {
    #[network(rename = "sslKey", variant = "ByteArray")]
    pub ssl_key: String,
    #[network(rename = "sslCert", variant = "ByteArray")]
    pub ssl_cert: String,
    // // C->S calls

    // /**
    //  * Replaces all properties of the object with the content of the
    //  * "properties" parameter. This parameter is in network representation.
    //  */
    // requestUpdate(properties: QVariantMap)

    // // S->C calls

    // setSslCert(encoded: QByteBuffer | null)
    // setSslKey(encoded: QByteBuffer | null)
    // /**
    //  * Replaces all properties of the object with the content of the
    //  * "properties" parameter. This parameter is in network representation.
    //  */
    // update(properties: QVariantMap)
}
