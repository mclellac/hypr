#!/bin/bash
#
# Configures passwordless sudo access for the 'asdcontrol' utility,
# which is used to control brightness on Apple Displays.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

# --- Source Utils ---
if [[ -n "${HYPR_INSTALL:-}" ]]; then
    source "${HYPR_INSTALL}/utils.sh"
else
    source "$(dirname "$0")/../utils.sh"
fi

#######################################
# Creates a sudoers file to allow the current user to run asdcontrol without a password.
#######################################
main() {
    local -r sudoers_file="/etc/sudoers.d/asdcontrol"
    local -r asdcontrol_path="/usr/local/bin/asdcontrol"
    local -r temp_sudoers=$(mktemp)

    echo "${USER} ALL=(ALL) NOPASSWD: ${asdcontrol_path}" > "${temp_sudoers}"

    if [[ ! -f "${sudoers_file}" ]] || ! cmp -s "${temp_sudoers}" "${sudoers_file}"; then
        log_info "Configuring sudo-less access for asdcontrol..."
        sudo cp "${temp_sudoers}" "${sudoers_file}"
        sudo chmod 440 "${sudoers_file}"
    else
        log_info "asdcontrol configuration is already up to date."
    fi

    rm -f "${temp_sudoers}"
}

main "$@"
