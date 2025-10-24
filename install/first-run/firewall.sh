#!/bin/bash
#
# Configures the UFW firewall with a default set of rules.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Sets the default UFW policies.
#######################################
set_default_policies() {
  echo "Setting default firewall policies..."
  sudo ufw default deny incoming
  sudo ufw default allow outgoing
}

#######################################
# Allows specific services through the firewall.
#######################################
allow_services() {
  echo "Allowing specific services..."
  # Allow ports for LocalSend
  sudo ufw allow 53317/udp
  sudo ufw allow 53317/tcp

  # Allow SSH in
  sudo ufw allow 22/tcp

  # Allow Docker containers to use DNS on host
  sudo ufw allow in proto udp from 172.16.0.0/12 to 172.17.0.1 port 53 comment 'allow-docker-dns'
}

#######################################
# Enables and reloads the firewall to apply changes.
#######################################
enable_firewall() {
  echo "Enabling the firewall..."
  # The --force option is used to enable without prompting.
  sudo ufw --force enable

  # Apply Docker protections and reload
  sudo ufw-docker install
  sudo ufw reload
}

#######################################
# Main function to orchestrate firewall configuration.
#######################################
main() {
  set_default_policies
  allow_services
  enable_firewall
  echo "Firewall configuration complete."
}

main "$@"
