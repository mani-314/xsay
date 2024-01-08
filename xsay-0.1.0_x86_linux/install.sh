#!/bin/bash

# Define paths
ASCIIART_SOURCE="./asciiart"
XSAY_SOURCE="./xsay"
CONFIG_DIR="$HOME/.config/xsay/"
BIN_DIR="$HOME/bin"

# Create xsay config directory if it doesn't exist
mkdir -p "$CONFIG_DIR"

# Copy asciiart to ~/.config/xsay/asciiart
cp -r "$ASCIIART_SOURCE" "$CONFIG_DIR"

# Create ~/bin if it doesn't exist
mkdir -p "$BIN_DIR"

# Copy xsay to ~/bin
cp "$XSAY_SOURCE" "$BIN_DIR"

echo "Installation completed successfully."
