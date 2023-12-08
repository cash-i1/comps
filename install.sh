#!/bin/bash

if [[ "$OSTYPE" == "linux"* ]]; then
    BINARY_NAME="comps"
    RELEASE_URL="https://api.github.com/repos/cash-i1/comps/releases/latest"

    INSTALL_DIR="$HOME/bin"

    release_info=$(curl -sSL ${RELEASE_URL})

    download_url=$(echo "$release_info" | grep -oP '"browser_download_url":\s*"\K[^"]*(?="_linux")')

    mkdir -p "$INSTALL_DIR"

    if [[ $download_url ]]; then
        echo "Downloading $BINARY_NAME..."
        curl -sSL "$download_url" -o "$INSTALL_DIR/$BINARY_NAME"

        chmod +x "$INSTALL_DIR/$BINARY_NAME"

        echo "$BINARY_NAME has been installed to $INSTALL_DIR"
    else
        echo "Could not install comps"
        exit 1
    fi

else
    echo "Your device does not support comps yet."
    exit 1
fi





