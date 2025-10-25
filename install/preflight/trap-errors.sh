#!/bin/bash
#
# Sets a trap to catch errors, display a message, and offer a retry.

# The -E flag ensures that the ERR trap is inherited by functions,
# command substitutions, and subshells.
set -eEuo pipefail

#######################################
# Error handler function for the ERR trap.
# Displays an error message, a QR code for help, and prompts the user
# to retry the installation.
# Globals:
#   BASH_COMMAND (read-only)
#######################################
error_handler() {
  local -r exit_code="$?"
  echo -e "\n\e[31mhypr installation failed!\e[0m"
  echo
  echo "This command halted with exit code ${exit_code}:"
  echo "${BASH_COMMAND}"
  echo
  echo "Get help from the community via QR code or at https://discord.gg/tXFUdasqhY"
  echo "                                 "
  echo "    █▀▀▀▀▀█ ▄ ▄ ▀▄▄▄█ █▀▀▀▀▀█    "
  echo "    █ ███ █ ▄▄▄▄▀▄▀▄▀ █ ███ █    "
  echo "    █ ▀▀▀ █ ▄█  ▄█▄▄▀ █ ▀▀▀ █    "
  echo "    ▀▀▀▀▀▀▀ ▀▄█ █ █ █ ▀▀▀▀▀▀▀    "
  echo "    ▀▀█▀▀▄▀▀▀▀▄█▀▀█  ▀ █ ▀ █     "
  echo "    █▄█ ▄▄▀▄▄ ▀ ▄ ▀█▄▄▄▄ ▀ ▀█    "
  echo "    ▄ ▄▀█ ▀▄▀▀▀▄ ▄█▀▄█▀▄▀▄▀█▀    "
  echo "    █ ▄▄█▄▀▄█ ▄▄▄  ▀ ▄▀██▀ ▀█    "
  echo "    ▀ ▀   ▀ █ ▀▄  ▀▀█▀▀▀█▄▀      "
  echo "    █▀▀▀▀▀█ ▀█  ▄▀▀ █ ▀ █▄▀██    "
  echo "    █ ███ █ █▀▄▄▀ █▀███▀█▄██▄    "
  echo "    █ ▀▀▀ █ ██  ▀ █▄█ ▄▄▄█▀ █    "
  echo "    ▀▀▀▀▀▀▀ ▀ ▀ ▀▀▀  ▀ ▀▀▀▀▀▀    "
  echo "                                 "

  if command -v gum >/dev/null && gum confirm "Retry installation?"; then
    bash "${HOME}/.local/share/hypr/install.sh"
  else
    echo "You can retry later by running: bash ${HOME}/.local/share/hypr/install.sh"
  fi
}

trap error_handler ERR
