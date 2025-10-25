#!/bin/bash
#
# Configures systemd to ignore the power button, allowing it to be rebound
# to a custom power menu in the window manager.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Modifies /etc/systemd/logind.conf to set HandlePowerKey=ignore.
#######################################
main() {
  local -r logind_conf="/etc/systemd/logind.conf"

  echo "Disabling default power button action..."

  # This sed command finds the line starting with '#HandlePowerKey=' or 'HandlePowerKey='
  # and replaces the entire line with 'HandlePowerKey=ignore'.
  sudo sed -i 's/.*HandlePowerKey=.*/HandlePowerKey=ignore/' "${logind_conf}"

  echo "Power button action disabled. A restart of systemd-logind may be required."
}

main "$@"
