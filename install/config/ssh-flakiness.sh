#!/bin/bash
#
# Mitigates common SSH connection flakiness by enabling TCP MTU probing.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Appends a setting to the sysctl configuration to improve SSH stability.
#######################################
main() {
  local -r sysctl_conf="/etc/sysctl.d/99-sysctl.conf"
  local -r setting="net.ipv4.tcp_mtu_probing=1"

  echo "Applying SSH flakiness workaround..."
  echo "${setting}" | sudo tee -a "${sysctl_conf}" >/dev/null
  echo "SSH workaround applied. A reboot may be required for changes to take effect."
}

main "$@"
