#!/bin/bash

# A safe copy function that backs up existing files or directories before copying.
#
# Arguments:
#   $1 - The source file or directory.
#   $2 - The destination directory.
BackupAndCopy() {
    local src="$1"
    local dest_dir="$2"
    local item_name
    item_name=$(basename "${src}")
    local dest_path="${dest_dir}/${item_name}"

    if [[ -e "${dest_path}" ]]; then
        local backup_path="${dest_path}.bak.$(date +%Y%m%d%H%M%S)"
        echo "INFO: '${dest_path}' already exists. Backing it up to '${backup_path}'."
        if ! mv "${dest_path}" "${backup_path}"; then
            echo "ERROR: Failed to back up '${dest_path}'. Aborting copy." >&2
            return 1
        fi
    fi

    echo "INFO: Installing '${item_name}' to '${dest_dir}'."
    if ! cp -R "${src}" "${dest_dir}"; then
        echo "ERROR: Failed to copy '${src}' to '${dest_dir}'." >&2
        return 1
    fi
}

echo "Installing hypr configurations into ~/.config..."
mkdir -p ~/.config
for item in "$HYPR_PATH"/config/*; do
    if [[ -e "$item" ]]; then
        BackupAndCopy "${item}" "${HOME}/.config"
    fi
done

echo "Installing .bashrc..."
BackupAndCopy "$HYPR_PATH/default/bashrc" "${HOME}"

echo "Configuration installation complete."
