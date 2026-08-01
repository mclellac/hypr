#!/bin/bash
#
# Installs various terminal user interface (TUI) applications.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Main function to install TUI applications.
#######################################
main() {
    print_header "Installing TUI Applications"
    # Helper wrapper to check existence
    install_if_missing() {
        local name="$1"
        local cmd="$2"
        local style="$3"
        local icon="$4"
        local desktop_file="${HOME}/.local/share/applications/${name}.desktop"

        if [[ -f "${desktop_file}" ]] && [[ "${INSTALL_MODE:-}" != "FRESH" ]]; then
            log_info "TUI app '${name}' already installed. Skipping."
        else
            hypr-tui-install "${name}" "${cmd}" "${style}" "${icon}" >/dev/null
        fi
    }

    install_if_missing "Disk Usage" "bash -c 'dust -r; read -n 1 -s'" float "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/png/qdirstat.png"
    install_if_missing "Docker" "lazydocker" tile "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/png/docker.png"
}

main "$@"
