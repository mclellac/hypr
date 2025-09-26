#!/bin/bash

# Allow the user to change the branding for fastfetch and screensaver
mkdir -p ~/.config/hypr/branding
cp $HYPR_PATH/icon.txt ~/.config/hypr/branding/about.txt
cp $HYPR_PATH/logo.txt ~/.config/hypr/branding/screensaver.txt
