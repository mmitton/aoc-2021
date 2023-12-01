use std::ffi::OsString;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum Error {
    Fmt(std::fmt::Error),
    IO(std::io::Error),
    InvalidInput(String),
    InvalidInputFile(OsString),
    MinReq(minreq::Error),
    MissingCookies,
    MissingInput,
    ParseIntError(ParseIntError),
    Runner(String),
    SearchUpFailed(String),
    Unsolved,
    YearExists(usize),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<std::fmt::Error> for Error {
    fn from(e: std::fmt::Error) -> Self {
        Self::Fmt(e)
    }
}

impl From<minreq::Error> for Error {
    fn from(e: minreq::Error) -> Self {
        Self::MinReq(e)
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}