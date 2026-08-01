#!/bin/bash
#
# Sets up the default passwordless keyring for auto-unlock.
#
set -euo pipefail

KEYRING_DIR="$HOME/.local/share/keyrings"
KEYRING_FILE="$KEYRING_DIR/Default_keyring.keyring"
DEFAULT_FILE="$KEYRING_DIR/default"

mkdir -p "$KEYRING_DIR"

if [[ ! -f "$KEYRING_FILE" ]]; then
    cat << EOF > "$KEYRING_FILE"
[keyring]
display-name=Default keyring
ctime=$(date +%s)
mtime=0
lock-on-idle=false
lock-after=false
EOF
    chmod 600 "$KEYRING_FILE"
fi

if [[ ! -f "$DEFAULT_FILE" ]]; then
    cat << EOF > "$DEFAULT_FILE"
Default_keyring
EOF
    chmod 644 "$DEFAULT_FILE"
fi

chmod 700 "$KEYRING_DIR"
