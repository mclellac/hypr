#!/bin/bash
#
# Pre-flight checks to ensure the installation environment is suitable.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Prints an error message and prompts the user to continue or exit.
# Arguments:
#   Error message string.
#######################################
prompt_and_exit_on_fail() {
  echo -e "\e[31mhypr install requires: $1\e[0m"
  echo
  # Assuming 'gum' is a dependency that will be present.
  gum confirm "Proceed anyway on your own accord and without assistance?" || exit 1
}

#######################################
# Checks if the operating system is vanilla Arch Linux.
#######################################
check_is_arch_distro() {
  [[ -f /etc/arch-release ]] || prompt_and_exit_on_fail "A vanilla Arch Linux distribution"
}

#######################################
# Checks for the presence of derivative Arch distribution markers.
#######################################
check_is_not_derivative() {
  local marker
  readonly markers=(
    /etc/cachyos-release
    /etc/eos-release
    /etc/garuda-release
    /etc/manjaro-release
  )
  for marker in "${markers[@]}"; do
    if [[ -f "${marker}" ]]; then
      prompt_and_exit_on_fail "A vanilla Arch Linux distribution (not a derivative)"
    fi
  done
}

#######################################
# Ensures the script is not being run as root.
#######################################
check_is_not_root() {
  [[ "${EUID}" -ne 0 ]] || prompt_and_exit_on_fail "Running as a non-root user"
}

#######################################
# Verifies the system architecture is x86_64.
#######################################
check_architecture() {
  [[ "$(uname -m)" == "x86_64" ]] || prompt_and_exit_on_fail "x86_64 CPU"
}

#######################################
# Checks if GNOME or KDE desktop environments are already installed.
#######################################
check_no_desktop_environment() {
  if pacman -Qe gnome-shell &>/dev/null; then
    prompt_and_exit_on_fail "A fresh install (GNOME detected)"
  fi
  if pacman -Qe plasma-desktop &>/dev/null; then
    prompt_and_exit_on_fail "A fresh install (KDE Plasma detected)"
  fi
}

#######################################
# Main function to run all pre-flight checks.
#######################################
main() {
  check_is_arch_distro
  check_is_not_derivative
  check_is_not_root
  check_architecture
  check_no_desktop_environment

  echo "Guards: OK"
}

main "$@"
