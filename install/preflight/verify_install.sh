#!/bin/bash
#
# Verifies that critical packages and commands are available after installation.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

# --- Source Utils ---
if [[ -n "${HYPR_INSTALL:-}" ]]; then
    # shellcheck source=install/utils.sh
    source "${HYPR_INSTALL}/utils.sh"
else
    # shellcheck source=install/utils.sh
    source "$(dirname "$0")/../utils.sh"
fi

main() {
    print_header "Verifying Installation"

    local -a critical_cmds=(
        "hyprland"
        "hyprctl"
        "waybar"
        "mako"
        "walker"
        "elephant"
        "hypr-preferences-app"
        "hypr-keybindings-editor"
        "hypr-theme-set"
        "docker"
        "updatedb"
        "git"
        "curl"
        "python3"
        "pip"
    )

    local missing=0
    for cmd in "${critical_cmds[@]}"; do
        if [[ "${cmd}" == "pip" ]]; then
            if ! has_command "pip" && ! has_command "pip3"; then
                log_error "Critical command 'pip' or 'pip3' is missing after installation."
                missing=1
            fi
            continue
        fi

        if ! has_command "${cmd}"; then
            log_error "Critical command '${cmd}' is missing after installation."
            missing=1
        fi
    done

    if [[ ${missing} -eq 1 ]]; then
        log_error "Some critical packages failed to install or are not in PATH."
        log_info "Please check the package installation logs and ensure your PATH is correct."
        exit 1
    fi

    log_success "All critical commands are present."
}

main "$@"
