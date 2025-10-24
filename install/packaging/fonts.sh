#!/bin/bash
#
# Installs the custom 'hypr' font for Waybar.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Copies the hypr.ttf font to the user's font directory and rebuilds the font cache.
# Globals:
#   HYPR_PATH (read-only)
#######################################
main() {
  local -r font_dir="${HOME}/.local/share/fonts"

  mkdir -p "${font_dir}"
  cp "${HYPR_PATH}/config/hypr.ttf" "${font_dir}/"
  fc-cache
}

main "$@"
