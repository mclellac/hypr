#!/bin/bash
#
# Sets up the Neovim configuration using the LazyVim starter.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

# Constants
readonly NVIM_CONFIG_DIR="${HOME}/.config/nvim"

#######################################
# Appends a line to the Neovim options file to disable relative numbers,
# if not already present.
#######################################
disable_relative_numbers() {
    local -r options_dir="${NVIM_CONFIG_DIR}/lua/config"
    local -r options_file="${options_dir}/options.lua"
    local -r setting="vim.opt.relativenumber = false"

    mkdir -p "${options_dir}"
    if [[ -f "${options_file}" ]] && ! grep -qF "${setting}" "${options_file}" 2>/dev/null; then
        log_info "Disabling relative numbers in Neovim..."
        echo "${setting}" >>"${options_file}"
    fi
}

#######################################
# Main function to orchestrate the Neovim setup.
#######################################
main() {
    print_header "Configuring Neovim (LazyVim)"
    # Remove the theme file if it exists, as it's managed by theme.sh
    # and can cause issues on re-runs.
    rm -f "${NVIM_CONFIG_DIR}/lua/plugins/theme.lua"

    disable_relative_numbers
    log_info "Neovim configuration setup complete."
}

main "$@"
