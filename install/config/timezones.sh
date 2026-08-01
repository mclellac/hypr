#!/bin/bash
#
# Configures passwordless sudo access for timezone-related commands.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Creates a sudoers file to allow members of the 'wheel' group to update
# the timezone without a password.
#######################################
main() {
    local -r sudoers_file="/etc/sudoers.d/hypr-tzupdate"
    local -r temp_sudoers=$(mktemp)

    cat > "${temp_sudoers}" <<EOF
# Allow users in the wheel group to run tzupdate and timedatectl without a password.
%wheel ALL=(root) NOPASSWD: /usr/bin/tzupdate, /usr/bin/timedatectl
EOF

    if [[ ! -f "${sudoers_file}" ]] || ! cmp -s "${temp_sudoers}" "${sudoers_file}"; then
        log_info "Configuring sudo-less access for timezone updates..."
        sudo cp "${temp_sudoers}" "${sudoers_file}"
        sudo chmod 0440 "${sudoers_file}"
    else
        log_info "Timezone update configuration is already up to date."
    fi

    rm -f "${temp_sudoers}"
}

main "$@"
