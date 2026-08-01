#!/bin/bash
#
# Configures global Git settings, including aliases and user identity.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Sets common and convenient Git aliases.
#######################################
set_git_aliases() {
    log_info "Setting global Git aliases..."
    
    # Only set aliases if they don't already exist
    for alias in co=checkout br=branch ci=commit st=status; do
        local key="${alias%=*}"
        local val="${alias#*=}"
        if ! git config --global --get "alias.${key}" >/dev/null 2>&1; then
            git config --global "alias.${key}" "${val}"
        fi
    done

    # Only set pull.rebase and init.defaultBranch if they don't already exist
    if ! git config --global --get pull.rebase >/dev/null 2>&1; then
        git config --global pull.rebase true
    fi

    if ! git config --global --get init.defaultBranch >/dev/null 2>&1; then
        git config --global init.defaultBranch master
    fi
}

#######################################
# Configures the global user name and email from environment variables.
# Globals:
#   HYPR_USER_NAME (read-only)
#   HYPR_USER_EMAIL (read-only)
#######################################
set_user_identity() {
    log_info "Configuring Git user identity..."
    # The parameter expansion `${VAR//[[:space:]]/}` removes all whitespace.
    if [[ -n "${HYPR_USER_NAME:-}" ]]; then
            if [[ -n "${HYPR_USER_NAME//[[:space:]]/}" ]]; then
                git config --global user.name "${HYPR_USER_NAME}"
            fi
    fi

    if [[ -n "${HYPR_USER_EMAIL:-}" ]]; then
            if [[ -n "${HYPR_USER_EMAIL//[[:space:]]/}" ]]; then
                git config --global user.email "${HYPR_USER_EMAIL}"
            fi
    fi
}

#######################################
# Main function to orchestrate Git configuration.
#######################################
main() {
    print_header "Configuring Git"
    set_git_aliases
    set_user_identity
    log_info "Git configuration complete."
}

main "$@"
