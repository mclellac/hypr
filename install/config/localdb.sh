#!/bin/bash
#
# Updates the mlocate database.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Runs updatedb to index the filesystem for the 'locate' command.
#######################################
main() {
  echo "Updating the mlocate database..."
  sudo updatedb
  echo "mlocate database update complete."
}

main "$@"
