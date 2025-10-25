#!/bin/bash
#
# Configures Docker, sets up DNS, enables the service, and grants user access.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Configures the Docker daemon with logging limits and DNS settings.
#######################################
configure_daemon() {
  echo "Configuring Docker daemon..."
  sudo mkdir -p /etc/docker
  sudo tee /etc/docker/daemon.json >/dev/null <<'EOF'
{
    "log-driver": "json-file",
    "log-opts": { "max-size": "10m", "max-file": "5" },
    "dns": ["172.17.0.1"],
    "bip": "172.17.0.1/16"
}
EOF
}

#######################################
# Configures systemd-resolved to work with the Docker network.
#######################################
configure_dns() {
  echo "Configuring systemd-resolved for Docker..."
  sudo mkdir -p /etc/systemd/resolved.conf.d
  echo -e '[Resolve]\nDNSStubListenerExtra=172.17.0.1' | sudo tee /etc/systemd/resolved.conf.d/20-docker-dns.conf >/dev/null
  sudo systemctl restart systemd-resolved
}

#######################################
# Enables and configures the Docker systemd service.
#######################################
setup_service() {
  echo "Enabling and configuring Docker service..."
  # Start Docker automatically
  sudo systemctl enable docker

  # Prevent Docker from blocking the boot process
  sudo mkdir -p /etc/systemd/system/docker.service.d
  sudo tee /etc/systemd/system/docker.service.d/no-block-boot.conf >/dev/null <<'EOF'
[Unit]
DefaultDependencies=no
EOF
  sudo systemctl daemon-reload
}

#######################################
# Adds the current user to the 'docker' group.
#######################################
grant_user_access() {
  echo "Adding user ${USER} to the docker group..."
  sudo usermod -aG docker "${USER}"
}

#######################################
# Main function to orchestrate Docker configuration.
#######################################
main() {
  configure_daemon
  configure_dns
  setup_service
  grant_user_access
  echo "Docker configuration complete."
}

main "$@"
