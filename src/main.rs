extern crate getopts;
extern crate hyper;
extern crate rustc_serialize;

mod opts;
mod slack;
mod utils;
mod errors;
mod cache;

use std::process;

fn main() {
    let conf = match opts::parse_args() {
        Ok(conf) => conf,
        Err(e) => {
            process::exit(1);
        }
    };

    let slack_client = slack::SlackAPIClient {
        token: &conf.token,
    };

    let user = slack_client.user(&conf.username);
    let channel = slack_client.channel(&conf.channel_name);

    println!("{}", conf.token);
}
