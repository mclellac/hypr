#!/bin/bash
#
# Enables the Bluetooth service.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

# Source the chroot helper function.
# The exact path might need adjustment depending on the execution context.
# Assuming it's sourced from the main install script where paths are set.
source "${HYPR_INSTALL}/preflight/chroot.sh"

#######################################
# Enables the Bluetooth systemd service.
#######################################
main() {
    log_info "Enabling Bluetooth service..."
    enable_systemctl_service "bluetooth.service"
    log_info "Bluetooth service enabled."
}

main "$@"
