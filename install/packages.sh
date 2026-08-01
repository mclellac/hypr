#!/bin/bash
#
# Installs all required system packages using yay.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

# --- Source Utils ---
if [[ -n "${HYPR_INSTALL:-}" ]]; then
    # shellcheck source=install/utils.sh
    source "${HYPR_INSTALL}/utils.sh"
else
    # shellcheck source=install/utils.sh
    source "$(dirname "$0")/utils.sh"
fi

#######################################
# Installs yay if it's not present.
#######################################
ensure_yay() {
    if has_command yay; then
        return 0
    fi

    log_info "yay not found. Installing yay from AUR..."

    # Ensure build dependencies are installed
    run_as_root pacman -S --needed --noconfirm base-devel git

    local temp_dir
    temp_dir=$(mktemp -d)
    git clone https://aur.archlinux.org/yay-bin.git "${temp_dir}"
    pushd "${temp_dir}" >/dev/null
    makepkg -si --noconfirm
    popd >/dev/null
    rm -rf "${temp_dir}"

    if ! has_command yay; then
        log_error "Failed to install yay. Please install it manually."
        exit 1
    fi
    log_success "yay installed successfully."
}

#######################################
# Installs a package if it is missing from the system.
#######################################
_install_pkg_if_missing() {
    local pkg="$1"
    if ! pacman -Q "${pkg}" &>/dev/null && ! pacman -T "${pkg}" &>/dev/null; then
        log_info "Installing missing dependency: ${pkg}..."
        if ! run_as_root pacman -S --needed --noconfirm "${pkg}"; then
            log_warn "Could not install ${pkg} via pacman. Attempting yay..."
            if ! yay -S --needed --noconfirm "${pkg}"; then
                log_error "Failed to install required dependency: ${pkg}"
                exit 1
            fi
        fi
    fi
}

#######################################
# Installs a list of packages using yay.
#######################################
main() {
    if ! is_arch_based; then
        log_error "This installer currently only supports Arch-based distributions."
        log_warn "Detected OS: $(check_os)"
        exit 1
    fi

    print_header "Installing System Packages"

    ensure_yay
    _install_pkg_if_missing tesseract
    _install_pkg_if_missing tesseract-data-eng

    log_info "Installing packages via yay..."

    # Ensure keyrings are up to date to avoid PGP signature errors
    log_info "Updating system keyrings..."
    # Initialize pacman-key if needed
    run_as_root pacman-key --init || true

    # Try updating keyrings
    if ! run_as_root pacman -Sy --needed --noconfirm archlinux-keyring; then
        log_warn "Failed to update archlinux-keyring. Attempting to refresh keys..."
        run_as_root pacman-key --populate archlinux || true
        run_as_root pacman -Sy --needed --noconfirm archlinux-keyring || true
    fi

    if grep -q "Manjaro" /etc/os-release 2>/dev/null; then
        if ! run_as_root pacman -Sy --needed --noconfirm manjaro-keyring; then
            run_as_root pacman-key --populate manjaro || true
            run_as_root pacman -Sy --needed --noconfirm manjaro-keyring || true
        fi
    fi

    # Refresh keys if we still have issues, but let's try just the update first as refresh is slow/unreliable
    # sudo pacman-key --refresh-keys || true

    # A list of packages to be installed.
    # Using generic provision names where possible to avoid conflicts (e.g. 'waybar' instead of 'waybar-git')
    local -a packages=(
        "alacritty"
        "avahi"
        "bash-completion"
        "bat"
        "blueman"
        "bluez"
        "bluez-utils"
        "brightnessctl"
        "btop"
        "cargo"
        "chromium"
        "clang"
        "cliphist"
        "cups"
        "cups-browsed"
        "cups-filters"
        "cups-pdf"
        "curl"
        "desktop-file-utils"
        "docker"
        "docker-buildx"
        "docker-compose"
        "dust"
        "elephant-archlinuxpkgs"
        "elephant"
        "elephant-bluetooth"
        "elephant-bookmarks"
        "elephant-clipboard"
        "elephant-desktopapplications"
        "elephant-files"
        "elephant-menus"
        "elephant-providerlist"
        "elephant-runner"
        "elephant-snippets"
        "elephant-unicode"
        "elephant-websearch"
        "evince"
        "eza"
        "fastfetch"
        "fcitx5"
        "fcitx5-config-qt"
        "fcitx5-gtk"
        "fcitx5-qt"
        "fd"
        "ffmpegthumbnailer"
        "fontconfig"
        "fzf"
        "ghostty"
        "git"
        "github-cli"
        "glib2-devel"
        "glow"
        "gnome-calculator"
        "gnome-keyring"
        #"gnome-themes-extra"
        "go"
        "grim"
        "gtk-layer-shell"
        "gtk4"
        "gum"
        "gvfs-mtp"
        "gvfs-smb"
        "hypridle"
        "hyprland"
        "hyprlock"
        "hyprpicker"
        "hyprshot"
        "hyprsunset"
        "hypremoji"
        "imagemagick"
        "impala"
        "imv"
        "inetutils"
        "iw"
        "iwd"
        "jq"
        "kdenlive"
        "keepassxc"
        "kitty"
        "kvantum-qt5"
        "lazydocker"
        "lazygit"
        "less"
        "libadwaita"
        "libnotify"
        "libqalculate"
        "libreoffice-fresh"
        "llvm"
        "localsend"
        "luarocks"
        "mako"
        "man-db"
        "mariadb-libs"
        "mise"
        "mpv"
        "nautilus"
        "network-manager-applet"
        "networkmanager"
        "noto-fonts"
        "noto-fonts-cjk"
        "noto-fonts-emoji"
        "noto-fonts-extra"
        "npm"
        "nss-mdns"
        "nvim"
        "obs-studio"
        "obsidian"
        "pacman-contrib"
        "pamixer"
        "pavucontrol"
        "pinta"
        "pkg-config"
        "playerctl"
        "plocate"
        "polkit-gnome"
        "postgresql-libs"
        "power-profiles-daemon"
        "protobuf"
        "python"
        "python-pip"
        "python-gobject"
        "python-jsonschema"
        "python-poetry-core"
        "python-ruamel-yaml"
        "python-setuptools"
        "qt5-wayland"
        "ripgrep"
        "satty"
        "signal-desktop"
        "slurp"
        "spotify"
        "starship"
        "sushi"
        "swaybg"
        "swayosd"
        "system-config-printer"
        "python-terminaltexteffects"
        "tldr"
        "tree-sitter-cli"
        "ttf-cascadia-mono-nerd"
        "ttf-ia-writer"
        "ttf-jetbrains-mono-nerd"
        "typora"
        "tzupdate"
        "ufw"
        "ufw-docker"
        "unzip"
        "upower"
        "uwsm"
        "walker"
        "waybar"
        "wf-recorder"
        "whois"
        "wiremix"
        "wireplumber"
        "wl-clip-persist"
        "wl-clipboard"
        "wl-screenrec"
        "woff2-font-awesome"
        "xdg-desktop-portal-gtk"
        "xdg-desktop-portal-hyprland"
        "xdg-terminal-exec"
        "xmlstarlet"
        "xournalpp"
        "xterm"
        "zoxide"
        "socat"
        "mesa-utils"
    )
    # Add ASUS-specific packages if ASUS hardware is detected

    if hypr-hw check asus 2>/dev/null || command -v asusctl &>/dev/null; then
        log_info "ASUS hardware detected. Adding asusctl and supergfxctl..."
        packages+=("asusctl" "supergfxctl")
    fi

    # Add VM-specific packages if running in a VM
    if hypr-hw check vm 2>/dev/null; then
        log_info "Virtual machine detected. Adding integration packages..."
        packages+=("spice-vdagent" "qemu-guest-agent")
    fi

    # Filter out ignored packages
    local ignored
    ignored=$(hypr-pkg-ignored 2>/dev/null || echo "")

    # Auto-ignore problematic packages on ARM (aarch64)
    if [[ "$(uname -m)" == "aarch64" ]]; then
        log_info "ARM architecture detected. Automatically ignoring known problematic or missing packages..."
        local -a arm_incompatible=("obs-studio" "spotify" "typora" "obsidian" "ghostty" "pinta" "dotnet-sdk" "dotnet-runtime" "dotnet-host" "dotnet-targeting-pack" "wl-screenrec" "hypremoji" "tzupdate" "signal-desktop")
        for pkg in "${arm_incompatible[@]}"; do
            if [[ ! ",${ignored}," =~ ,${pkg}, ]]; then
                if [[ -z "${ignored}" ]]; then
                    ignored="${pkg}"
                else
                    ignored="${ignored},${pkg}"
                fi
            fi
        done
    fi

    if [[ -n "${ignored}" ]]; then
        local -a filtered=()
        for pkg in "${packages[@]}"; do
            if [[ ! ",${ignored}," =~ ,${pkg}, ]]; then
                filtered+=("${pkg}")
            fi
        done
        packages=("${filtered[@]}")
    fi

    # Determine which packages are truly missing or unsatisfied
    log_info "Checking which packages need installation..."
    local -a missing_packages=()
    for pkg in "${packages[@]}"; do
        # 1a. Exact match or provision match (standard pacman way)
        if pacman -Q "${pkg}" &>/dev/null || pacman -T "${pkg}" &>/dev/null; then
            continue
        fi

        # 1b. Fuzzy check: If the user has a -git or -bin variant of this package
        if [[ ! "${pkg}" =~ -(bin|git)$ ]]; then
            if pacman -Q "${pkg}-bin" &>/dev/null || pacman -Q "${pkg}-git" &>/dev/null; then
                continue
            fi
        fi

        # 1c. Special case for terminaltexteffects (check owner of binary)
        if [[ "${pkg}" == "python-terminaltexteffects" ]] || [[ "${pkg}" == "terminaltexteffects" ]]; then
            if command -v tte &>/dev/null || pacman -Q tte &>/dev/null; then
                continue
            fi
        fi

        missing_packages+=("${pkg}")
    done

    # Check for updates for the ones that ARE satisfied
    local -a outdated_packages=()
    if command -v yay &>/dev/null && [[ ${#packages[@]} -gt 0 ]]; then
        log_info "Checking for package updates..."
        local all_outdated
        all_outdated=$(yay -Qu --quiet 2>/dev/null | cut -d' ' -f1 || true)
        if [[ -n "${all_outdated}" ]]; then
            local -a all_outdated_arr
            readarray -t all_outdated_arr <<<"${all_outdated}"

            # Only update packages that were explicitly in our requested list
            # and are NOT already in the missing list
            for pkg in "${packages[@]}"; do
                # Check if this package name is in the outdated list
                if [[ " ${all_outdated_arr[*]} " == *" $pkg "* ]]; then
                    # Skip if already marked for install
                    local already_missing=false
                    for mp in "${missing_packages[@]}"; do
                        [[ "$mp" == "$pkg" ]] && already_missing=true && break
                    done
                    [[ "$already_missing" == "true" ]] && continue

                    outdated_packages+=("$pkg")
                fi
            done
        fi
    fi

    # Final list to install
    local -a to_install=("${missing_packages[@]}" "${outdated_packages[@]}")

    if [[ ${#to_install[@]} -eq 0 ]]; then
        log_success "All system packages are already installed and up to date."
        return 0
    fi

    log_info "Installing/updating ${#to_install[@]} packages via yay..."
    # Note: we still use --needed just in case, though pacman -T should have filtered most.
    if ! yay -Syu --noconfirm --needed "${to_install[@]}"; then
        log_error "Failed to install some packages. Please check the output above."
        exit 1
    fi

    # User Group Management
    log_info "Checking user groups..."
    local user="${USER:-$(whoami)}"
    for group in "video" "input"; do
        if ! groups "${user}" | grep -q "\b${group}\b"; then
            log_info "Adding user '${user}' to group '${group}'..."
            run_as_root usermod -aG "${group}" "${user}" || log_warn "Failed to add user to group ${group}."
        fi
    done
}






main "$@"
