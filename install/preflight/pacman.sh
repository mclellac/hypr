#!/bin/bash
#
# Sets up pacman configuration and refreshes repositories.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Main function
#######################################
main() {
  # Install build tools
  sudo pacman -S --needed --noconfirm base-devel

  # Configure pacman
  sudo cp -f ~/.local/share/hypr/default/pacman/pacman.conf /etc/pacman.conf
  sudo cp -f ~/.local/share/hypr/default/pacman/mirrorlist /etc/pacman.d/mirrorlist

  # Refresh all repos
  sudo pacman -Syu --noconfirm
}

main "$@"
