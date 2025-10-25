#!/bin/bash
#
# Applies a fix for audio input on AMD Family 17h/19h audio cards
# by setting the correct audio profile.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Detects the AMD audio card and sets the appropriate HiFi profile.
#######################################
main() {
  echo "Checking for AMD Family 17h/19h audio card..."

  # This command chain finds the internal name of the audio card.
  local amd_audio_card
  amd_audio_card=$(pactl list cards | grep -B20 "Family 17h/19h" | grep "Name: " | awk '{print $2}')

  if [[ -n "${amd_audio_card}" ]]; then
    echo "AMD audio card found: ${amd_audio_card}. Applying audio profile fix."
    # Set the card profile to enable all inputs and outputs.
    # The '|| true' prevents the script from exiting if the profile is already set
    # or if the command otherwise fails.
    pactl set-card-profile "${amd_audio_card}" "HiFi (Mic1, Mic2, Speaker)" 2>/dev/null || true
    echo "Audio profile fix applied."
  else
    echo "No relevant AMD audio card detected. Skipping fix."
  fi
}

main "$@"
