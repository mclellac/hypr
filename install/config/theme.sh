#!/bin/bash
#
# Sets the initial GTK and application themes for the environment.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

# --- Source Utils ---
if [[ -n "${HYPR_INSTALL:-}" ]]; then
    source "${HYPR_INSTALL}/utils.sh"
else
    source "$(dirname "$0")/../utils.sh"
fi

#######################################
# Creates symbolic links for Nautilus action icons if they are missing.
#######################################
fix_nautilus_icons() {
    local -r adwaita_icon_dir="/usr/share/icons/Adwaita"
    if [[ -d "${adwaita_icon_dir}" ]]; then
        log_info "Fixing Nautilus action icons..."
        sudo gtk-update-icon-cache "${adwaita_icon_dir}" || true
    fi
}

#######################################
# Applies the theme system-wide using the hypr-theme-set script.
# Globals:
#   HYPR_PATH (read-only)
#   INSTALL_MODE (optional)
#   TARGET_THEME (optional)
#######################################
apply_theme() {
    # Determine which theme to use
    local theme_to_apply=""

    if [[ -n "${TARGET_THEME:-}" ]]; then
        theme_to_apply="${TARGET_THEME}"
        log_info "Explicit theme requested: ${theme_to_apply}"
    elif [[ "${INSTALL_MODE:-}" == "UPDATE" ]]; then
        # Check if a theme is already active
        local current_theme_path="${HOME}/.config/hypr/current_theme_path"
        if [[ -f "${current_theme_path}" ]]; then
            theme_to_apply=$(cat "${current_theme_path}" | xargs basename)
            log_info "Update mode: Re-applying active theme '${theme_to_apply}' to sync latest configuration colors."
        else
            log_warn "Update mode, but no active theme found. Falling back to default."
            theme_to_apply="amateur"
        fi
    else
        # Fresh install or force mode defaults
        theme_to_apply="amateur"
    fi

    log_info "Applying theme '${theme_to_apply}'..."
    # bin/hypr-theme-set is now a shell script that handles its own environment.
    if [[ -x "${HYPR_PATH}/bin/hypr-theme-set" ]]; then
         "${HYPR_PATH}/bin/hypr-theme-set" --no-reload "${theme_to_apply}" >/dev/null
    else
         log_error "Theme script not found: ${HYPR_PATH}/bin/hypr-theme-set"
         exit 1
    fi
}

#######################################
# Cleans up system-wide browser policy directories under /etc.
#######################################
cleanup_system_policies() {
    log_info "Cleaning up system-wide browser policies under /etc to release organizational settings locks..."
    local -a policy_dirs=(
        "/etc/chromium/policies"
        "/etc/opt/chrome/policies"
        "/etc/brave/policies"
        "/etc/vivaldi/policies"
        "/etc/thorium/policies"
        "/etc/opt/edge/policies"
    )
    for pdir in "${policy_dirs[@]}"; do
        for folder in "managed" "recommended"; do
            if [[ -d "${pdir}/${folder}" ]]; then
                sudo rm -f "${pdir}/${folder}/theme.json" "${pdir}/${folder}/color.json" || true
                sudo rmdir "${pdir}/${folder}" 2>/dev/null || true
            fi
        done
        sudo rmdir "${pdir}" 2>/dev/null || true
    done
}

#######################################
# Cleans up old browser wrapper symlinks and local desktop overrides.
#######################################
cleanup_browser_wrappers() {
    log_info "Cleaning up old browser wrapper symlinks..."
    local -r bin_dir="${HYPR_PATH}/bin"
    
    local -a wrappers=(
        "google-chrome"
        "google-chrome-stable"
        "google-chrome-beta"
        "google-chrome-unstable"
        "google-chrome-dev"
        "chromium"
        "brave-browser"
        "vivaldi"
        "vivaldi-stable"
        "vivaldi-snapshot"
        "thorium"
        "thorium-browser"
        "microsoft-edge"
        "microsoft-edge-stable"
        "microsoft-edge-beta"
        "microsoft-edge-dev"
        "microsoft-edge-unstable"
        "opera"
        "opera-beta"
        "opera-developer"
    )
    
    for name in "${wrappers[@]}"; do
        if [[ -L "${bin_dir}/${name}" ]]; then
            rm -f "${bin_dir}/${name}"
        fi
        if [[ -L "${HOME}/.local/bin/${name}" ]]; then
            rm -f "${HOME}/.local/bin/${name}"
        fi
    done
    
    # Clean up user overridden desktop entries we generated earlier
    local -r local_apps_dir="${HOME}/.local/share/applications"
    if [[ -d "${local_apps_dir}" ]]; then
        rm -f "${local_apps_dir}/google-chrome.desktop" \
              "${local_apps_dir}/google-chrome-unstable.desktop" \
              "${local_apps_dir}/chromium.desktop" \
              "${local_apps_dir}/brave-browser.desktop" \
              "${local_apps_dir}/brave.desktop" || true
    fi
}

################################
# Main function to orchestrate the theme setup.
#######################################
main() {
    print_header "Applying System Theme"
    fix_nautilus_icons
    cleanup_system_policies
    cleanup_browser_wrappers
    apply_theme
    log_success "Theme configuration complete."
}

main "$@"
