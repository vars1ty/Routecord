# Routecord
Routecord reimplement/reroute base Discord features to work better.

> [!CAUTION]
> This acts as a **selfbot** if you activated `enable_notifications` and requires your User Token, proceed at your own risk!

> [!WARNING]  
> **Windows**: Images for notifications are currently **not** supported!

## Why
Discord on Linux is functioning with help of band-aid as-is, and while using alternative clients like GoofCord helps, notifications can fail to deliver.

There's also issues like RPC not working for games, and while this **does not fix the applications own implementation**,

it does allow you to create custom RPC statuses yourself.

## How it works
Routecord connects to your account via your User Token, specified in `secrets.json`.

For RPC it doesn't require a token and works per Discord's own documentation.

Note that notifications are very basic if you enable them, and are still work-in-progress.

It also doesn't care if you are in Do not Disturb; It'll still show a notification.

## Installation
### Windows
1. Navigate to `C:\Users\YOUR_USERNAME\`
2. Create a new directory named `routecord`
3. Open the directory and create 2 files: `secrets.json` & `config.json`
4. Open each file and copy the content from the examples into each respective file, then modify the content as needed.

### Linux
1. Navigate to `~/.config/`
2. Create a new directory named `routecord`
3. Open the directory and create 2 files: `secrets.json` & `config.json`
4. Open each file and copy the content from the examples into each respective file, then modify the content as needed.

## Configuration
### Config (`config.json`)
The config has several keys, here's them all and a short summary:
- `enable_rpc` [true/false]: Enable RPC support, allowing you to set custom statuses.
- `enable_notifications` [true/false]: Enable notifications support, **connects via your token**.
- `rpc` [Object Array]: Array of processes to be looked for, and their status-configuration. Below is a tree-view of the layout:
    - `Process Name` [Object]: Process name string to be found.
        - `name` [String]: Display name for this status, like `Playing (name)`
        - `state` [String]: State for this status, like `In a Match`
        - `details` [String]: Details for this status, like `Hello, I'm using (name)!`
        - `type` [0/5]: The [activity type](https://discord.com/developers/docs/events/gateway-events#activity-object-activity-types) to be used.
        - `assets` [String Array]: Assets to be used for this status. Below is a tree-view of the layout:
            - `large_image`: [String]: URL for the large image that should be used for this status.
            - `large_text`: [String]: Short summary that should be shown when hovering over the large image.
            - `small_image`: [String]: URL for the small image that should be used for this status.
            - `small_text`: [String]: Short summary that should be shown when hovering over the small image.

RPC example for notepad.exe:
```json
"rpc": {
    "notepad.exe": {
        "name": "Notepad",
        "details": "I'm using Notepad!",
        "type": 0
    }
}
```

## Secrets (`secrets.json`)
The secrets file only has 2 keys, here's them all and a short summary:
- `token` [String]: Your Discord Account Token.
- `user_id` [Number]: Your Discord Account ID.

## Download
See [releases](https://github.com/vars1ty/DNotify/releases).
