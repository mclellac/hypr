#!/bin/bash

# hypr logo in a font for Waybar use
mkdir -p ~/.local/share/fonts
cp ~/.local/share/hypr/config/hypr.ttf ~/.local/share/fonts/
fc-cache
