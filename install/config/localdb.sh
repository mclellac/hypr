#!/bin/bash
#
# Updates the mlocate database.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Runs updatedb to index the filesystem for the 'locate' command.
#######################################
main() {
    print_header "Refreshing System Databases"

    if ! has_command updatedb; then
        log_warn "updatedb command not found. Skipping mlocate database update."
        log_info "You may want to install 'plocate' or 'mlocate'."
        return 0
    fi

    log_info "Updating the mlocate database..."
    run_as_root updatedb

    # Enable the timer for automatic updates using the standardized activate_service helper
    if systemctl list-unit-files | grep -q "plocate-updatedb.timer"; then
        activate_service "plocate-updatedb.timer"
    elif systemctl list-unit-files | grep -q "mlocate-updatedb.timer"; then
        activate_service "mlocate-updatedb.timer"
    fi

    log_info "mlocate database update complete."
}

main "$@"
