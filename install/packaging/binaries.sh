#!/bin/bash
#
# Symlinks core binaries to ~/.local/bin for global access.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

# --- Source Utils ---
if [[ -n "${HYPR_INSTALL:-}" ]]; then
    source "${HYPR_INSTALL}/utils.sh"
else
    source "$(dirname "$0")/../utils.sh"
fi

main() {
    print_header "Installing Core Binaries"

    local -r local_bin_dir="${HOME}/.local/bin"
    mkdir -p "${local_bin_dir}"

    log_info "Symlinking binaries from '${HYPR_PATH}/bin' to '${local_bin_dir}'..."

    # Ensure ~/.local/bin is in PATH for the user, but we can't force that here.
    # We just ensure the symlinks are created.

    for bin in "${HYPR_PATH}/bin"/*; do
        if [[ -x "${bin}" ]] && [[ -f "${bin}" ]]; then
            local bin_name
            bin_name=$(basename "${bin}")
            
            # Skip non-executable files if any (already checked -x but just in case)
            if [[ "${bin_name}" == "README.md" ]] || [[ "${bin_name}" == "LICENSE" ]]; then
                continue
            fi

            # Create symlink
            ln -sf "${bin}" "${local_bin_dir}/${bin_name}"
        fi
    done

    log_success "Core binaries installed to ${local_bin_dir}."
    
    if [[ ":$PATH:" != *":${local_bin_dir}:"* ]]; then
        log_warn "${local_bin_dir} is not in your PATH. Please add it to your shell configuration."
    fi
}

main "$@"
