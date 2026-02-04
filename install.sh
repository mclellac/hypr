#!/bin/bash
#
# Main installation script for the hyprland environment.

# Exit immediately if a command exits with a non-zero status.
# Exit if any command in a pipeline fails.
# Exit if an unset variable is used.
set -euo pipefail

main() {
  # Constants
  readonly HYPR_PATH="$(cd "$(dirname "$0")" && pwd)"
  readonly HYPR_INSTALL="${HYPR_PATH}/install"
  export PATH="${HYPR_PATH}/bin:${PATH}"

  # Ensure the repository is linked to the expected location
  local target_dir="${HOME}/.local/share/hypr"
  if [[ "${HYPR_PATH}" != "${target_dir}" ]]; then
    echo "Ensuring repository is available at ${target_dir}..."
    mkdir -p "$(dirname "${target_dir}")"
    if [[ -d "${target_dir}" || -L "${target_dir}" ]]; then
      echo "Backing up existing directory at ${target_dir}..."
      mv "${target_dir}" "${target_dir}.bak.$(date +%s)"
    fi
    ln -sf "${HYPR_PATH}" "${target_dir}"
  fi

  # Packaging
  # source "${HYPR_INSTALL}/packages.sh"
  source "${HYPR_INSTALL}/packaging/fonts.sh"
  source "${HYPR_INSTALL}/packaging/webapps.sh"
  source "${HYPR_INSTALL}/packaging/tuis.sh"

  # Configuration
  source "${HYPR_INSTALL}/config/config.sh"
  source "${HYPR_INSTALL}/packaging/lazyvim.sh"
  source "${HYPR_INSTALL}/config/theme.sh"
  source "${HYPR_INSTALL}/config/branding.sh"
  source "${HYPR_INSTALL}/config/git.sh"
  source "${HYPR_INSTALL}/config/gpg.sh"
  source "${HYPR_INSTALL}/config/xcompose.sh"
  source "${HYPR_INSTALL}/config/mise-ruby.sh"
  source "${HYPR_INSTALL}/config/docker.sh"
  source "${HYPR_INSTALL}/config/mimetypes.sh"
  source "${HYPR_INSTALL}/config/localdb.sh"
}

main "$@"
