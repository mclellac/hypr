#!/bin/bash

# Setup GPG configuration with multiple keyservers for better reliability
sudo mkdir -p /etc/gnupg
sudo cp $HYPR_PATH/default/gpg/dirmngr.conf /etc/gnupg/
sudo chmod 644 /etc/gnupg/dirmngr.conf
sudo gpgconf --kill dirmngr >/dev/null 2>&1 || true
sudo gpgconf --launch dirmngr >/dev/null 2>&1 || true
