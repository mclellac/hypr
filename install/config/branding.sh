#!/bin/bash
#
# Sets up the branding assets for fastfetch and the screensaver.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Copies branding files to the user's config directory.
# Globals:
#   HYPR_PATH (read-only)
#######################################
main() {
  local -r branding_dir="${HOME}/.config/hypr/branding"

  echo "Setting up branding..."
  mkdir -p "${branding_dir}"
  cp "${HYPR_PATH}/icon.txt" "${branding_dir}/about.txt"
  cp "${HYPR_PATH}/logo.txt" "${branding_dir}/screensaver.txt"
}

main "$@"
