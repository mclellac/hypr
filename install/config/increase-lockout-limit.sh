#!/bin/bash
#
# Increases the PAM faillock limit to 10 attempts and sets the unlock time to 120 seconds.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Modifies the system-auth PAM configuration to increase the lockout limit.
#######################################
main() {
  local -r system_auth_file="/etc/pam.d/system-auth"
  local -r deny_limit="10"
  local -r unlock_time_secs="120"

  echo "Increasing PAM faillock limits..."

  # Configure the pre-authentication check.
  sudo sed -i \
    "s|^\(auth\s\+required\s\+pam_faillock.so\)\s\+preauth.*$|\1 preauth silent deny=${deny_limit} unlock_time=${unlock_time_secs}|" \
    "${system_auth_file}"

  # Configure the authentication failure action.
  sudo sed -i \
    "s|^\(auth\s\+\[default=die\]\s\+pam_faillock.so\)\s\+authfail.*$|\1 authfail deny=${deny_limit} unlock_time=${unlock_time_secs}|" \
    "${system_auth_file}"

  echo "PAM faillock configuration updated."
}

main "$@"
