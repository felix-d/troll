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

```sh
./troll -t SLACK_ACCESS_TOKEN -c CHANNEL_NAME -u USERNAME -m MESSAGE
```
