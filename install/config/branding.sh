#!/bin/bash

# Allow the user to change the branding for fastfetch and screensaver
mkdir -p ~/.config/hypr/branding
cp ~/.local/share/hypr/icon.txt ~/.config/hypr/branding/about.txt
cp ~/.local/share/hypr/logo.txt ~/.config/hypr/branding/screensaver.txt
