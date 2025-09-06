#!/bin/bash

# Copy over hypr configs
mkdir -p ~/.config
cp -R ~/.local/share/hypr/config/* ~/.config/

# Use default bashrc from hypr
cp ~/.local/share/hypr/default/bashrc ~/.bashrc
