use hyper;
use std::io;
use rustc_serialize::json;


impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Hyper(err)
    }
}

impl From<json::ParserError> for Error {
    fn from(err: json::ParserError) -> Error {
        Error::Parser(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

pub enum Error {
    Hyper(hyper::Error),
    Parser(json::ParserError),
    Io(io::Error),
    NoResult,
    NoChannels,
    NoMembers,
    UserNotFound,
    ChannelNotFound,
    CantReadCache,
    CantWriteCache,
}
