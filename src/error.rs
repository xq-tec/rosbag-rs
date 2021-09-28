use std::fmt;
use std::convert::From;
use crate::cursor::OutOfBounds;

/// The error type for ROS bag file reading and parsing.
#[derive(Debug)]
pub enum Error {
    InvalidHeader,
    InvalidRecord,
    UnsupportedVersion,
    UnsupportedCompression,
    OutOfBounds,
}

impl From<OutOfBounds> for Error {
    fn from(_: OutOfBounds) -> Error {
        Error::OutOfBounds
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        let s = match self {
            InvalidHeader => "invalid header",
            InvalidRecord => "invalid record",
            UnsupportedVersion => "unsupported version",
            UnsupportedCompression => "unsupported compression type",
            OutOfBounds => "out of bounds",
        };
        write!(f, "rosbag::Error: {}", s)
    }
}

impl std::error::Error for Error { }