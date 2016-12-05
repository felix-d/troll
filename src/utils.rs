use hyper;
use errors::Error;
use std::io::Read;
use rustc_serialize::json;
use std::collections::BTreeMap;


pub fn get(url: &str) -> Result<BTreeMap<String, json::Json>, Error> {
    let client = hyper::Client::new();

    let mut res = try!(client.get(url).send());
    let mut buffer = String::new();
    try!(res.read_to_string(&mut buffer));

    let data  = try!(json::Json::from_str(&buffer));
    let parsed_result = try!(data.as_object().ok_or(Error::NoResult)).clone();

    Ok(parsed_result)


    // let members = obj.get("members").unwrap();
    // println!("{:?}", members);
    // for member in members {
    //     println!("{:?}", member);
    //     println!("");
    //     println!("");

    //     // println!("{:?}", (*member).as_object().unwrap());
    // }

    // // println!("{}", members);


    // for (key, value) in obj.iter() {
    //     println!("{}: {}", key, match *value {
    //         Json::U64(v) => format!("{} (u64)", v),
    //         Json::String(ref v) => format!("{} (string)", v),
    //         _ => format!("other")
    //     });
    // }

    // println!("{}", data);
    // response_body
    // "foo".to_string()
}
