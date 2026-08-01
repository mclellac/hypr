#!/bin/bash
#
# Installs custom binaries that are not available in official repos or AUR.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

# --- Source Utils ---
if [[ -n "${HYPR_INSTALL:-}" ]]; then
    source "${HYPR_INSTALL}/utils.sh"
else
    source "$(dirname "$0")/../utils.sh"
fi

install_asdcontrol() {
    if command -v asdcontrol &>/dev/null; then
        log_info "asdcontrol already installed."
        return 0
    fi

    log_info "Building and installing asdcontrol from source..."
    local temp_dir
    temp_dir=$(mktemp -d)
    git clone https://github.com/nikosdion/asdcontrol.git "${temp_dir}"
    pushd "${temp_dir}" >/dev/null
    make
    sudo make install
    popd >/dev/null
    rm -rf "${temp_dir}"
    log_success "asdcontrol installed."
}

install_qmk_hid() {
    if command -v qmk_hid &>/dev/null; then
        log_info "qmk_hid already installed."
        return 0
    fi

    if ! command -v cargo &>/dev/null; then
        log_warn "Cargo not found. Skipping qmk_hid installation."
        return 0
    fi

    log_info "Installing qmk_hid via cargo from Framework git repo..."
    # Framework's tool is in a git repo, not crates.io
    cargo install --git https://github.com/FrameworkComputer/qmk_hid
    
    # Ensure ~/.cargo/bin is in PATH for the rest of the script if needed
    export PATH="${HOME}/.cargo/bin:${PATH}"
    log_success "qmk_hid installed."
}

main() {
    if ! hypr-hw-apple && ! hypr-hw-framework; then
        return 0
    fi

    print_header "Installing Custom Binaries"

    # Check for build dependencies
    if ! command -v make &>/dev/null || ! command -v gcc &>/dev/null; then
        log_info "Installing build dependencies..."
        sudo pacman -S --needed --noconfirm base-devel systemd pkgconf
    fi

    if hypr-hw-apple; then
        install_asdcontrol
    fi
    if hypr-hw-framework; then
        install_qmk_hid
    fi
}

main "$@"
