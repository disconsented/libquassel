#[derive(Debug)]
pub enum ErrorKind {
    WrongMsgType,
    BoolOutOfRange,
    UnknownVariant,
    WrongVariant,
    IOError(std::io::Error),
    TryFromIntError(std::num::TryFromIntError),
    Utf8Error(std::string::FromUtf8Error),
}

impl std::convert::From<std::io::Error> for ErrorKind {
    fn from(error: std::io::Error) -> Self {
        ErrorKind::IOError(error)
    }
}

impl std::convert::From<std::num::TryFromIntError> for ErrorKind {
    fn from(error: std::num::TryFromIntError) -> Self {
        ErrorKind::TryFromIntError(error)
    }
}

impl std::convert::From<std::string::FromUtf8Error> for ErrorKind {
    fn from(error: std::string::FromUtf8Error) -> Self {
        ErrorKind::Utf8Error(error)
    }
}
