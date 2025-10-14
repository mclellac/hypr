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
        # Don't back up symlinks, just remove them.
        if [[ -L "${dest_path}" ]]; then
            rm -f "${dest_path}"
        else
            local backup_path="${dest_path}.bak.$(date +%Y%m%d%H%M%S)"
            echo "INFO: '${dest_path}' already exists. Backing it up to '${backup_path}'."
            if ! mv "${dest_path}" "${backup_path}"; then
                echo "ERROR: Failed to back up '${dest_path}'. Aborting copy." >&2
                return 1
            fi
        fi
    fi

    # echo "INFO: Installing '${item_name}' to '${dest_dir}'."
    if ! cp -R "${src}" "${dest_dir}"; then
        echo "ERROR: Failed to copy '${src}' to '${dest_dir}'." >&2
        return 1
    fi
}

echo "Installing hypr configurations into ~/.config..."
mkdir -p ~/.config

# Loop through all items in the source config directory.
for item in "$HYPR_PATH"/config/*; do
    if [[ ! -e "$item" ]]; then
        continue
    fi

    item_name=$(basename "$item")

    # Special, non-destructive handling for the 'hypr' directory.
    if [[ "${item_name}" == "hypr" ]]; then
        echo "INFO: Updating hypr configurations in ~/.config/hypr..."
        mkdir -p "${HOME}/.config/hypr"
        # Loop through the contents of the source hypr directory.
        for sub_item in "$item"/*; do
            if [[ -e "$sub_item" ]]; then
                # Copy each file/dir individually into ~/.config/hypr
                BackupAndCopy "${sub_item}" "${HOME}/.config/hypr"
            fi
        done
    else
        # For all other configs, use the standard backup and copy.
        echo "INFO: Installing ${item_name} configuration..."
        BackupAndCopy "${item}" "${HOME}/.config"
    fi
done

echo "Installing .bashrc..."
BackupAndCopy "$HYPR_PATH/default/bashrc" "${HOME}"

echo "Configuration installation complete."
