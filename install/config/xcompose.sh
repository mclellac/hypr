#!/bin/bash
#
# Creates a default .XCompose file for custom key compositions.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Creates the ~/.XCompose file with default and user-specific bindings.
# Globals:
#   HYPR_PATH (read-only)
#   HYPR_USER_NAME (read-only)
#   HYPR_USER_EMAIL (read-only)
#######################################
main() {
    print_header "Setting Up Keyboard Compose"
    local -r xcompose_file="${HOME}/.XCompose"
    local -r temp_xcompose=$(mktemp)

    # Use provided globals or fallback to git config
    local user_name="${HYPR_USER_NAME:-$(git config --global user.name || echo "")}"
    local user_email="${HYPR_USER_EMAIL:-$(git config --global user.email || echo "")}"

    # Generate content into a temporary file
    cat > "${temp_xcompose}" <<EOF
# Include the base xcompose file from the hypr repository.
include "${HYPR_PATH}/templates/xcompose"
EOF

    # Append user info if available
    if [[ -n "${user_name}" ]]; then
        cat >> "${temp_xcompose}" <<EOF

# Custom user identification bindings, triggered with <CapsLock> <space> <key>.
<Multi_key> <space> <n> : "${user_name}"
EOF
    fi

    if [[ -n "${user_email}" ]]; then
        cat >> "${temp_xcompose}" <<EOF
<Multi_key> <space> <e> : "${user_email}"
EOF
    fi

    # Compare and update if necessary
    if [[ ! -f "${xcompose_file}" ]] || ! cmp -s "${temp_xcompose}" "${xcompose_file}"; then
        log_info "Updating .XCompose file..."
        cp "${temp_xcompose}" "${xcompose_file}"
    else
        log_info ".XCompose file is up to date."
    fi

    rm -f "${temp_xcompose}"
}


main "$@"
