#!/usr/bin/env bash

set -e


BINARY_INSTALL_DIR=/usr/local/bin
BINARY_NAME=auto-restart-wifi
BINARY_INSTALL_PATH="$BINARY_INSTALL_DIR/$BINARY_NAME"

SYSTEMD_SERVICE_INSTALL_PATH=/etc/systemd/system/auto-restart-wifi.service


# Configure basic parameters (usb device, test address)

echo 'USB devices: '
lsusb
echo

read -rp 'Enter USB network device name: ' USB_DEVICE_TARGET </dev/tty

echo
read -rp 'Enter network test address (leave blank to use 1.1.1.1:80): ' TEST_ADDRESS </dev/tty

if [[ -z $TEST_ADDRESS ]]; then
    TEST_ADDRESS='1.1.1.1:80'
fi


# Download and install binary

BINARY_DOWNLOAD_URL='https://github.com/{{GH_REPOSITORY}}/releases/download/{{GH_RELEASE_TAG}}/{{GH_RELEASE_BINARY}}'
curl -sSL --output auto-restart-wifi "$BINARY_DOWNLOAD_URL"
chmod +x "./$BINARY_NAME"
mv "./$BINARY_NAME" "$BINARY_INSTALL_PATH"


# Setup systemd service

BINARY_INSTALL_PATH=$BINARY_INSTALL_PATH \
USB_DEVICE_TARGET=$USB_DEVICE_TARGET \
TEST_ADDRESS=$TEST_ADDRESS \
envsubst << 'EOF' > $SYSTEMD_SERVICE_INSTALL_PATH
[Unit]
Description=Automatically restart USB network device when network connectivity is lost
After=network-online.target

[Service]
ExecStart=$BINARY_INSTALL_PATH --test-address $TEST_ADDRESS --usb-device-target "$USB_DEVICE_TARGET"
Type=simple

[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload
systemctl restart auto-restart-wifi