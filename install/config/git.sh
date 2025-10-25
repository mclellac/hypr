#!/bin/bash
#
# Configures global Git settings, including aliases and user identity.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Sets common and convenient Git aliases.
#######################################
set_git_aliases() {
  echo "Setting global Git aliases..."
  git config --global alias.co checkout
  git config --global alias.br branch
  git config --global alias.ci commit
  git config --global alias.st status
  git config --global pull.rebase true
  git config --global init.defaultBranch master
}

#######################################
# Configures the global user name and email from environment variables.
# Globals:
#   hypr_USER_NAME (read-only)
#   hypr_USER_EMAIL (read-only)
#######################################
set_user_identity() {
  echo "Configuring Git user identity..."
  # The parameter expansion `${VAR//[[:space:]]/}` removes all whitespace.
  if [[ -n "${hypr_USER_NAME//[[:space:]]/}" ]]; then
    git config --global user.name "${hypr_USER_NAME}"
  fi

  if [[ -n "${hypr_USER_EMAIL//[[:space:]]/}" ]]; then
    git config --global user.email "${hypr_USER_EMAIL}"
  fi
}

#######################################
# Main function to orchestrate Git configuration.
#######################################
main() {
  set_git_aliases
  set_user_identity
  echo "Git configuration complete."
}

main "$@"
