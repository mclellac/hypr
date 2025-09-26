#!/bin/bash

# Copy over hypr configs
mkdir -p ~/.config
cp -R $HYPR_PATH/config/* ~/.config/

# Use default bashrc from hypr
cp $HYPR_PATH/default/bashrc ~/.bashrc
