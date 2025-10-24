#!/bin/bash
#
# Sets up the Neovim configuration using the LazyVim starter.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

# Constants
readonly NVIM_CONFIG_DIR="${HOME}/.config/nvim"
readonly HYPR_NVIM_CONFIG_DIR="${HYPR_PATH}/config/nvim"

#######################################
# Clones the LazyVim starter configuration if it doesn't already exist.
#######################################
clone_lazyvim_if_needed() {
  if [[ ! -d "${NVIM_CONFIG_DIR}" ]]; then
    echo "Cloning LazyVim starter configuration..."
    git clone https://github.com/LazyVim/starter "${NVIM_CONFIG_DIR}"
    rm -rf "${NVIM_CONFIG_DIR}/.git"
  fi
}

#######################################
# Copies the hyprland nvim configuration files without overwriting.
# Ensures custom user directories exist and removes the managed theme file.
#######################################
setup_hypr_config() {
  # Ensure the 'user' directory exists for custom configs
  mkdir -p "${NVIM_CONFIG_DIR}/lua/user"

  # Remove the theme file if it exists, as it's managed by theme.sh
  # and can cause issues on re-runs.
  rm -f "${NVIM_CONFIG_DIR}/lua/plugins/theme.lua"

  # Copy hypr's base nvim configuration, but don't overwrite existing files
  echo "Copying hypr nvim configuration..."
  cp -n -R "${HYPR_NVIM_CONFIG_DIR}/"* "${NVIM_CONFIG_DIR}/"
}

#######################################
# Appends a line to the Neovim options file to disable relative numbers,
# if not already present.
#######################################
disable_relative_numbers() {
  local -r options_dir="${NVIM_CONFIG_DIR}/lua/config"
  local -r options_file="${options_dir}/options.lua"
  local -r setting="vim.opt.relativenumber = false"

  mkdir -p "${options_dir}"
  if ! grep -qF "${setting}" "${options_file}" 2>/dev/null; then
    echo "Disabling relative numbers in Neovim..."
    echo "${setting}" >>"${options_file}"
  fi
}

#######################################
# Main function to orchestrate the Neovim setup.
#######################################
main() {
  clone_lazyvim_if_needed
  setup_hypr_config
  disable_relative_numbers
  echo "Neovim configuration setup complete."
}

main "$@"
