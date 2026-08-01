#!/bin/bash
#
# Creates user override files if they don't exist.
#
# These files are intended for user-specific configurations that should not be
# overwritten by future updates or theme changes.

set -euo pipefail

if [[ -n "${HYPR_INSTALL:-}" ]]; then
    source "${HYPR_INSTALL}/utils.sh"
else
    source "$(dirname "$0")/../utils.sh"
fi

# Ensure ~/.config/hypr/monitors.lua exists
MONITORS_LUA="${HOME}/.config/hypr/monitors.lua"
if [[ ! -f "${MONITORS_LUA}" ]]; then
    log_info "Creating default monitor configuration..."
    mkdir -p "$(dirname "${MONITORS_LUA}")"
    cat > "${MONITORS_LUA}" <<EOF
-- See https://wiki.hyprland.org/Configuring/Monitors/
-- Generated and managed by Hyprland Preferences Application

hl.monitor({
    output = "",
    mode = "preferred",
    position = "auto",
    scale = 1
})
EOF
fi

log_info "User configuration check complete."
