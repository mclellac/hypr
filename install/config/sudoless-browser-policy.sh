#!/usr/bin/env bash
# Configures passwordless sudo access for the browser theme policy helper.
set -euo pipefail

# --- Source Utils ---
if [[ -n "${HYPR_INSTALL:-}" ]]; then
    source "${HYPR_INSTALL}/utils.sh"
else
    source "$(dirname "$0")/../utils.sh"
fi

main() {
    print_header "Configuring Sudo-less Browser Theme Policy Updates"
    
    local -r sudoers_file="/etc/sudoers.d/hypr-theme-browser-policy"
    local -r policy_script="${HYPR_PATH}/bin/hypr-theme-set-browser-policy"
    
    log_info "Creating sudoers rule for browser theme updates..."
    
    # Allow the current user to run the script with sudo without a password
    echo "${USER} ALL=(ALL) NOPASSWD: ${policy_script}" | sudo tee "${sudoers_file}" >/dev/null
    
    # Set restrictive permissions for the sudoers file
    sudo chmod 440 "${sudoers_file}"
    
    log_success "Sudo-less browser theme updates configured."
}

main "$@"
