#!/bin/bash
#
# Sets default applications for various MIME types by directly modifying
# ~/.config/mimeapps.list to avoid hangs with xdg-mime.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

# --- Source Utils ---
if [[ -n "${HYPR_INSTALL:-}" ]]; then
    source "${HYPR_INSTALL}/utils.sh"
else
    source "$(dirname "$0")/../utils.sh"
fi

readonly MIME_FILE="${HOME}/.config/mimeapps.list"

#######################################
# Ensures the [Default Applications] section exists in the mime file.
#######################################
ensure_mime_section() {
    mkdir -p "$(dirname "${MIME_FILE}")"
    touch "${MIME_FILE}"

    if ! grep -q "\[Default Applications\]" "${MIME_FILE}"; then
        # If file is not empty, ensure it ends with a newline
        [[ -s "${MIME_FILE}" ]] && echo "" >> "${MIME_FILE}"
        echo "[Default Applications]" >> "${MIME_FILE}"
    fi
}

#######################################
# Sets a default application for a MIME type.
# Arguments:
#   $1 - Desktop file name (e.g., imv.desktop)
#   $2 - MIME type (e.g., image/png)
#######################################
set_mime_default() {
    local -r desktop_file="$1"
    local -r mime_type="$2"

    # Check if the mime type already has a default assigned in the [Default Applications] section.
    # We skip setting it if it's already defined to respect user preferences.
    if sed -n '/\[Default Applications\]/,/^\s*\[/p' "${MIME_FILE}" | grep -q "^${mime_type}="; then
        return 0
    fi
    
    # Add the new entry after the section header
    sed -i "/\[Default Applications\]/a ${mime_type}=${desktop_file}" "${MIME_FILE}"
}

#######################################
# Refreshes the application database.
#######################################
refresh_database() {
    log_info "Refreshing application database..."
    hypr-refresh-applications
    if command -v update-desktop-database &>/dev/null; then
        update-desktop-database "${HOME}/.local/share/applications"
    fi
}

#######################################
# Sets the default image viewer.
#######################################
set_image_defaults() {
    # Check if any image mime type already has a default
    if sed -n '/\[Default Applications\]/,/^\s*\[/p' "${MIME_FILE}" | grep -qE "^image/(png|jpeg|gif)="; then
        log_info "Existing image viewer detected. Skipping image defaults."
        return 0
    fi

    log_info "Setting default image viewer to imv.desktop..."
    local -r desktop_file="imv.desktop"
    local -ra mime_types=(
        "image/png"
        "image/jpeg"
        "image/gif"
        "image/webp"
        "image/bmp"
        "image/tiff"
    )
    local mime_type
    for mime_type in "${mime_types[@]}"; do
        set_mime_default "${desktop_file}" "${mime_type}"
    done
}

#######################################
# Sets the default PDF viewer.
#######################################
set_pdf_defaults() {
    if sed -n '/\[Default Applications\]/,/^\s*\[/p' "${MIME_FILE}" | grep -q "^application/pdf="; then
        log_info "Existing PDF viewer detected. Skipping PDF defaults."
        return 0
    fi

    log_info "Setting default PDF viewer to org.gnome.Evince.desktop..."
    set_mime_default "org.gnome.Evince.desktop" "application/pdf"
}

#######################################
# Sets the default web browser.
#######################################
set_browser_defaults() {
    # Check if a default browser is already set via xdg-settings.
    local current_browser
    if current_browser=$(timeout 1s env -u KDE_SESSION_VERSION -u KDE_FULL_SESSION xdg-settings get default-web-browser 2>/dev/null) && [[ -n "${current_browser}" ]]; then
        log_info "Default web browser is already set to '${current_browser}'. Skipping browser configuration."
        return 0
    fi

    # Secondary check: If any core browser mime types are already set in the file
    if sed -n '/\[Default Applications\]/,/^\s*\[/p' "${MIME_FILE}" | grep -qE "^(x-scheme-handler/https|x-scheme-handler/http|text/html)="; then
        log_info "Existing browser associations detected. Skipping browser configuration."
        return 0
    fi

    log_info "No default browser detected. Setting to chromium.desktop..."
    local -r desktop_file="chromium.desktop"
    
    set_mime_default "${desktop_file}" "x-scheme-handler/http"
    set_mime_default "${desktop_file}" "x-scheme-handler/https"
    set_mime_default "${desktop_file}" "text/html"
    set_mime_default "${desktop_file}" "application/x-extension-htm"
    set_mime_default "${desktop_file}" "application/x-extension-html"
    set_mime_default "${desktop_file}" "application/x-extension-shtml"
    set_mime_default "${desktop_file}" "application/xhtml+xml"
    set_mime_default "${desktop_file}" "application/x-extension-xhtml"
    set_mime_default "${desktop_file}" "application/x-extension-xht"
}

#######################################
# Sets the default video player.
#######################################
set_video_defaults() {
    # Check if any video mime type already has a default
    if sed -n '/\[Default Applications\]/,/^\s*\[/p' "${MIME_FILE}" | grep -qE "^video/(mp4|x-matroska|mpeg)="; then
        log_info "Existing video player detected. Skipping video defaults."
        return 0
    fi

    log_info "Setting default video player to mpv.desktop..."
    local -r desktop_file="mpv.desktop"
    local -ra mime_types=(
        "video/mp4"
        "video/x-msvideo"
        "video/x-matroska"
        "video/x-flv"
        "video/x-ms-wmv"
        "video/mpeg"
        "video/ogg"
        "video/webm"
        "video/quicktime"
        "video/3gpp"
        "video/3gpp2"
        "video/x-ms-asf"
        "video/x-ogm+ogg"
        "video/x-theora+ogg"
        "application/ogg"
    )
    local mime_type
    for mime_type in "${mime_types[@]}"; do
        set_mime_default "${desktop_file}" "${mime_type}"
    done
}

#######################################
# Sets the default file manager.
#######################################
set_file_manager_defaults() {
    if sed -n '/\[Default Applications\]/,/^\s*\[/p' "${MIME_FILE}" | grep -q "^inode/directory="; then
        log_info "Existing file manager association detected. Skipping directory defaults."
        return 0
    fi

    log_info "Setting default file manager to org.gnome.Nautilus.desktop..."
    set_mime_default "org.gnome.Nautilus.desktop" "inode/directory"
}

#######################################
# Main function to orchestrate MIME type configuration.
#######################################
main() {
    print_header "Configuring MIME Types"
    refresh_database
    ensure_mime_section
    set_image_defaults
    set_pdf_defaults
    set_browser_defaults
    set_video_defaults
    set_file_manager_defaults
    log_info "MIME type configuration complete."
}

main "$@"
