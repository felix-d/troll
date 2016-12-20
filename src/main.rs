extern crate getopts;
extern crate hyper;
extern crate rustc_serialize;

mod opts;
mod slack;
mod utils;
mod errors;
mod cache;

use std::process;
use std::cell::RefCell;

fn main() {
    let conf = match opts::parse_args() {
        Ok(conf) => conf,
        Err(_) => process::exit(1),
    };

    let slack_client = slack::SlackAPIClient {
        token: &conf.token,
        use_real_name: &conf.use_real_name,
        image: &conf.image,
        cache: RefCell::new(match cache::Cache::new() {
            Ok(cache) => cache,
            Err(e) => {
                println!("{:?}", e);
                process::exit(1);
            }
        }),
    };

    let channel = slack_client.channel(&conf.channel_name);

    let user = match slack_client.user(&conf.username) {
        Ok(user) => user,
        Err(_) => {
            println!("User could not be found.");
            process::exit(1);
        },
    };

    slack_client.post_message(&user, &channel, &conf.message);
}
