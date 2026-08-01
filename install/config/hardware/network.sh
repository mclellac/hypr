#!/bin/bash
#
# Configures core network services, enabling iwd and disabling the
# online wait service to prevent boot delays.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Enables the iwd service for wireless networking.
#######################################
enable_iwd() {
    if ! systemctl is-enabled iwd.service >/dev/null 2>&1; then
        log_info "Enabling iwd service..."
        sudo systemctl enable iwd.service >/dev/null 2>&1 || true
    else
        log_info "iwd service is already enabled."
    fi
}

#######################################
# Disables and masks the systemd-networkd-wait-online service
# to prevent it from delaying the boot process.
#######################################
disable_wait_online() {
    local status
    status=$(systemctl is-enabled systemd-networkd-wait-online.service 2>/dev/null || echo "disabled")
    if [[ "${status}" != "masked" ]]; then
        log_info "Disabling and masking systemd-networkd-wait-online service..."
        sudo systemctl disable systemd-networkd-wait-online.service >/dev/null 2>&1 || true
        sudo systemctl mask systemd-networkd-wait-online.service >/dev/null 2>&1 || true
    else
        log_info "systemd-networkd-wait-online service is already masked."
    fi
}

#######################################
# Main function to orchestrate network service configuration.
#######################################
main() {
    enable_iwd
    disable_wait_online
    log_info "Network service configuration complete."
}

main "$@"
