#!/bin/bash
#
# Provides a helper function to enable systemd services,
# handling the case where the installer is running in a chroot environment.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Enables a systemd service, handling chroot environments.
# If in chroot, it enables the service without starting it.
# Otherwise, it enables and starts the service immediately.
# Arguments:
#   The name of the systemd service to enable.
# Globals:
#   HYPR_CHROOT_INSTALL (read-only)
#######################################
enable_systemctl_service() {
  local service_name="$1"
  if [[ -n "${HYPR_CHROOT_INSTALL:-}" ]]; then
    sudo systemctl enable "${service_name}"
  else
    sudo systemctl enable --now "${service_name}"
  fi
}
