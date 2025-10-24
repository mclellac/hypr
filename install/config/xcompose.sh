#!/bin/bash
#
# Creates a default .XCompose file for custom key compositions.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Creates the ~/.XCompose file with default and user-specific bindings.
# Globals:
#   HYPR_PATH (read-only)
#   hypr_USER_NAME (read-only)
#   hypr_USER_EMAIL (read-only)
#######################################
main() {
  local -r xcompose_file="${HOME}/.XCompose"

  echo "Creating default .XCompose file..."

  # The tee command writes the here-document to the specified file.
  tee "${xcompose_file}" >/dev/null <<EOF
# Include the base xcompose file from the hypr repository.
include "${HYPR_PATH}/default/xcompose"

# Custom user identification bindings, triggered with <CapsLock> <space> <key>.
<Multi_key> <space> <n> : "${hypr_USER_NAME}"
<Multi_key> <space> <e> : "${hypr_USER_EMAIL}"
EOF

  echo ".XCompose file created."
}

main "$@"
