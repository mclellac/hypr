#!/bin/bash
#
# Sets the power profile based on whether a battery is detected.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Detects if a battery is present and sets the power profile accordingly.
# Enables a battery monitor service if a battery is found.
#######################################
main() {
  if ls /sys/class/power_supply/BAT* &>/dev/null; then
    # This computer runs on a battery
    echo "Battery detected. Setting power profile to 'balanced'."
    powerprofilesctl set balanced || true

    # Enable battery monitoring timer for low battery notifications
    echo "Enabling battery monitor timer."
    systemctl --user enable --now hypr-battery-monitor.timer
  else
    # This computer runs on a power outlet
    echo "No battery detected. Setting power profile to 'performance'."
    powerprofilesctl set performance || true
  fi
}

main "$@"
