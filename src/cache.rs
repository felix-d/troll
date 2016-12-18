use errors::Error;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::fs::File;
use rustc_serialize::json;
use std::fs::OpenOptions;
use std::collections::BTreeMap;

const CACHE: &'static str = "/tmp/troll_cache";

pub struct Cache {
    handle: File,
    content: BTreeMap<String, json::Json>,
}

impl Cache {
    pub fn new() -> Result<Cache, Error> {
        let mut handle: File = try!(OpenOptions::new().read(true).write(true).create(true).open(CACHE));

        let content: BTreeMap<String, json::Json> = match Cache::file_content_as_json(&mut handle) {
            Ok(content) => content,
            Err(_) => {
                handle.set_len(0);
                handle.seek(SeekFrom::Start(0));
                BTreeMap::new()
            }
        };

        Ok(Cache { handle: handle, content: content })
    }

    fn file_content_as_json(file: &mut File) -> Result<BTreeMap<String, json::Json>, Error> {
        let mut cache_content = String::new();
        try!(file.read_to_string(&mut cache_content));

        let json_obj = try!(json::Json::from_str(&cache_content));
        let obj: BTreeMap<String, json::Json> = try!(json_obj.as_object().ok_or(Error::CantConvertJsonToObj)).clone();

        Ok(obj)
    }

    pub fn get(&mut self, key: &str) -> Option<json::Json> {
        match self.content.get(key) {
            Some(x) => Some(x.clone()),
            None => None,
        }
    }

    pub fn set(&mut self, key: &str, json: &json::Json) -> Result<(), Error> {
        self.content.remove(key);
        self.content.insert(key.to_string(), json.clone());

        try!(self.write_cache());

        Ok(())
    }

    fn write_cache(&mut self) -> Result<(), Error> {
        self.handle.set_len(0);
        self.handle.seek(SeekFrom::Start(0));

        let stringified = json::Json::Object(self.content.clone()).to_string();
        let bytes = stringified.as_bytes();
        try!(self.handle.write_all(bytes));

        Ok(())
    }
}
