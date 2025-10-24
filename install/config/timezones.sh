#!/bin/bash
#
# Configures passwordless sudo access for timezone-related commands.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Creates a sudoers file to allow members of the 'wheel' group to update
# the timezone without a password.
#######################################
main() {
  local -r sudoers_file="/etc/sudoers.d/hypr-tzupdate"

  echo "Configuring sudo-less access for timezone updates..."

  # The tee command is used to write to a file with sudo privileges.
  sudo tee "${sudoers_file}" >/dev/null <<EOF
# Allow users in the wheel group to run tzupdate and timedatectl without a password.
%wheel ALL=(root) NOPASSWD: /usr/bin/tzupdate, /usr/bin/timedatectl
EOF

  # Set restrictive permissions for the sudoers file.
  sudo chmod 0440 "${sudoers_file}"

  echo "Timezone update configuration complete."
}

main "$@"
