#!/bin/bash

# Set default XCompose that is triggered with CapsLock
tee ~/.XCompose >/dev/null <<EOF
include "$HYPR_PATH/default/xcompose"

# Identification
<Multi_key> <space> <n> : "$hypr_USER_NAME"
<Multi_key> <space> <e> : "$hypr_USER_EMAIL"
EOF
