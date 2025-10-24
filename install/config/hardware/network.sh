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
  echo "Enabling iwd service..."
  sudo systemctl enable iwd.service
}

#######################################
# Disables and masks the systemd-networkd-wait-online service
# to prevent it from delaying the boot process.
#######################################
disable_wait_online() {
  echo "Disabling systemd-networkd-wait-online service..."
  sudo systemctl disable systemd-networkd-wait-online.service
  sudo systemctl mask systemd-networkd-wait-online.service
}

#######################################
# Main function to orchestrate network service configuration.
#######################################
main() {
  enable_iwd
  disable_wait_online
  echo "Network service configuration complete."
}

main "$@"
