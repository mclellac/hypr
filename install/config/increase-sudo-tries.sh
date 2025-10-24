#!/bin/bash
#
# Increases the number of sudo password attempts to 10.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Creates a sudoers file to override the default number of password attempts.
#######################################
main() {
  local -r sudoers_file="/etc/sudoers.d/passwd-tries"
  local -r tries="10"

  echo "Increasing sudo password attempts to ${tries}..."

  echo "Defaults passwd_tries=${tries}" | sudo tee "${sudoers_file}" >/dev/null

  # Set restrictive permissions for the sudoers file.
  sudo chmod 440 "${sudoers_file}"

  echo "Sudo password attempts configuration updated."
}

main "$@"
