#!/bin/bash
#
# Configures Docker, sets up DNS, enables the service, and grants user access.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Configures the Docker daemon with logging limits and DNS settings.
#######################################
configure_daemon() {
    if [[ -f /etc/docker/daemon.json ]]; then
        log_info "/etc/docker/daemon.json already exists. Skipping daemon configuration to preserve user custom settings."
        return 0
    fi
    log_info "Configuring Docker daemon..."
    run_as_root mkdir -p /etc/docker
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
    if ! systemctl is-active --quiet systemd-resolved; then
        log_info "systemd-resolved is not active. Skipping DNS configuration for Docker."
        return 0
    fi

    log_info "Configuring systemd-resolved for Docker..."
    run_as_root mkdir -p /etc/systemd/resolved.conf.d
    echo -e '[Resolve]\nDNSStubListenerExtra=172.17.0.1' | sudo tee /etc/systemd/resolved.conf.d/20-docker-dns.conf >/dev/null
    run_as_root systemctl restart systemd-resolved
}

#######################################
# Enables and configures the Docker systemd service.
#######################################
setup_service() {
    log_info "Configuring Docker service..."

    # Prevent Docker from blocking the boot process
    run_as_root mkdir -p /etc/systemd/system/docker.service.d
    sudo tee /etc/systemd/system/docker.service.d/no-block-boot.conf >/dev/null <<'EOF'
[Unit]
DefaultDependencies=no
EOF

    # Reload systemd to pick up the new drop-in and the service itself if just installed
    run_as_root systemctl daemon-reload

    log_info "Enabling and starting Docker service..."
    # Enable and start Docker automatically
    if ! run_as_root systemctl enable --now docker; then
        log_warn "Failed to enable/start docker service. You may need to reboot or start it manually."
    fi
}

#######################################
# Adds the current user to the 'docker' group.
#######################################
grant_user_access() {
    if ! getent group docker >/dev/null; then
        log_info "Creating docker group..."
        run_as_root groupadd docker || true
    fi

    log_info "Adding user ${USER} to the docker group..."
    if ! groups "${USER}" | grep -q "\bdocker\b"; then
        run_as_root usermod -aG docker "${USER}"
    else
        log_info "User ${USER} is already in the docker group."
    fi
}

#######################################
# Main function to orchestrate Docker configuration.
#######################################
main() {
    if ! has_command docker; then
        log_warn "Docker is not installed. Skipping Docker configuration."
        return 0
    fi

    print_header "Configuring Docker"
    
    grant_user_access
    configure_daemon
    configure_dns
    setup_service
    
    log_info "Docker configuration complete."
}

main "$@"
