use std::collections::BTreeMap;
use std::cell::RefCell;

use rustc_serialize::json;

use errors::Error;
use utils;
use cache;


pub struct SlackAPIClient<'a> {
    pub token: &'a str,
    pub image: &'a str,
    pub cache: RefCell<cache::Cache>,
    pub use_real_name: &'a bool,
}

pub struct User {
    pub username: String,
    pub picture_url: String,
}

pub struct Channel {
    channel_name: String,
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
          channel=channel.channel_name,
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

    fn find_user(&self, username: &str, users: &json::Json) -> Result<Option<User>, Error> {
        let users = try!(users.as_array().ok_or(Error::UnexpectedJson));

        for user in users {
            let name_json = try!(user.find("name").ok_or(Error::UnexpectedJson));
            let name_from_json = try!(name_json.as_string().ok_or(Error::UnexpectedJson));

            if username == name_from_json {
                let user = try!(self.build_user_from_json(&user, username));
                return Ok(Some(user));
            } else {
                continue;
            }
        }
        Ok(None)
    }

    fn build_user_from_json(&self, user: &json::Json, username: &str) -> Result<User, Error> {
        let picture_url_json = try!(user.find_path(&["profile", "image_72"]).ok_or(Error::UnexpectedJson));
        let picture_url = try!(picture_url_json.as_string().ok_or(Error::UnexpectedJson));

        let name = match *self.use_real_name {
            true => {
                let real_name_json = try!(user.find("real_name").ok_or(Error::UnexpectedJson));
                try!(real_name_json.as_string().ok_or(Error::UnexpectedJson)).to_string()
            },
            false => username.to_string(),
        };

        Ok(User {
            username: name,
            picture_url: picture_url.to_string(),
        })
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

    fn existing_user(&self, username: &str) -> Result<User, Error> {
        let users = try!(self.resolve_users());

        match try!(self.find_user(username, &users)) {
            Some(user) => Ok(user),
            None => {
                let users = try!(self.users());
                self.cache.borrow_mut().set("users", &users);
                match try!(self.find_user(username, &users)) {
                    Some(user) => Ok(user),
                    None => Err(Error::UserNotFound),
                }
            }
        }
    }

    fn fake_user(&self, username: &str) -> User {
        User {
            username: username.to_string(),
            picture_url: self.image.to_string(),
        }
    }

    pub fn user(&self, username: &str) -> Result<User, Error> {
        if self.image.is_empty() {
            self.existing_user(username)
        } else {
            Ok(self.fake_user(username))
        }
    }

    pub fn channel(&self, channel_name: &str) -> Channel {
        Channel {
            channel_name: channel_name.to_string(),
        }
    }
}
