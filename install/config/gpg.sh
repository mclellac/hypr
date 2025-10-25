#!/bin/bash
#
# Configures GPG with a custom keyserver list for improved reliability.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Copies the GPG configuration and restarts the directory manager.
# Globals:
#   HYPR_PATH (read-only)
#######################################
main() {
  echo "Configuring GPG..."
  local -r gpg_config_dir="/etc/gnupg"
  local -r dirmngr_conf="${gpg_config_dir}/dirmngr.conf"

  sudo mkdir -p "${gpg_config_dir}"
  sudo cp "${HYPR_PATH}/default/gpg/dirmngr.conf" "${dirmngr_conf}"
  sudo chmod 644 "${dirmngr_conf}"

  # Restart the dirmngr to apply the new configuration.
  # The '|| true' prevents the script from exiting if the service isn't running.
  echo "Restarting GPG directory manager..."
  sudo gpgconf --kill dirmngr >/dev/null 2>&1 || true
  sudo gpgconf --launch dirmngr >/dev/null 2>&1 || true

  echo "GPG configuration complete."
}

main "$@"
