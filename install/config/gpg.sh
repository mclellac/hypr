#!/bin/bash
#
# Configures GPG with a custom keyserver list for improved reliability.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Copies the GPG configuration and restarts the directory manager.
# Globals:
#   HYPR_PATH (read-only)
#######################################
main() {
    print_header "Configuring GPG"
    local -r gpg_config_dir="/etc/gnupg"
    local -r dirmngr_conf="${gpg_config_dir}/dirmngr.conf"
    local -r src_conf="${HYPR_PATH}/templates/gpg/dirmngr.conf"

    local config_changed=false
    if [[ ! -f "${dirmngr_conf}" ]] || ! cmp -s "${src_conf}" "${dirmngr_conf}"; then
        log_info "Configuring GPG..."
        sudo mkdir -p "${gpg_config_dir}"
        sudo cp "${src_conf}" "${dirmngr_conf}"
        sudo chmod 644 "${dirmngr_conf}"
        config_changed=true
    else
        log_info "GPG configuration is already up to date."
    fi

    if [[ "${config_changed}" == "true" ]]; then
        # Restart the dirmngr to apply the new configuration.
        # The '|| true' prevents the script from exiting if the service isn't running.
        log_info "Restarting GPG directory manager..."
        sudo gpgconf --kill dirmngr >/dev/null 2>&1 || true
        sudo gpgconf --launch dirmngr >/dev/null 2>&1 || true
    fi

    log_info "GPG configuration complete."
}

main "$@"
