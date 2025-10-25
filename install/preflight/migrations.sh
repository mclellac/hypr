#!/bin/bash
#
# Creates migration state files.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Creates migration state files to prevent them from running on first install.
# Globals:
#   None
# Arguments:
#   None
#######################################
main() {
  local hypr_migrations_state_path="${HOME}/.local/state/hypr/migrations"
  mkdir -p "${hypr_migrations_state_path}"

  local file
  for file in "${HOME}/.local/share/hypr/migrations/"*.sh; do
    touch "${hypr_migrations_state_path}/$(basename "${file}")"
  done
}

main "$@"
