# drink-water

An hourly macOS reminder to drink water, delivered as a Telegram message.

A launchd agent runs `drink-water` every hour; the script sends a message through a Telegram bot, except between 8 p.m. and 9 a.m. in the system's local time zone.

## Setup

1. Create a bot with [@BotFather](https://t.me/BotFather) (`/newbot`) and copy its token.
2. Send your bot any message, then find your chat ID:
   ```sh
   curl -s "https://api.telegram.org/bot<TOKEN>/getUpdates" | grep -o '"chat":{"id":[0-9-]*'
   ```
3. Install the config, script and launchd agent:
   ```sh
   mkdir -p ~/.config/drink-water ~/.local/bin
   cp env.example ~/.config/drink-water/env    # fill in TELEGRAM_BOT_TOKEN and TELEGRAM_CHAT_ID
   chmod 600 ~/.config/drink-water/env
   cp drink-water ~/.local/bin/
   cp com.u9g.drink-water.plist ~/Library/LaunchAgents/
   launchctl bootstrap gui/$(id -u) ~/Library/LaunchAgents/com.u9g.drink-water.plist
   ```
   The plist points at `/Users/jason.lernerman/.local/bin/drink-water`; change that path for your user.

## Test

```sh
launchctl kickstart -k gui/$(id -u)/com.u9g.drink-water
```
