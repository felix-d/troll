use errors::Error;
use rustc_serialize::json;

pub fn get(key: &str) -> Result<Option<json::Json>, Error> {
    Err(Error::CantReadCache)
}

pub fn set(key: &str, json: &json::Json) -> Result<(), Error> {
    Err(Error::CantWriteCache)
}

