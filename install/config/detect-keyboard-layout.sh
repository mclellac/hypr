#!/bin/bash
#
# Detects the system keyboard layout from /etc/vconsole.conf and applies it
# to the Hyprland input configuration.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

# Constants
readonly VCONSOLE_CONF="/etc/vconsole.conf"
readonly HYPRLAND_INPUT_CONF="${HOME}/.config/hypr/input.conf"

#######################################
# Extracts a specific key's value from the vconsole.conf file.
# Arguments:
#   The key to search for (e.g., "XKBLAYOUT").
# Outputs:
#   The value of the key, with quotes removed.
#######################################
get_vconsole_value() {
  local -r key="$1"
  local value
  value=$(grep "^${key}=" "${VCONSOLE_CONF}" | cut -d= -f2 | tr -d '"')
  echo "${value}"
}

#######################################
# Inserts a key-value pair into the Hyprland input configuration file.
# The setting is inserted just before the 'kb_options' line.
# Arguments:
#   The configuration key (e.g., "kb_layout").
#   The configuration value (e.g., "us").
#######################################
set_hyprland_input_setting() {
  local -r key="$1"
  local -r value="$2"

  if [[ -n "${value}" ]]; then
    # Use sed to insert the new setting before the line containing 'kb_options'.
    # The 'i' command in sed inserts a line before the matched line.
    sed -i "/^[[:space:]]*kb_options *=/i\  ${key} = ${value}" "${HYPRLAND_INPUT_CONF}"
  fi
}

#######################################
# Main function to detect and apply keyboard settings.
#######################################
main() {
  if [[ ! -f "${VCONSOLE_CONF}" ]]; then
    echo "Warning: ${VCONSOLE_CONF} not found. Skipping keyboard layout detection."
    return 0
  fi

  if [[ ! -f "${HYPRLAND_INPUT_CONF}" ]]; then
    echo "Error: ${HYPRLAND_INPUT_CONF} not found. Cannot apply keyboard settings."
    return 1
  fi

  echo "Detecting and applying keyboard layout..."
  local layout
  layout=$(get_vconsole_value "XKBLAYOUT")
  set_hyprland_input_setting "kb_layout" "${layout}"

  local variant
  variant=$(get_vconsole_value "XKBVARIANT")
  set_hyprland_input_setting "kb_variant" "${variant}"

  echo "Keyboard layout configuration updated."
}

main "$@"
