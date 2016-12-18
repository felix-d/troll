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

#[derive(Debug)]
pub enum Error {
    Hyper(hyper::Error),
    Parser(json::ParserError),
    Io(io::Error),
    UnexpectedJson,
    NoResult,
    NoChannels,
    NoMembers,
    UserNotFound,
    CacheKeyDoesNotExist,
    ChannelNotFound,
    CantReadCache,
    CantWriteCache,
    CantConvertJsonToObj,
}
