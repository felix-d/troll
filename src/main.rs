extern crate getopts;

use std::process;

mod opts;


fn main() {
    let conf = match opts::parse_args() {
        Ok(conf) => conf,
        Err(e) => {
            process::exit(1);
        }
    };
    println!("{}", conf.token);
}
