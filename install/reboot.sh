#!/bin/bash
#
# Displays a completion message and reboots the system.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Displays the final message, cleans up temporary installer files, and reboots.
# Globals:
#   HOME (read-only)
#######################################
main() {
  clear

  # Display the logo and completion message with a terminal text effect.
  tte -i "${HOME}/.local/share/hypr/logo.txt" --frame-rate 920 laseretch
  echo
  echo "You're done! So we're ready to reboot now..." | tte --frame-rate 640 wipe

  # Clean up the temporary sudoers file created by the installer.
  local -r installer_sudoers_file="/etc/sudoers.d/99-hypr-installer"
  if sudo test -f "${installer_sudoers_file}"; then
    sudo rm -f "${installer_sudoers_file}" &>/dev/null
    echo -e "\nRemember to remove the USB installer!\n\n"
  fi

  # Pause before rebooting to allow the user to read the message.
  sleep 5
  reboot
}

main "$@"
