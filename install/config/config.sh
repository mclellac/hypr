#!/bin/bash

echo "Installing hypr configurations into ~/.config..."
mkdir -p ~/.config
cp -R "$HYPR_PATH"/config/* "${HOME}/.config/"

echo "Copying themes and ensuring backgrounds directory exists..."
# Ensure the main themes directory exists
mkdir -p "${HOME}/.config/hypr/themes"
# Copy each theme and create a backgrounds/ subdirectory if it's missing.
for theme_dir in "$HYPR_PATH"/themes/*; do
  if [[ -d "$theme_dir" ]]; then
    # Copy the full theme directory to the destination
    cp -R "$theme_dir" "${HOME}/.config/hypr/themes/"
    # Ensure the backgrounds subdirectory exists in the new location
    theme_name=$(basename "$theme_dir")
    mkdir -p "${HOME}/.config/hypr/themes/$theme_name/backgrounds"
  fi
done

echo "Creating current theme directory..."
mkdir -p ~/.config/hypr/current

echo "Installing .bashrc..."
cp -R "$HYPR_PATH/default/bashrc" "${HOME}/.bashrc"

echo "Configuration installation complete."
