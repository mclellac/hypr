#!/bin/bash
#
# Disables USB autosuspend to prevent issues with peripherals disconnecting
# unexpectedly.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Creates a modprobe configuration file to disable USB autosuspend.
#######################################
main() {
    local -r config_file="/etc/modprobe.d/disable-usb-autosuspend.conf"
    local -r setting="options usbcore autosuspend=-1"

    if [[ ! -f "${config_file}" ]]; then
        log_info "Disabling USB autosuspend..."
        echo "${setting}" | sudo tee "${config_file}" >/dev/null
        log_info "USB autosuspend disabled. A reboot is required for the changes to take effect."
    else
        log_info "USB autosuspend configuration already exists. Skipping."
    fi
}

main "$@"
