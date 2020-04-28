 #[derive(Debug, Fail)]
pub enum ProtocolError {
    #[fail(display = "message has wrong type")]
    WrongMsgType,
    #[fail(display = "bool value is neither 0 nor 1")]
    BoolOutOfRange,
    #[fail(display = "QVariant is not known")]
    UnknownVariant,
    #[fail(display = "wrong variant has been given")]
    WrongVariant,
    #[fail(display = "io error")]
    IOError(std::io::Error),
    #[fail(display = "could not convert from int")]
    TryFromIntError(std::num::TryFromIntError),
    #[fail(display = "utf8 error")]
    Utf8Error(std::string::FromUtf8Error),
 }

// impl std::error::Error for ErrorKind {}
//
// impl std::convert::From<std::io::Error> for ErrorKind {
//     fn from(error: std::io::Error) -> Self {
//         ErrorKind::IOError(error)
//     }
// }
//
// impl std::convert::From<std::num::TryFromIntError> for ErrorKind {
//     fn from(error: std::num::TryFromIntError) -> Self {
//         ErrorKind::TryFromIntError(error)
//     }
// }
//
// impl std::convert::From<std::string::FromUtf8Error> for ErrorKind {
//     fn from(error: std::string::FromUtf8Error) -> Self {
//         ErrorKind::Utf8Error(error)
//     }
// }
