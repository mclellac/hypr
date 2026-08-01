#!/bin/bash
#
# Main installation script for the hyprland environment.
# Updated to be intelligent, idempotent, and non-destructive.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

# --- Source Utils ---
source "$(dirname "$0")/install/utils.sh"

show_help() {
    echo "Usage: ./install.sh [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --update         Update code and templates only (no theme reset, no user config overwrite)"
    echo "  --packages       Only check and install missing system packages"
    echo "  --force          Force a full re-installation (may overwrite configs)"
    echo "  --norust         Skip compiling and building Rust applications"
    echo "  --theme <name>   Set a specific theme after installation"
    echo "  --help           Show this help message"
    exit 0
}

preflight_check() {
    print_header "Preflight Check"

    local missing=0

    # Critical tools for the installer itself
    for tool in sudo curl git python3 cargo; do
        if ! has_command "${tool}"; then
            log_error "Required tool '${tool}' is missing."
            missing=1
        fi
    done

    if [[ ${missing} -eq 1 ]]; then
        log_info "Please install missing tools before running the installer."
        exit 1
    fi

    if ! is_arch_based; then
        log_error "This installer currently only supports Arch-based distributions (Arch, Manjaro, EndeavourOS, etc.)."
        log_warn "Detected OS: $(check_os)"
        exit 1
    fi

    # Check for systemd
    if ! has_command systemctl; then
        log_error "Systemd (systemctl) is missing. This environment requires systemd."
        exit 1
    fi

    log_success "Preflight check passed."
}

main() {
    # Default variables
    local update_mode=false
    local packages_only=false
    local force_install=false
    local skip_rust=false
    local target_theme=""

    # --- Parse Arguments ---
    while [[ $# -gt 0 ]]; do
        case "$1" in
        --update)
            update_mode=true
            shift
            ;;
        --packages)
            packages_only=true
            shift
            ;;
        --force)
            force_install=true
            shift
            ;;
        --norust)
            skip_rust=true
            shift
            ;;
        --theme)
            if [[ -n "${2:-}" ]]; then
                target_theme="$2"
                shift 2
            else
                log_error "Missing theme name for --theme argument."
                exit 1
            fi
            ;;
        --help)
            show_help
            ;;
        *)
            log_error "Unknown argument: $1"
            show_help
            ;;
        esac
    done

    # Define a helper function to verify binary compatibility
    is_compatible_bin() {
        local bin_path="$1"
        if [[ -f "${bin_path}" ]] && [[ -x "${bin_path}" ]]; then
            local bin_info
            bin_info=$(file -b "${bin_path}" 2>/dev/null || echo "")
            if [[ "$(uname -m)" == "aarch64" ]]; then
                if [[ "${bin_info}" =~ "ARM" || "${bin_info}" =~ "aarch64" ]]; then
                    return 0
                fi
            elif [[ "$(uname -m)" == "x86_64" ]]; then
                if [[ "${bin_info}" =~ "x86-64" || "${bin_info}" =~ "x86_64" ]]; then
                    return 0
                fi
            else
                if "${bin_path}" --help >/dev/null 2>&1; then
                    return 0
                fi
            fi
        fi
        return 1
    }

    # Constants
    HYPR_PATH="$(cd "$(dirname "$0")" && pwd)"
    export HYPR_PATH
    readonly HYPR_INSTALL="${HYPR_PATH}/install"
    export PATH="${HYPR_PATH}/bin:${PATH}"

    preflight_check

    local source_path="${HYPR_PATH}"
    local target_path="${HOME}/.local/share/hypr"
    # Ensure target directory exists
    mkdir -p "${target_path}"

    # Check for compatible pre-compiled or previously built binaries.
    # If skip_rust is true, we verify if the user already has compatible binaries
    # in target_path/bin/, source_path/target/release/, or source_path/bin/.
    # If they do, we back them up and preserve them. Otherwise, we force compilation.
    local -a rust_binaries=(
        "hypr-theme-templater-bin"
        "hypr-theme-get-bin"
        "hypr-preferences-app-bin"
        "keybindings_editor-bin"
    )

    local backup_dir=""
    if [[ "${skip_rust}" == "true" ]]; then
        local missing_compatible=false
        backup_dir=$(mktemp -d)

        for bin in "${rust_binaries[@]}"; do
            if is_compatible_bin "${source_path}/target/release/${bin}"; then
                cp -f "${source_path}/target/release/${bin}" "${backup_dir}/${bin}"
            elif is_compatible_bin "${target_path}/bin/${bin}"; then
                cp -f "${target_path}/bin/${bin}" "${backup_dir}/${bin}"
            elif is_compatible_bin "${source_path}/bin/${bin}"; then
                cp -f "${source_path}/bin/${bin}" "${backup_dir}/${bin}"
            else
                log_warn "No compatible pre-compiled binary found for '${bin}' on this system ($(uname -m))."
                missing_compatible=true
            fi
        done

        if [[ "${missing_compatible}" == "true" ]]; then
            log_error "Cannot skip Rust compilation with --norust because compatible pre-compiled binaries were not found for architecture '$(uname -m)'."
            log_info "Please run the installer without --norust to compile binaries for $(uname -m)."
            rm -rf "${backup_dir}"
            exit 1
        else
            log_info "Compatible binaries found for $(uname -m). Skipping Rust compilation."
        fi
    fi

    # --- Determine Install Mode ---
    if [[ "${force_install}" == "true" ]]; then
        log_warn "Force install enabled. Overwriting user configurations..."
        export INSTALL_MODE="FRESH"
    elif [[ "${update_mode}" == "true" ]]; then
        log_info "Update mode explicitly requested. Preserving user configurations."
        export INSTALL_MODE="UPDATE"
    elif [[ -d "${target_path}" ]] && [[ -d "${HOME}/.config/hypr" ]]; then
        log_info "Existing installation detected but no arguments passed. Defaulting to Update Install and preserving user configurations."
        export INSTALL_MODE="UPDATE"
    else
        log_info "Fresh installation detected."
        export INSTALL_MODE="FRESH"
    fi

    # Pass target theme to environment if set
    if [[ -n "${target_theme}" ]]; then
        export TARGET_THEME="${target_theme}"
    fi

    # --- Sync Repo ---
    if [[ "${source_path}" == "${target_path}" ]]; then
        log_info "Running from installation directory. Skipping sync."
        if [[ -n "${backup_dir}" ]]; then
            rm -rf "${backup_dir}"
        fi
    else
        print_header "Synchronizing Core System"

        if [[ "${force_install}" == "true" ]]; then
            log_warn "Force mode: Cleaning existing installation and state..."
            # Remove everything except the directory itself to avoid issues if it's a mount point
            find "${target_path}" -mindepth 1 -delete || true

            # Clear persistent state and current theme to ensure a clean slate
            rm -rf "${HOME}/.local/state/hypr" || true
            rm -f "${HOME}/.config/hypr/current_theme_path" || true
        fi

        local settings_backup=""
        if [[ "${force_install}" != "true" ]] && [[ -f "${target_path}/templates/hypr/settings.yaml" ]]; then
            log_info "Preserving existing user settings.yaml..."
            settings_backup=$(mktemp)
            cp "${target_path}/templates/hypr/settings.yaml" "${settings_backup}"
        fi

        local themes_backup_dir=""
        if [[ "${force_install}" != "true" ]] && [[ -d "${target_path}/themes" ]]; then
            log_info "Preserving existing user theme configurations..."
            themes_backup_dir=$(mktemp -d)
            cp -rf "${target_path}/themes/." "${themes_backup_dir}/"
        fi

        log_info "Synchronizing installation from '${source_path}' to '${target_path}'..."

        # In FRESH or force mode, we copy over system files.
        log_info "Copying system files..."
        if command -v rsync >/dev/null 2>&1; then
            rsync -a --exclude='.git/' --exclude='target/' --exclude='*.bak' "${source_path}/" "${target_path}/"
        else
            find "${source_path}" -mindepth 1 -maxdepth 1 ! -name '.git' ! -name 'target' ! -name '*.bak' -exec cp -rf {} "${target_path}/" \;
        fi

        # If skip_rust is true, restore the compatible pre-compiled/pre-existing release binaries
        if [[ "${skip_rust}" == "true" ]] && [[ -n "${backup_dir}" ]]; then
            log_info "Rust compilation skipped. Restoring compatible pre-compiled/pre-existing release binaries..."
            mkdir -p "${target_path}/bin"
            for bin in "${rust_binaries[@]}"; do
                if [[ -f "${backup_dir}/${bin}" ]]; then
                    cp -f "${backup_dir}/${bin}" "${target_path}/bin/${bin}"
                fi
            done
            rm -rf "${backup_dir}"
        fi

        if [[ -n "${settings_backup}" ]]; then
            log_info "Restoring preserved user settings.yaml..."
            mkdir -p "${target_path}/templates/hypr"
            cp "${settings_backup}" "${target_path}/templates/hypr/settings.yaml"
            rm -f "${settings_backup}"
        fi

        if [[ -n "${themes_backup_dir}" ]]; then
            log_info "Restoring preserved user theme configurations..."
            cp -rf "${themes_backup_dir}/." "${target_path}/themes/"
            rm -rf "${themes_backup_dir}"
        fi

        # Cleanup unwanted copied files
        rm -rf "${target_path}/.git" || true
        rm -rf "${target_path}/.git*" || true
        rm -f "${target_path}/install.sh" || true
        find "${target_path}" -name "*.bak" -type f -delete || true
    fi

    # Update HYPR_PATH to the installed location
    export HYPR_PATH="${target_path}"
    export PATH="${HYPR_PATH}/bin:${PATH}"


    if [[ "${packages_only}" == "true" ]]; then
        source "${HYPR_INSTALL}/packages.sh"
        source "${HYPR_INSTALL}/packaging/fonts.sh"
        source "${HYPR_INSTALL}/packaging/webapps.sh"
        source "${HYPR_INSTALL}/packaging/tuis.sh"
        source "${HYPR_INSTALL}/packaging/custom_binaries.sh"
        if [[ "${skip_rust}" == "false" ]]; then
            source "${HYPR_INSTALL}/packaging/rust_apps.sh"
        fi
        source "${HYPR_INSTALL}/packaging/binaries.sh"

        # Verify that everything critical was installed
        source "${HYPR_INSTALL}/preflight/verify_install.sh"

        log_success "Package installation complete."
        exit 0
    fi

    source "${HYPR_INSTALL}/packages.sh"
    source "${HYPR_INSTALL}/packaging/fonts.sh"
    source "${HYPR_INSTALL}/packaging/webapps.sh"
    source "${HYPR_INSTALL}/packaging/tuis.sh"
    source "${HYPR_INSTALL}/packaging/custom_binaries.sh"
    if [[ "${skip_rust}" == "false" ]]; then
        source "${HYPR_INSTALL}/packaging/rust_apps.sh"
    fi
    source "${HYPR_INSTALL}/packaging/binaries.sh"

    # Verify that everything critical was installed before proceeding to configuration
    source "${HYPR_INSTALL}/preflight/verify_install.sh"

    source "${HYPR_INSTALL}/config/config.sh"
    source "${HYPR_INSTALL}/config/hardware.sh"
    source "${HYPR_INSTALL}/packaging/lazyvim.sh"
    source "${HYPR_INSTALL}/config/theme.sh"
    source "${HYPR_INSTALL}/config/sudoless-browser-policy.sh"
    source "${HYPR_INSTALL}/config/timezones.sh"
    source "${HYPR_INSTALL}/config/branding.sh"
    source "${HYPR_INSTALL}/config/git.sh"
    source "${HYPR_INSTALL}/config/gpg.sh"
    source "${HYPR_INSTALL}/config/xcompose.sh"
    source "${HYPR_INSTALL}/config/mise-ruby.sh"
    source "${HYPR_INSTALL}/config/docker.sh"
    source "${HYPR_INSTALL}/config/mimetypes.sh"
    source "${HYPR_INSTALL}/config/localdb.sh"


    print_header "Installation Complete"
    log_success "Installation/Update scripts finished successfully."
    if [[ "${INSTALL_MODE}" == "UPDATE" ]]; then
        log_info "Your theme and configurations were preserved."
    else
        log_info "Welcome to your new Hyprland environment!"
    fi

    log_info "Reloading Hyprland and applications..."
    if command -v hyprctl >/dev/null 2>&1; then
        hyprctl reload >/dev/null 2>&1 || true
    fi

    # Ensure Waybar is started or reloaded:
    # If the process isn't running start it, if it is reload.
    if [[ -x "${HYPR_PATH}/bin/hypr-restart-waybar" ]]; then
        "${HYPR_PATH}/bin/hypr-restart-waybar" >/dev/null 2>&1 || true
    fi

    # Ensure Mako is started or reloaded
    if [[ -x "${HYPR_PATH}/bin/hypr-restart-mako" ]]; then
        "${HYPR_PATH}/bin/hypr-restart-mako" >/dev/null 2>&1 || true
    fi

    # Ensure Walker is started or reloaded
    if [[ -x "${HYPR_PATH}/bin/hypr-restart-walker" ]]; then
        "${HYPR_PATH}/bin/hypr-restart-walker" >/dev/null 2>&1 || true
    fi

    # Refresh other app-specific themes (VSCode, Obsidian, Browser, Keyboard backlights)
    if [[ -x "${HYPR_PATH}/bin/hypr-theme-refresh" ]]; then
        "${HYPR_PATH}/bin/hypr-theme-refresh" >/dev/null 2>&1 || true
    fi

    log_success "Done! Settings have been reloaded."
}

main "$@"
