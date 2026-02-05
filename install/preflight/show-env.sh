#!/bin/bash
#
# Displays relevant environment variables for the installation.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Prints key environment variables to the console.
#######################################
main() {
  echo "Installation ENV:"
  env | grep -E "^(HYPR_CHROOT_INSTALL|HYPR_USER_NAME|HYPR_USER_EMAIL|USER|HOME|HYPR_REPO|HYPR_REF)="
}

main "$@"
