#!/bin/bash
#
# Compiles and installs the Rust-based Hypr Apps.

set -euo pipefail

# --- Source Utils ---
if [[ -n "${HYPR_INSTALL:-}" ]]; then
    source "${HYPR_INSTALL}/utils.sh"
else
    source "$(dirname "$0")/../utils.sh"
fi

main() {
    if ! has_command cargo; then
        log_error "Rust/Cargo not found. Cannot build Rust apps."
        exit 1
    fi

    print_header "Building Rust Applications via Cargo Workspace"

    pushd "${HYPR_PATH}" >/dev/null
    
    log_info "Compiling all workspace members in release mode..."
    if ! cargo build --release; then
        log_error "Failed to build workspace applications."
        popd >/dev/null
        exit 1
    fi

    find_rust_bin() {
        local name="$1"
        local candidates=(
            "target/release/${name}-bin"
            "target/release/${name}"
            "hypr_apps/${name}/target/release/${name}-bin"
            "hypr_apps/${name}/target/release/${name}"
            "hypr_config_manager/target/release/${name}-bin"
            "hypr_config_manager/target/release/${name}"
        )
        for c in "${candidates[@]}"; do
            if [[ -f "${c}" ]]; then
                echo "${c}"
                return 0
            fi
        done
        return 1
    }

    local pref_bin kb_bin tpl_bin get_bin
    pref_bin=$(find_rust_bin "hypr-preferences-app" || find_rust_bin "hypr-preferences-app-bin" || echo "")
    kb_bin=$(find_rust_bin "keybindings_editor" || find_rust_bin "keybindings_editor-bin" || echo "")
    tpl_bin=$(find_rust_bin "hypr-theme-templater" || find_rust_bin "hypr-theme-templater-bin" || echo "")
    get_bin=$(find_rust_bin "hypr-theme-get" || find_rust_bin "hypr-theme-get-bin" || echo "")

    if [[ -z "${pref_bin}" ]]; then
        log_error "hypr-preferences-app binary not found in workspace target."
        popd >/dev/null
        exit 1
    fi
    if [[ -z "${kb_bin}" ]]; then
        log_error "keybindings_editor binary not found in workspace target."
        popd >/dev/null
        exit 1
    fi
    if [[ -z "${tpl_bin}" ]]; then
        log_error "hypr-theme-templater-bin binary not found in workspace target."
        popd >/dev/null
        exit 1
    fi
    if [[ -z "${get_bin}" ]]; then
        log_error "hypr-theme-get-bin binary not found in workspace target."
        popd >/dev/null
        exit 1
    fi

    local target_dirs=("${HYPR_PATH}/bin")

    for dest_dir in "${target_dirs[@]}"; do
        mkdir -p "${dest_dir}"
        cp -f "${pref_bin}" "${dest_dir}/hypr-preferences-app-bin"
        cp -f "${kb_bin}" "${dest_dir}/keybindings_editor-bin"
        cp -f "${tpl_bin}" "${dest_dir}/hypr-theme-templater-bin"
        cp -f "${get_bin}" "${dest_dir}/hypr-theme-get-bin"
    done

    popd >/dev/null
    log_success "Rust applications built and installed successfully via workspace."
}

main "$@"
