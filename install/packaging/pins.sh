#!/bin/bash
#
# Installs pinned packages that are held back from upstream.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Installs packages from a predefined list of pinned versions.
#######################################
main() {
  # The `mapfile` command reads lines from standard input into an array.
  # This is safer than simple command substitution if filenames contain spaces.
  local -a pinned_packages
  mapfile -t pinned_packages < <(hypr-pkg-pinned)

  if [[ "${#pinned_packages[@]}" -gt 0 ]]; then
    echo -e "\e[32m\nInstall pinned system packages\e[0m"

    local pinned
    for pinned in "${pinned_packages[@]}"; do
      echo "Installing pinned package: ${pinned}"
      sudo pacman -U --noconfirm "${pinned}"
    done
  fi
}

main "$@"
