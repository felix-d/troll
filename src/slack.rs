use rustc_serialize::json;
use std::collections::BTreeMap;
use utils;
use cache;
use errors::Error;


const SLACK_API_BASE_URL: &'static str = "https://slack.com/api/{endpoint}?token={token}";


pub struct SlackAPIClient<'a> {
    pub token: &'a str,
}

pub struct User<'a> {
    pub username: &'a str,
    pub picture_url: &'a str,
}

pub struct Channel<'a> {
    channel_name: &'a str,
}

impl<'a> SlackAPIClient<'a> {
    fn url(&self, endpoint: &str) -> String {
        format!("https://slack.com/api/{endpoint}?token={token}&pretty=1", endpoint=endpoint, token=self.token)
    }

    fn get(&self, endpoint: &str) -> Result<BTreeMap<String, json::Json>, Error> {
        utils::get(&self.url(endpoint))
    }

    fn users(&self) -> Result<json::Json, Error> {
        let result = try!(self.get("users.list"));
        let users = try!(result.get("members").ok_or(Error::NoMembers));
        cache::set("users", users);
        Ok(users.clone())
    }

    fn channels(&self) -> Result<json::Json, Error> {
        let result = try!(self.get("channels.list"));
        let channels = try!(result.get("channels").ok_or(Error::NoChannels));
        Ok(channels.clone())
    }

    fn find_user<'b>(username: &str, users: &json::Json) -> Result<Option<User<'b>>, Error> {
        let user = User {
            username: "foo",
            picture_url: "foo",
        };
        Ok(Some(user))
    }

    fn find_channel<'b>(channel_name: &str, channels: &json::Json) -> Result<Option<Channel<'b>>, Error> {
        let channel = Channel {
            channel_name: "foo",
        };
        Ok(Some(channel))
    }

    pub fn user(&self, username: &str) -> Result<User, Error> {
        let users_from_cache = try!(cache::get("users"));
        let users = match users_from_cache {
            Some(users) => users,
            None => try!(self.users()),
        };
        let user = try!(SlackAPIClient::find_user(username, &users));
        match user {
            Some(user) => Ok(user),
            None => Err(Error::UserNotFound),
        }
    }

    pub fn channel(&self, channel_name: &str) -> Result<Channel, Error> {
        let channels_from_cache = try!(cache::get("channels"));
        let channels = match channels_from_cache {
            Some(channels) => channels,
            None => try!(self.channels()),
        };
        let channel = try!(SlackAPIClient::find_channel(channel_name, &channels));
        match channel {
            Some(channel) => Ok(channel),
            None => Err(Error::ChannelNotFound),
        }
    }
}
