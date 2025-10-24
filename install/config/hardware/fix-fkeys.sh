#!/bin/bash
#
# Configures Apple-like keyboards to have function keys (F1-F12) enabled by default,
# rather than media keys.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Creates a modprobe configuration file to set the 'fnmode' for Apple keyboards.
#######################################
main() {
  local -r config_file="/etc/modprobe.d/hid_apple.conf"
  local -r setting="options hid_apple fnmode=2"

  if [[ ! -f "${config_file}" ]]; then
    echo "Applying F-key fix for Apple-like keyboards..."
    echo "${setting}" | sudo tee "${config_file}" >/dev/null
    echo "F-key fix applied. A reboot is required for the changes to take effect."
  else
    echo "F-key configuration already exists. Skipping."
  fi
}

main "$@"
