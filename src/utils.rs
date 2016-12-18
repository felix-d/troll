use hyper;
use errors::Error;
use std::io::Read;
use rustc_serialize::json;
use std::collections::BTreeMap;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper::header::{Headers, ContentType};


pub fn get(url: &str) -> Result<BTreeMap<String, json::Json>, Error> {
    let client = hyper::Client::new();

    let mut res = try!(client.get(url).send());
    let mut buffer = String::new();
    try!(res.read_to_string(&mut buffer));

    let data  = try!(json::Json::from_str(&buffer));
    let parsed_result = try!(data.as_object().ok_or(Error::NoResult)).clone();

    Ok(parsed_result)
}

pub fn post(url: &str, payload: &str) -> Result<(), Error> {
    let client = hyper::Client::new();
    let mut headers = Headers::new();
    headers.set(
        ContentType(Mime(TopLevel::Application, SubLevel::WwwFormUrlEncoded, vec![(Attr::Charset, Value::Utf8)]))
    );

    let mut res = try!(client.post(url).headers(headers).body(payload).send());
    let mut buffer = String::new();
    try!(res.read_to_string(&mut buffer));
    println!("{}", buffer);
    Ok(())
}
