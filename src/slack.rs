use rustc_serialize::json;
use std::collections::BTreeMap;
use std::cell::RefCell;
use utils;
use cache;
use errors::Error;


const SLACK_API_BASE_URL: &'static str = "https://slack.com/api/{endpoint}?token={token}";


pub struct SlackAPIClient<'a> {
    pub token: &'a str,
    pub cache: RefCell<cache::Cache>,
}

pub struct User {
    pub id: String,
    pub username: String,
    pub picture_url: String,
}

pub struct Channel {
    id: String,
}

impl<'a> SlackAPIClient<'a> {
    fn url(&self, endpoint: &str) -> String {
        format!("https://slack.com/api/{endpoint}?token={token}&pretty=1", endpoint=endpoint, token=self.token)
    }

    fn get(&self, endpoint: &str) -> Result<BTreeMap<String, json::Json>, Error> {
        utils::get(&self.url(endpoint))
    }

    pub fn post_message(&self, user: &User, channel: &Channel, message: &str) -> Result<(), Error> {
        let payload = format!("token={token}&channel={channel}&text={message}&username={username}&icon_url={img}",
          token=self.token,
          channel=channel.id,
          message=message,
          username=user.username,
          img=user.picture_url,
        );
        try!(utils::post("https://slack.com/api/chat.postMessage", &payload));
        Ok(())
    }

    fn users(&self) -> Result<json::Json, Error> {
        let result = try!(self.get("users.list"));
        let users = try!(result.get("members").ok_or(Error::NoMembers));

        Ok(users.clone())
    }

    fn channels(&self) -> Result<json::Json, Error> {
        let result = try!(self.get("channels.list"));
        let channels = try!(result.get("channels").ok_or(Error::NoChannels));

        Ok(channels.clone())
    }

    fn find_user(username: &str, users: &json::Json) -> Result<Option<User>, Error> {
        let users = try!(users.as_array().ok_or(Error::UnexpectedJson));

        for user in users {
            let name_json = try!(user.find("name").ok_or(Error::UnexpectedJson));
            let name_from_json = try!(name_json.as_string().ok_or(Error::UnexpectedJson));

            if username == name_from_json {
                let user = try!(Self::build_user(&user, username));
                return Ok(Some(user));
            } else {
                continue;
            }
        }
        Ok(None)
    }

    fn build_user(user: &json::Json, username: &str) -> Result<User, Error> {
        let picture_url_json = try!(user.find_path(&["profile", "image_72"]).ok_or(Error::UnexpectedJson));
        let picture_url = try!(picture_url_json.as_string().ok_or(Error::UnexpectedJson));

        let id_json = try!(user.find("id").ok_or(Error::UnexpectedJson));
        let id = try!(picture_url_json.as_string().ok_or(Error::UnexpectedJson));

        let realname_json = try!(user.find("real_name").ok_or(Error::UnexpectedJson));
        let realname = try!(realname_json.as_string().ok_or(Error::UnexpectedJson));

        Ok(User {
            id: id.to_string(),
            username: realname.to_string(),
            picture_url: picture_url.to_string(),
        })
    }

    fn build_channel(channel: &json::Json) -> Result<Channel, Error> {
        let id_json = try!(channel.find("id").ok_or(Error::UnexpectedJson));
        let id_from_json = try!(id_json.as_string().ok_or(Error::UnexpectedJson));

        Ok(Channel {
            id: id_from_json.to_string()
        })
    }

    fn find_channel(channel_name: &str, channels: &json::Json) -> Result<Option<Channel>, Error> {
        let channels = try!(channels.as_array().ok_or(Error::UnexpectedJson));

        for channel in channels {
            let name_json = try!(channel.find("name").ok_or(Error::UnexpectedJson));
            let name_from_json = try!(name_json.as_string().ok_or(Error::UnexpectedJson));

            if name_from_json == channel_name {
                let channel = try!(Self::build_channel(&channel));
                return Ok(Some(channel));
            }
        }
        Ok(None)
    }

    fn resolve_users(&self) -> Result<json::Json, Error> {
        let mut cache = self.cache.borrow_mut();
        match cache.get("users") {
            Some(users) => Ok(users),
            None => {
                let users = try!(self.users());
                cache.set("users", &users);
                Ok(users)
            },
        }
    }

    pub fn user(&self, username: &str) -> Result<User, Error> {
        let users = try!(self.resolve_users());

        match try!(SlackAPIClient::find_user(username, &users)) {
            Some(user) => Ok(user),
            None => {
                let users = try!(self.users());
                self.cache.borrow_mut().set("users", &users);
                match try!(SlackAPIClient::find_user(username, &users)) {
                    Some(user) => Ok(user),
                    None => Err(Error::UserNotFound)
                }
            },
        }
    }

    fn resolve_channels(&self) -> Result<json::Json, Error>{
        let mut cache = self.cache.borrow_mut();
        match cache.get("channels") {
            Some(channels) => Ok(channels),
            None => {
                let channels = try!(self.channels());
                cache.set("channels", &channels);
                Ok(channels)
            },
        }
    }

    pub fn channel(&self, channel_name: &str) -> Result<Channel, Error> {
        let channels = try!(self.resolve_channels());

        match try!(SlackAPIClient::find_channel(channel_name, &channels)) {
            Some(channel) => Ok(channel),
            None => {
                let channels = try!(self.channels());
                self.cache.borrow_mut().set("channels", &channels);
                match try!(SlackAPIClient::find_channel(channel_name, &channels)) {
                    Some(channel) => Ok(channel),
                    None => Err(Error::ChannelNotFound),
                }
            },
        }
    }
}
