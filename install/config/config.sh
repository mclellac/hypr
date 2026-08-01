#!/bin/bash
#
# Copies the core configuration files, themes, and shell settings.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

# --- Source Utils ---
if [[ -n "${HYPR_INSTALL:-}" ]]; then
    source "${HYPR_INSTALL}/utils.sh"
else
    source "$(dirname "$0")/../utils.sh"
fi

#######################################
# Copies the main configuration files from the repo to ~/.config.
# Globals:
#   HYPR_PATH (read-only)
#   VENV_PYTHON (optional)
#   VENV_PIP (optional)
#######################################
copy_main_config() {
    log_info "Installing hypr configurations into ~/.config..."
    mkdir -p "${HOME}/.config"

    # Install Python dependencies for templater into VENV if available
    # install.sh sources utils.sh which sets up VENV.

    # Generate configurations using the templater, but skip if we are applying a target theme in FRESH/FORCE mode anyway
    if [[ -n "${TARGET_THEME:-}" ]] && [[ "${INSTALL_MODE:-}" != "UPDATE" ]]; then
        log_info "Skipping initial template generation as TARGET_THEME is set..."
    else
        log_info "Generating configuration files from templates..."
        if [[ -x "${HYPR_PATH}/bin/hypr-theme-templater" ]]; then
             "${HYPR_PATH}/bin/hypr-theme-templater" --init --no-reload || log_warn "Templater failed during init."
        else
            log_error "hypr-theme-templater not found or not executable."
        fi
    fi
}
main() {
    print_header "Applying System Configuration"
    copy_main_config

    # Ensure user config files exist
    if [[ -f "${HYPR_PATH}/install/config/user_configs.sh" ]]; then
        # Check if the script is executable, if not make it executable
        if [[ ! -x "${HYPR_PATH}/install/config/user_configs.sh" ]]; then
             chmod +x "${HYPR_PATH}/install/config/user_configs.sh"
        fi
        "${HYPR_PATH}/install/config/user_configs.sh"
    else
        log_warn "user_configs.sh not found at ${HYPR_PATH}/install/config/user_configs.sh"
    fi

    # Configure asdcontrol sudo access

    # Configure UTM/QEMU guest integration if in a VM

    log_success "Configuration installation complete."
}

main "$@"
