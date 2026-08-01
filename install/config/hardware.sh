#!/bin/bash
#
# Hardware-specific configuration orchestration.
# Detects system hardware/environment (baremetal, VM, Apple, AMD, NVIDIA, etc.)
# and applies appropriate hardware configurations and services.

set -euo pipefail

# --- Source Utils ---
if [[ -n "${HYPR_INSTALL:-}" ]]; then
    source "${HYPR_INSTALL}/utils.sh"
else
    source "$(dirname "$0")/../utils.sh"
fi

main() {
    print_header "Applying Hardware Configuration"

    # Check if running in a virtual machine
    if hypr-hw check vm >/dev/null 2>&1; then
        log_info "Virtual machine environment detected."
        
        # Enable QEMU/guest agent integration services if packages are present
        if systemctl list-unit-files spice-vdagentd.service >/dev/null 2>&1; then
            activate_service spice-vdagentd.service
        fi
        if systemctl list-unit-files qemu-guest-agent.service >/dev/null 2>&1; then
            activate_service qemu-guest-agent.service
        fi
        
        log_info "VM hardware configuration complete."
        return 0
    fi

    # Baremetal hardware configurations
    log_info "Baremetal hardware environment detected. Applying system tweaks..."

    # Configure Network (enable iwd, disable wait-online)
    if [[ -f "${HYPR_INSTALL}/config/hardware/network.sh" ]]; then
        source "${HYPR_INSTALL}/config/hardware/network.sh"
    fi

    # Configure USB Autosuspend
    if [[ -f "${HYPR_INSTALL}/config/hardware/usb-autosuspend.sh" ]]; then
        source "${HYPR_INSTALL}/config/hardware/usb-autosuspend.sh"
    fi

    # Configure Power Button action (allow window manager binds)
    if [[ -f "${HYPR_INSTALL}/config/hardware/ignore-power-button.sh" ]]; then
        source "${HYPR_INSTALL}/config/hardware/ignore-power-button.sh"
    fi

    # Configure Bluetooth if service/adapter is present
    if systemctl list-unit-files bluetooth.service >/dev/null 2>&1; then
        if [[ -f "${HYPR_INSTALL}/config/hardware/bluetooth.sh" ]]; then
            source "${HYPR_INSTALL}/config/hardware/bluetooth.sh"
        fi
    fi

    # Configure CUPS / Printers if cups package is installed
    if systemctl list-unit-files cups.service >/dev/null 2>&1; then
        if [[ -f "${HYPR_INSTALL}/config/hardware/printer.sh" ]]; then
            source "${HYPR_INSTALL}/config/hardware/printer.sh"
        fi
    fi

    # Configure Apple Keyboard function keys and display controls if Apple hardware is detected
    if hypr-hw check apple >/dev/null 2>&1; then
        if [[ -f "${HYPR_INSTALL}/config/hardware/fix-fkeys.sh" ]]; then
            source "${HYPR_INSTALL}/config/hardware/fix-fkeys.sh"
        fi
        if [[ -f "${HYPR_INSTALL}/config/sudoless-asdcontrol.sh" ]]; then
            source "${HYPR_INSTALL}/config/sudoless-asdcontrol.sh"
        fi
    fi

    # Configure AMD audio card profile if family 17h/19h is detected
    if pactl list cards 2>/dev/null | grep -q "Family 17h/19h"; then
        if [[ -f "${HYPR_INSTALL}/config/hardware/fix-f13-amd-audio-input.sh" ]]; then
            source "${HYPR_INSTALL}/config/hardware/fix-f13-amd-audio-input.sh"
        fi
    fi

    # Configure NVIDIA drivers, KMS early loading, and environments if NVIDIA GPU is present
    if hypr-hw check nvidia >/dev/null 2>&1; then
        if [[ -f "${HYPR_INSTALL}/config/hardware/nvidia.sh" ]]; then
            log_info "NVIDIA GPU detected. Setting up NVIDIA configuration..."
            (
                export HYPR_INSTALL
                export HYPR_PATH
                source "${HYPR_INSTALL}/utils.sh"
                source "${HYPR_INSTALL}/config/hardware/nvidia.sh"
            )
        fi
    fi

    log_success "Hardware configuration complete."
}

main "$@"
