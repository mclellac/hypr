#!/bin/bash
#
# Configures printing services, including CUPS, Avahi for discovery,
# and remote printer browsing.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

# Source the chroot helper function.
source "${HYPR_INSTALL}/preflight/chroot.sh"

#######################################
# Enables the core CUPS printing service.
#######################################
enable_cups() {
  echo "Enabling CUPS service..."
  enable_systemctl_service "cups.service"
}

#######################################
# Configures and enables Avahi for network printer discovery.
# This involves disabling MulticastDNS in systemd-resolved to prevent conflicts.
#######################################
configure_avahi() {
  echo "Configuring Avahi for network printer discovery..."

  # Disable multicast DNS in systemd-resolved to let Avahi handle it.
  sudo mkdir -p /etc/systemd/resolved.conf.d
  echo -e "[Resolve]\nMulticastDNS=no" | sudo tee /etc/systemd/resolved.conf.d/10-disable-multicast.conf >/dev/null

  enable_systemctl_service "avahi-daemon.service"
}

#######################################
# Enables automatic discovery of remote printers.
#######################################
configure_remote_printers() {
  local -r cups_browsed_conf="/etc/cups/cups-browsed.conf"
  local -r setting="CreateRemotePrinters Yes"

  echo "Enabling remote printer discovery..."

  # Add the setting if it's not already present in the file.
  if ! grep -qF "${setting}" "${cups_browsed_conf}"; then
    echo "${setting}" | sudo tee -a "${cups_browsed_conf}" >/dev/null
  fi

  enable_systemctl_service "cups-browsed.service"
}

#######################################
# Main function to orchestrate printer configuration.
#######################################
main() {
  enable_cups
  configure_avahi
  configure_remote_printers
  echo "Printer configuration complete."
}

main "$@"
