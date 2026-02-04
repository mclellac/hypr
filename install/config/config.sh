#!/bin/bash
#
# Copies the core configuration files, themes, and shell settings.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Copies the main configuration files from the repo to ~/.config.
# Globals:
#   HYPR_PATH (read-only)
#######################################
copy_main_config() {
  echo "Installing hypr configurations into ~/.config..."
  mkdir -p "${HOME}/.config"
  cp -R "${HYPR_PATH}"/config/* "${HOME}/.config/"

  # Copy .desktop files if they exist and sanitize paths
  if [[ -d "${HYPR_PATH}/applications" ]]; then
    echo "Installing application shortcuts..."
    mkdir -p "${HOME}/.local/share/applications"
    cp "${HYPR_PATH}"/applications/*.desktop "${HOME}/.local/share/applications/"
    # Replace placeholder with actual home directory
    sed -i "s|/HOME_PLACEHOLDER/|${HOME}/|g" "${HOME}/.local/share/applications/"*.desktop
  fi

  if command -v systemd-detect-virt >/dev/null 2>&1; then
    if systemd-detect-virt >/dev/null; then
      echo "Virtualization detected. Applying VM specific environment settings..."
      cp "${HOME}/.config/hypr/envs.conf.vms" "${HOME}/.config/hypr/envs.conf"
    fi
  fi
}

#######################################
# Copies all themes and ensures a 'backgrounds' subdirectory exists for each.
# Globals:
#   HYPR_PATH (read-only)
#######################################
copy_themes() {
  echo "Copying themes and ensuring backgrounds directory exists..."
  local -r themes_dest_dir="${HOME}/.config/hypr/themes"
  mkdir -p "${themes_dest_dir}"

  local theme_dir
  for theme_dir in "${HYPR_PATH}"/themes/*; do
    if [[ -d "${theme_dir}" ]]; then
      # Copy the full theme directory to the destination
      cp -R "${theme_dir}" "${themes_dest_dir}/"
      # Ensure the backgrounds subdirectory exists in the new location
      local theme_name
      theme_name=$(basename "${theme_dir}")
      mkdir -p "${themes_dest_dir}/${theme_name}/backgrounds"
    fi
  done
}

#######################################
# Installs the default .bashrc file.
# Globals:
#   HYPR_PATH (read-only)
#######################################
install_bashrc() {
  echo "Installing .bashrc..."
  cp -R "${HYPR_PATH}/default/bashrc" "${HOME}/.bashrc"
}

#######################################
# Main function to orchestrate the configuration file installation.
#######################################
main() {
  copy_main_config
  copy_themes

  echo "Creating current theme directory..."
  mkdir -p "${HOME}/.config/hypr/current"

  install_bashrc
  echo "Configuration installation complete."
}

main "$@"
