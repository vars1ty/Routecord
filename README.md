# DNotify
An alternative Discord notification system for direct messages.

> [!CAUTION]
> This acts as a **selfbot** and requires your User Token, proceed at your own risk!

> [!WARNING]  
> **Windows**: Images for notifications are currently **not** supported!

## Why
Because alternative clients like GoofCord and alike don't properly send notifications, but can play the ping sound.

## How it works
DNotify connects to your account via the token given, then listens for direct messages and display a notification.

Note that it does not currently support servers, and is very much work-in-progress.

It also doesn't care if you are in Do not Disturb; It'll still show a notification.

## Usage
- Windows: `./dnotify.exe TOKEN_HERE HOST_USER_ID`
- Linux: `./dnotify TOKEN_HERE HOST_USER_ID`

### Fallback Methods
1. Environment Variable(s): `TOKEN`, `HOST_USER_ID`
2. File(s): `./token.txt`, `./host_user_id.txt`

## Download
See [releases](https://github.com/vars1ty/DNotify/releases).
