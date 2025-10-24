#!/bin/bash
#
# Installs various terminal user interface (TUI) applications.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Main function to install TUI applications.
#######################################
main() {
  hypr-tui-install "Disk Usage" "bash -c 'dust -r; read -n 1 -s'" float "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/png/qdirstat.png"
  hypr-tui-install "Docker" "lazydocker" tile "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/png/docker.png"
}

main "$@"
