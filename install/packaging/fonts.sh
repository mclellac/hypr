#!/bin/bash

# hypr logo in a font for Waybar use
mkdir -p ~/.local/share/fonts
cp $HYPR_PATH/config/hypr.ttf ~/.local/share/fonts/
fc-cache
