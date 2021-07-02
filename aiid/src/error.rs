/// a simple struct(String) for reporting aiid errors
#[derive(Debug, PartialEq, Clone)]
pub struct aiidError(pub String);

/// aiid Result type
pub type aiidResult<T> = Result<T, aiidError>;

impl std::fmt::Display for aiidError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for aiidError {
    fn description(&self) -> &str {
        &self.0
    }
    fn cause(&self) -> Option<&std::error::Error> {
        None
    }
}

impl From<String> for aiidError {
    fn from(error: String) -> Self {
        Self(error)
    }
}

impl<'a> From<&'a str> for aiidError {
    fn from(error: &'a str) -> Self {
        Self(error.to_string())
    }
}

impl From<reed_solomon::DecoderError> for aiidError {
    fn from(error: reed_solomon::DecoderError) -> Self {
        Self(format!("{:?}", error))
    }
}

impl From<std::num::ParseIntError> for aiidError {
    fn from(error: std::num::ParseIntError) -> Self {
        Self(format!("{:?}", error))
    }
}
