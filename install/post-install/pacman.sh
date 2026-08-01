#!/bin/bash
#
# Configures pacman settings and mirrorlists.
#
set -euo pipefail

# Configure pacman
sudo cp -f ~/.local/share/hypr/default/pacman/pacman-"${HYPR_MIRROR:-stable}".conf /etc/pacman.conf
sudo cp -f ~/.local/share/hypr/default/pacman/mirrorlist-"${HYPR_MIRROR:-stable}" /etc/pacman.d/mirrorlist

if lspci -nn | grep -q "106b:180[12]"; then
  if ! grep -q "\[arch-mact2\]" /etc/pacman.conf; then
    cat <<EOF | sudo tee -a /etc/pacman.conf >/dev/null

[arch-mact2]
Server = https://github.com/NoaHimesaka1873/arch-mact2-mirror/releases/download/release
SigLevel = Never
EOF
  fi
fi
