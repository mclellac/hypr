#!/bin/bash
#
# Configures passwordless sudo access for the 'asdcontrol' utility,
# which is used to control brightness on Apple Displays.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Creates a sudoers file to allow the current user to run asdcontrol without a password.
#######################################
main() {
  local -r sudoers_file="/etc/sudoers.d/asdcontrol"
  local -r asdcontrol_path="/usr/local/bin/asdcontrol"

  echo "Configuring sudo-less access for asdcontrol..."

  echo "${USER} ALL=(ALL) NOPASSWD: ${asdcontrol_path}" | sudo tee "${sudoers_file}" >/dev/null

  # Set restrictive permissions for the sudoers file.
  sudo chmod 440 "${sudoers_file}"

  echo "asdcontrol configuration complete."
}

main "$@"
