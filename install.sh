#!/usr/bin/env bash

set -e

BINARY_INSTALL_DIR=/usr/local/bin
BINARY_NAME=auto-restart-wifi
BINARY_INSTALL_PATH="$BINARY_INSTALL_DIR/$BINARY_NAME"

SYSTEMD_SERVICE_INSTALL_PATH=/etc/systemd/user/auto-restart-wifi.service

# Download and install binary
BINARY_DOWNLOAD_URL='https://github.com/{{GH_REPOSITORY}}/releases/download/{{GH_RELEASE_TAG}}/{{GH_RELEASE_BINARY}}'
curl --output auto-restart-wifi "$BINARY_DOWNLOAD_URL"
mv "./$BINARY_NAME" "$BINARY_INSTALL_PATH"

# Setup systemd service
BINARY_INSTALL_PATH=$BINARY_INSTALL_PATH \
envsubst << 'EOF' > $SYSTEMD_SERVICE_INSTALL_PATH
[Unit]
Description=Automatically restart USB network device when network connectivity is lost
After=network.target

[Service]
ExecStart=$BINARY_INSTALL_PATH
Type=simple

[Install]
default.targets
EOF
