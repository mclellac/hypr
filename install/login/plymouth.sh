#!/bin/bash
#
# Configures the default Plymouth boot splash theme.
#
set -euo pipefail

if command -v plymouth-set-default-theme &>/dev/null; then
  if [[ $(plymouth-set-default-theme) != "hypr" ]]; then
    sudo mkdir -p /usr/share/plymouth/themes/hypr/
    sudo cp -rf "$HOME/.local/share/hypr/default/plymouth/." /usr/share/plymouth/themes/hypr/
    sudo plymouth-set-default-theme -R hypr || sudo plymouth-set-default-theme hypr
  fi
fi
