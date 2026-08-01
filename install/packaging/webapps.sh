#!/bin/bash
#
# Installs various web applications as desktop entries.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

# --- Source Utils ---
if [[ -n "${HYPR_INSTALL:-}" ]]; then
    source "${HYPR_INSTALL}/utils.sh"
else
    source "$(dirname "$0")/../utils.sh"
fi

#######################################
# Main function to install web applications.
#######################################
main() {
    print_header "Installing Web Applications"
    log_info "Installing web applications..."

    # Helper wrapper to check existence
    install_if_missing() {
        local name="$1"
        local url="$2"
        local icon="$3"
        local desktop_file="${HOME}/.local/share/applications/${name}.desktop"

        if [[ -f "${desktop_file}" ]] && [[ "${INSTALL_MODE:-}" != "FRESH" ]]; then
            log_info "Web app '${name}' already installed. Skipping."
        else
            hypr-webapp-install "${name}" "${url}" "${icon}" >/dev/null
        fi
    }

    install_if_missing "WhatsApp" "https://web.whatsapp.com/" "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/png/whatsapp.png"
    install_if_missing "Google Photos" "https://photos.google.com/" "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/png/google-photos.png"
    install_if_missing "Google Contacts" "https://contacts.google.com/" "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/png/google-contacts.png"
    install_if_missing "Google Messages" "https://messages.google.com/web/conversations" "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/png/google-messages.png"
    install_if_missing "Google Meet" "https://meet.google.com/" "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/png/google-meet.png"
    install_if_missing "Google Jules" "https://jules.google/" "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/png/google-gemini.png"
    install_if_missing "ChatGPT" "https://chatgpt.com/" "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/png/chatgpt.png"
    install_if_missing "YouTube" "https://youtube.com/" "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/png/youtube.png"
    install_if_missing "YouTube Music" "https://music.youtube.com/" "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/png/youtube-music.png"
    install_if_missing "Reddit" "https://www.reddit.com/" "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/png/reddit.png"
    install_if_missing "GitHub" "https://github.com/" "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/png/github-light.png"
    install_if_missing "Discord" "https://discord.com/channels/@me" "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/png/discord.png"
    install_if_missing "Zoom" "https://app.zoom.us/wc/home" "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/png/zoom.png"
}

main "$@"
