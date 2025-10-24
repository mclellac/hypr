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
    echo "Disabling USB autosuspend..."
    echo "${setting}" | sudo tee "${config_file}" >/dev/null
    echo "USB autosuspend disabled. A reboot is required for the changes to take effect."
  else
    echo "USB autosuspend configuration already exists. Skipping."
  fi
}

main "$@"
