use std::env;
use getopts::{Options};
use errors::Error;


const PROGRAM: &'static str = "troll";

pub struct Config {
    pub token: String,
    pub channel_name: String,
    pub username: String,
    pub message: String,
    pub image: String,
    pub use_real_name: bool,
}

fn print_usage(opts: Options) {
    let brief = format!("Usage: ./{} [options]", PROGRAM);
    print!("{}", opts.usage(&brief));
}

pub fn parse_args() -> Result<Config, Error> {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();

    opts.reqopt("t", "token", "You must provide the access token.", "TOKEN");
    opts.reqopt("c", "channel", "You must provide the channel name.", "CHANNEL");
    opts.reqopt("u", "username", "You must provide the username.", "USERNAME");
    opts.reqopt("m", "message", "You must provide the message you want to ouput.", "MESSAGE");
    opts.optflag("r", "real-name", "Use real_name instead of username if user exists.");
    opts.optopt("f", "fake-user-image", "Use a fake user and an arbitrary image", "IMAGE HREF");
    opts.optflag("h", "help", "Print this help menu.");

    let matches = try!(opts.parse(&args[1..]).or(Err(Error::InvalidArgError)));

    if matches.opt_present("h") {
        print_usage(opts);
        return Err(Error::HelpMenuRequested);
    }

    let conf = Config {
        token: try!(matches.opt_str("t").ok_or(Error::InvalidArgError)),
        channel_name: try!(matches.opt_str("c").ok_or(Error::InvalidArgError)),
        username: try!(matches.opt_str("u").ok_or(Error::InvalidArgError)),
        message: try!(matches.opt_str("m").ok_or(Error::InvalidArgError)),
        image: matches.opt_str("f").unwrap_or("".to_string()),
        use_real_name: matches.opt_present("r"),
    };

    Ok(conf)
}
