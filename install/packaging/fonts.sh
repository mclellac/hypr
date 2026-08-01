#!/bin/bash
#
# Installs the custom 'hypr' font for Waybar.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Copies the hypr.ttf font to the user's font directory and rebuilds the font cache.
# Globals:
#   HYPR_PATH (read-only)
#######################################
main() {
    print_header "Installing Fonts"
    local -r font_dir="${HOME}/.local/share/fonts"
    local fonts_changed=false

    mkdir -p "${font_dir}"

    local source_fonts=("${HYPR_PATH}/fonts"/*.ttf "${HYPR_PATH}/fonts"/*.otf)
    
    for src in "${source_fonts[@]}"; do
        [[ -e "$src" ]] || continue
        local filename
        filename=$(basename "$src")
        local dest="${font_dir}/${filename}"

        if [[ ! -f "$dest" ]] || ! cmp -s "$src" "$dest"; then
            log_info "Installing/Updating ${filename}..."
            cp "$src" "$dest"
            fonts_changed=true
        fi
    done

    if [[ "${fonts_changed}" == "true" ]]; then
         log_info "Regenerating font cache..."
         fc-cache -f >/dev/null
    else
         log_info "All fonts are up to date. Skipping cache regeneration."
    fi
}


main "$@"
