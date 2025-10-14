#!/bin/bash

echo "Installing hypr configurations into ~/.config..."
mkdir -p ~/.config
cp -R "$HYPR_PATH"/config/* "${HOME}/.config/"

echo "Copying themes..."
mkdir -p ~/.config/hypr/themes
cp -R "$HYPR_PATH"/themes/* "${HOME}/.config/hypr/themes/"

echo "Creating theme and background directories..."
mkdir -p ~/.config/hypr/current/theme
mkdir -p ~/.config/hypr/current/backgrounds

echo "Installing .bashrc..."
cp -R "$HYPR_PATH/default/bashrc" "${HOME}/.bashrc"

echo "Configuration installation complete."
