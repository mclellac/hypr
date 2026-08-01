#!/bin/bash
#
# Sets up the branding assets for fastfetch and the screensaver.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Copies branding files to the user's config directory.
# Globals:
#   HYPR_PATH (read-only)
#######################################
main() {
    print_header "Setting Up Branding"
    local -r branding_dir="${HOME}/.config/hypr/branding"

    log_info "Setting up branding..."
    mkdir -p "${branding_dir}"

    local -A branding_files=(
        ["${HYPR_PATH}/icon.txt"]="${branding_dir}/about.txt"
        ["${HYPR_PATH}/logo.txt"]="${branding_dir}/screensaver.txt"
    )

    for src in "${!branding_files[@]}"; do
        local dest="${branding_files[$src]}"
        if [[ -f "$src" ]]; then
            if [[ ! -f "$dest" ]] || ! cmp -s "$src" "$dest"; then
                log_info "Updating branding asset: $(basename "$dest")"
                cp "$src" "$dest"
            fi
        fi
    done
}


main "$@"
