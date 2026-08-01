#!/bin/bash
#
# Utility functions for the Hyprland installation scripts.

set -euo pipefail

# --- Logging ---

log_info() {
    echo -e "\e[34m[INFO]\e[0m $*"
}

log_success() {
    echo -e "\e[32m[OK]\e[0m $*"
}

log_warn() {
    echo -e "\e[33m[WARN]\e[0m $*"
}

log_error() {
    echo -e "\e[31m[ERROR]\e[0m $*" >&2
}

# --- Command Helpers ---

has_command() {
    # We use -v to check if the command exists in PATH.
    # This is more portable and robust than 'which'.
    command -v "$1" >/dev/null 2>&1
}

check_requirement() {
    local cmd="$1"
    local pkg="${2:-$cmd}"
    if ! has_command "${cmd}"; then
        log_error "Required command '${cmd}' is missing. Please ensure '${pkg}' is installed."
        return 1
    fi
    return 0
}

run_as_root() {
    if [[ $EUID -eq 0 ]]; then
        "$@"
    else
        sudo "$@"
    fi
}

# --- System Detection ---

check_os() {
    if [[ -f /etc/os-release ]]; then
        # shellcheck disable=SC1091
        source /etc/os-release
        echo "${ID}"
    else
        echo "unknown"
    fi
}

is_arch_based() {
    local os_id
    os_id=$(check_os)
    if [[ "${os_id}" == "arch" ]] || [[ "${os_id}" == "manjaro" ]] || [[ "${os_id}" == "endeavouros" ]] || [[ "${os_id}" == "garuda" ]]; then
        return 0
    fi
    # Check ID_LIKE
    if grep -q "arch" /etc/os-release 2>/dev/null; then
        return 0
    fi
    return 1
}

#######################################
# Prints a centered header in a box.
# Arguments:
#   $1 - The text to print in the header.
#######################################
print_header() {
    local text="$1"
    # Use ANSI color codes
    local cyan='\033[0;36m'
    local green='\033[0;32m'
    local nc='\033[0m' # No Color

    # Box width 80
    # Content width 78 (excluding borders)

    echo ""
    echo -e "${cyan}┌──────────────────────────────────────────────────────────────────────────────┐${nc}"

    # Calculate padding for centering
    local text_len=${#text}
    local total_width=78
    local padding
    padding=$(((total_width - text_len) / 2))

    # Create padding string
    local pad_str=""
    for ((i = 0; i < padding; i++)); do pad_str="${pad_str} "; done

    # Create empty line string (78 spaces)
    local empty_line=""
    for ((i = 0; i < total_width; i++)); do empty_line="${empty_line} "; done

    # Adjust for odd length
    local extra_space=""
    if (((total_width - text_len) % 2 != 0)); then extra_space=" "; fi

    echo -e "${cyan}│${nc}${empty_line}${cyan}│${nc}"
    echo -e "${cyan}│${nc}${pad_str}${green}${text}${nc}${pad_str}${extra_space}${cyan}│${nc}"
    echo -e "${cyan}│${nc}${empty_line}${cyan}│${nc}"
    echo -e "${cyan}└──────────────────────────────────────────────────────────────────────────────┘${nc}"
    echo ""
}

# --- State Detection ---

is_fresh_install() {
    if [[ ! -d "${HOME}/.local/share/hypr" ]] || [[ ! -d "${HOME}/.config/hypr" ]]; then
        return 0 # True
    else
        return 1 # False
    fi
}

# --- Service Management ---

#######################################
# Safely enables and starts a systemd service.
# Only attempts to enable if the unit has a valid [Install] section.
# Arguments:
#   $1 - The unit name (e.g., spice-vdagentd.service)
#######################################
activate_service() {
    local unit="$1"
    if systemctl list-unit-files "${unit}" &>/dev/null; then
        # Only try to enable if the unit has a valid [Install] section with keys
        if systemctl cat "${unit}" | grep -qE "^(WantedBy|RequiredBy|Alias|Also)="; then
            log_info "Enabling ${unit}..."
            run_as_root systemctl enable "${unit}"
        fi
        log_info "Starting ${unit}..."
        run_as_root systemctl start "${unit}"
    fi
}


