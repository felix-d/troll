# Troll - Troll your co-workers in Slack

This script let you post as a bot in Slack under the identity of any user in a
given channel. It fetches and caches the list of users, get the user id corresponding to
the provided username along with its profile picture, and post in the specified
channel the provided message.

## Build

```
cargo build
```

## Run

### Options

```
-t TOKEN Slack access token
-c CHANNEL_NAME channel name
-u USERNAME username
-m MESSAGE message
-f IMAGE_HREF in case you dont want to impersonate an existing user, you can provide an arbitrary image href. Note that -r won't have any effect in that case.
-r use the user real name instead of its username.
```

Example

```sh
./troll -t xoxp-3456365645645-34535345-45345345-9088098vcvsdgasdd980dgf980 -c general -u simon -m 'FOOBAR' -r
```
