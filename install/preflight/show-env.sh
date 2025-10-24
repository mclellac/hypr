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
  env | grep -E "^(hypr_CHROOT_INSTALL|hypr_USER_NAME|hypr_USER_EMAIL|USER|HOME|hypr_REPO|hypr_REF)="
}

main "$@"
