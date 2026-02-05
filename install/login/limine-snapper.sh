#!/bin/bash
#
# Configures the Limine bootloader with Snapper integration for bootable snapshots.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

# Source the chroot helper function.
source "${HYPR_INSTALL}/preflight/chroot.sh"

#######################################
# Creates a dedicated mkinitcpio hooks file for the btrfs-overlayfs hook.
#######################################
configure_mkinitcpio_hooks() {
  echo "Configuring mkinitcpio hooks for Limine..."
  sudo tee /etc/mkinitcpio.conf.d/hypr_hooks.conf >/dev/null <<'EOF'
HOOKS=(base udev plymouth keyboard autodetect microcode modconf kms keymap consolefont block encrypt filesystems fsck btrfs-overlayfs)
EOF
}

#######################################
# Creates the /etc/default/limine file for the update script.
#######################################
create_limine_defaults_config() {
  local -r is_efi_system="$1"
  local limine_config_path="/boot/limine/limine.conf"
  [[ "${is_efi_system}" == "true" ]] && limine_config_path="/boot/EFI/limine/limine.conf"

  # Extract the existing kernel command line to preserve it.
  local cmdline
  cmdline=$(grep "^[[:space:]]*cmdline:" "${limine_config_path}" | head -1 | sed 's/^[[:space:]]*cmdline:[[:space:]]*//')

  echo "Creating /etc/default/limine configuration..."
  sudo tee /etc/default/limine >/dev/null <<EOF
TARGET_OS_NAME="hypr"
ESP_PATH="/boot"
KERNEL_CMDLINE[default]="${cmdline}"
KERNEL_CMDLINE[default]+=" quiet splash"
ENABLE_UKI=yes
ENABLE_LIMINE_FALLBACK=yes
FIND_BOOTLOADERS=yes
BOOT_ORDER="*, *fallback, Snapshots"
MAX_SNAPSHOT_ENTRIES=5
SNAPSHOT_FORMAT_CHOICE=5
EOF

  # UKI and EFI fallback are only supported on EFI systems.
  if [[ "${is_efi_system}" != "true" ]]; then
    sudo sed -i '/^ENABLE_UKI=/d; /^ENABLE_LIMINE_FALLBACK=/d' /etc/default/limine
  fi
}

#######################################
# Creates the main limine.conf file with theming.
#######################################
create_limine_theme_config() {
  echo "Creating Limine theme configuration..."
  sudo tee /boot/limine.conf >/dev/null <<'EOF'
### Read more at config document: https://github.com/limine-bootloader/limine/blob/trunk/CONFIG.md
#timeout: 3
default_entry: 2
interface_branding: hypr Bootloader
interface_branding_color: 2
hash_mismatch_panic: no
term_background: 1a1b26
backdrop: 1a1b26
# Terminal colors (Tokyo Night palette)
term_palette: 15161e;f7768e;9ece6a;e0af68;7aa2f7;bb9af7;7dcfff;a9b1d6
term_palette_bright: 414868;f7768e;9ece6a;e0af68;7aa2f7;bb9af7;7dcfff;c0caf5
# Text colors
term_foreground: c0caf5
term_foreground_bright: c0caf5
term_background_bright: 24283b
EOF
}

#######################################
# Creates Snapper configurations if they don't exist.
# Globals:
#   HYPR_CHROOT_INSTALL (read-only)
#######################################
create_snapper_configs() {
  # Only run this if not in the initial chroot install from the ISO.
  if [[ -z "${HYPR_CHROOT_INSTALL:-}" ]]; then
    echo "Setting up Snapper configurations..."
    if ! sudo snapper list-configs 2>/dev/null | grep -q "root"; then
      sudo snapper -c root create-config /
    fi
    if ! sudo snapper list-configs 2>/dev/null | grep -q "home"; then
      sudo snapper -c home create-config /home
    fi
  fi
}

#######################################
# Tweaks the default Snapper configuration values.
#######################################
tweak_snapper_configs() {
  echo "Tweaking default Snapper configurations..."
  sudo sed -i 's/^TIMELINE_CREATE="yes"/TIMELINE_CREATE="no"/' /etc/snapper/configs/{root,home}
  sudo sed -i 's/^NUMBER_LIMIT="50"/NUMBER_LIMIT="5"/' /etc/snapper/configs/{root,home}
  sudo sed -i 's/^NUMBER_LIMIT_IMPORTANT="10"/NUMBER_LIMIT_IMPORTANT="5"/' /etc/snapper/configs/{root,home}
}

#######################################
# Creates a UEFI boot entry for the UKI to skip the bootloader menu on normal boot.
#######################################
create_efi_boot_entry() {
  echo "Creating UEFI boot entry..."
  # Do not create an entry if the BIOS is from American Megatrends, as it can cause issues.
  if cat /sys/class/dmi/id/bios_vendor 2>/dev/null | grep -qi "American Megatrends"; then
    echo "American Megatrends BIOS detected. Skipping UEFI entry creation to avoid potential issues."
    return
  fi

  local -r disk_device=$(findmnt -n -o SOURCE /boot | sed 's/p\?[0-9]*$//')
  local -r partition_number=$(findmnt -n -o SOURCE /boot | grep -o 'p\?[0-9]*$' | sed 's/^p//')
  local -r loader_path="\\EFI\\Linux\\$(cat /etc/machine-id)_linux.efi"

  sudo efibootmgr --create \
    --disk "${disk_device}" \
    --part "${partition_number}" \
    --label "hypr" \
    --loader "${loader_path}"
}

#######################################
# Main function to orchestrate the Limine and Snapper setup.
#######################################
main() {
  if ! command -v limine &>/dev/null; then
    echo "Limine bootloader not found. Skipping Limine-Snapper configuration."
    exit 0
  fi

  local is_efi="false"
  [[ -f /boot/EFI/limine/limine.conf ]] && is_efi="true"

  configure_mkinitcpio_hooks
  create_limine_defaults_config "${is_efi}"
  create_limine_theme_config

  echo "Installing Limine Snapper packages..."
  sudo pacman -S --noconfirm --needed limine-snapper-sync limine-mkinitcpio-hook

  echo "Updating Limine configuration..."
  sudo limine-update

  create_snapper_configs
  tweak_snapper_configs

  enable_systemctl_service "limine-snapper-sync.service"

  if [[ "${is_efi}" == "true" ]] && efibootmgr &>/dev/null && ! efibootmgr | grep -q "hypr"; then
    create_efi_boot_entry
  fi

  echo "Limine and Snapper configuration complete."
}

main "$@"
