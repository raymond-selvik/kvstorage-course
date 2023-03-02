use std::{fmt, io, error};


#[derive(Debug)]
pub enum KvsError  {
    Io(io::Error),
    KeyNotFound
}

impl fmt::Display for KvsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            KvsError::Io(ref err) => write!(f, "IO error: {}", err),
            KvsError::KeyNotFound => write!(f, "Key not found"),
        }
    }
}

impl error::Error for KvsError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            KvsError::Io(ref err) => Some(err),
            KvsError::KeyNotFound => None
        }
    }
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> KvsError {
        KvsError::Io(err)
    }
}

pub type Result<T> = std::result::Result<T, KvsError>;