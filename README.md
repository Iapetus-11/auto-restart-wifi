# Auto-Restart-Wifi
- Only works with USB network devices (doesn't matter if it's WiFi or not)
- Only tested with Debian 13
- Created for my home server, because my WiFi is extraordinarily shitty and my apartment does not offer ethernet

## Installation Instructions
Use the install script to download the binary and setup a systemd service automatically:
```
curl https://github.com/Iapetus-11/auto-restart-wifi/releases/latest/download/install.sh | bash
```

You can edit the flags in (`/etc/systemd/user/auto-restart-wifi.service`, see `ExecStart`) to your liking. Run `auto-restart-wifi --help` to see available flags first.

Or, download the latest binary from [releases](https://github.com/Iapetus-11/auto-restart-wifi/releases) and run it however you like.