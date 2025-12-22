#!/bin/bash
#
# Sets the initial GTK and application themes for the environment.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Sets the GTK theme, color scheme, and icon theme using gsettings.
#######################################
set_gtk_theme() {
    if gsettings list-schemas | grep -q "org.gnome.desktop.interface"; then
        echo "Setting GTK theme..."
        gsettings set org.gnome.desktop.interface gtk-theme "Adwaita-dark"
        gsettings set org.gnome.desktop.interface color-scheme "prefer-dark"
        gsettings set org.gnome.desktop.interface icon-theme "Adwaita"
    fi
}

#######################################
# Creates symbolic links for Nautilus action icons if they are missing.
#######################################
fix_nautilus_icons() {
    local -r adwaita_icon_dir="/usr/share/icons/Adwaita"
    if [[ -d "${adwaita_icon_dir}" ]]; then
        echo "Fixing Nautilus action icons..."
        local -r actions_dir="${adwaita_icon_dir}/scalable/actions"
        sudo mkdir -p "${actions_dir}"
        #sudo ln -snf /usr/share/icons/Adwaita/symbolic/actions/go-previous-symbolic.svg "${actions_dir}/go-previous-symbolic.svg"
        #sudo ln -snf /usr/share/icons/Adwaita/symbolic/actions/go-next-symbolic.svg "${actions_dir}/go-next-symbolic.svg"
        sudo gtk-update-icon-cache "${adwaita_icon_dir}"
    fi
}

#######################################
# Sets up the initial theme symlinks for hyprland.
#######################################
set_initial_hypr_theme() {
    echo "Setting initial hypr theme..."
    local -r current_dir="${HOME}/.config/hypr/current"
    mkdir -p "${current_dir}"
    ln -snf "${HOME}/.config/hypr/themes/ristretto" "${current_dir}/theme"
    ln -snf "${current_dir}/theme/backgrounds/1-ristretto.jpg" "${current_dir}/background"
}

#######################################
# Creates symbolic links for application-specific theme files.
#######################################
link_application_configs() {
    echo "Linking application theme configurations..."
    mkdir -p "${HOME}/.config/btop/themes"
    ln -snf "${HOME}/.config/hypr/current/theme/btop.theme" "${HOME}/.config/btop/themes/current.theme"

    mkdir -p "${HOME}/.config/mako"
    ln -snf "${HOME}/.config/hypr/current/theme/mako.ini" "${HOME}/.config/mako/config"

    mkdir -p "${HOME}/.config/kitty"
    ln -snf "${HOME}/.config/hypr/current/theme/kitty.conf" "${HOME}/.config/kitty/theme.conf"
}

#######################################
# Applies the default theme system-wide using the hypr-theme-set script.
# Globals:
#   HYPR_PATH (read-only)
#######################################
apply_default_theme() {
    echo "Applying default theme..."
    "${HYPR_PATH}/bin/hypr-theme-set" ristretto
}

#######################################
# Main function to orchestrate the theme setup.
#######################################
main() {
    set_gtk_theme
    fix_nautilus_icons
    set_initial_hypr_theme
    link_application_configs
    apply_default_theme
    echo "Theme configuration complete."
}

main "$@"
