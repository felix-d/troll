use std::env;
use getopts::{Options};
use std::fmt;


const PROGRAM: &'static str = "troll";

pub struct Config {
    pub token: String,
    pub channel_name: String,
    pub username: String,
    pub message: String,
}

pub struct InvalidArgError;

impl fmt::Display for InvalidArgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid argument.")
    }
}

fn print_usage(opts: Options) {
    let brief = format!("Usage: ./{} [options]", PROGRAM);
    print!("{}", opts.usage(&brief));
}

pub fn parse_args() -> Result<Config, InvalidArgError> {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();

    opts.reqopt("t", "token", "You must provide the access token.", "TOKEN");
    opts.reqopt("c", "channel", "You must provide the channel name.", "CHANNEL");
    opts.reqopt("u", "username", "You must provide the username.", "USERNAME");
    opts.reqopt("m", "message", "You must provide the message you want to ouput.", "MESSAGE");
    opts.optflag("h", "help", "Print this help menu.");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_) => {
            print_usage(opts);
            return Err(InvalidArgError);
        }
    };

    let conf = Config {
        token: try!(matches.opt_str("t").ok_or(InvalidArgError)),
        channel_name: try!(matches.opt_str("c").ok_or(InvalidArgError)),
        username: try!(matches.opt_str("u").ok_or(InvalidArgError)),
        message: try!(matches.opt_str("m").ok_or(InvalidArgError)),
    };

    Ok(conf)
}
