extern crate hyper;

use hyper::Url;
use hyper::client::Request;
mod util;


const SLACK_API_BASE_URL: &'static str = "https://slack.com/api/{endpoint}?token={token}"

struct User<'a> {
    username: &'a str,
    picture: &'a str
}

struct Channel<'a> {
    name: &'a str
}

pub struct SlackAPIClient<'a> {
    token: &'a str,
}

impl SlackAPIClient {
    fn request(self: &self, endpoint: &str) -> HttpResult<String> {
        util::get(format!(SLACK_API_BASE_URL, endpoint=endpoint, token=self.token))
    }

    pub fn post(self: &self, channel: Channel, user: User, message: &str) -> HttpResult<String>{
        let data = ();
        util::post(format!(SLACK_WEBHOOK_BASE_URL, token=self.token), data)
    }

    pub fn users (self: &self) -> HttpResult<String> {
        self.get("users.list")
    }

    pub fn channels (self: &self) -> HttpResult<String> {
        self.get("channels.list")
    }
}
