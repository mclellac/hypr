#!/bin/bash
#
# Configures Plymouth for various alternative bootloaders (systemd-boot, GRUB, UKI)
# if the primary 'limine' bootloader is not detected.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Adds the 'plymouth' hook to the mkinitcpio.conf file if it's not already present.
#######################################
add_plymouth_hook() {
  if grep -Eq '^HOOKS=.*plymouth' /etc/mkinitcpio.conf; then
    echo "Plymouth hook already present in mkinitcpio.conf. Skipping."
    return
  fi

  echo "Adding Plymouth hook to mkinitcpio.conf..."
  sudo cp /etc/mkinitcpio.conf "/etc/mkinitcpio.conf.bak.$(date +%Y%m%d%H%M%S)"

  # Add plymouth hook after 'base udev' or 'base systemd'.
  if grep "^HOOKS=" /etc/mkinitcpio.conf | grep -q "base systemd"; then
    sudo sed -i '/^HOOKS=/s/base systemd/base systemd plymouth/' /etc/mkinitcpio.conf
  elif grep "^HOOKS=" /etc/mkinitcpio.conf | grep -q "base udev"; then
    sudo sed -i '/^HOOKS=/s/base udev/base udev plymouth/' /etc/mkinitcpio.conf
  else
    echo "Error: Could not find a suitable place to add the Plymouth hook."
    return 1
  fi

  echo "Regenerating initramfs..."
  sudo mkinitcpio -P
}

#######################################
# Adds splash screen kernel parameters for systemd-boot.
#######################################
configure_systemd_boot() {
  echo "Detected systemd-boot. Configuring kernel parameters..."
  local entry
  for entry in /boot/loader/entries/*.conf; do
    if [[ -f "${entry}" && ! "$(basename "${entry}")" == *"fallback"* ]]; then
      if ! grep -q "splash" "${entry}"; then
        sudo sed -i '/^options/ s/$/ splash quiet/' "${entry}"
        echo "Updated: $(basename "${entry}")"
      else
        echo "Skipped: $(basename "${entry}") (splash already present)"
      fi
    fi
  done
}

#######################################
# Adds splash screen kernel parameters for GRUB.
#######################################
configure_grub() {
  echo "Detected GRUB. Configuring kernel parameters..."
  if grep -q "GRUB_CMDLINE_LINUX_DEFAULT.*splash" /etc/default/grub; then
    echo "GRUB already configured with splash kernel parameters. Skipping."
    return
  fi

  sudo cp /etc/default/grub "/etc/default/grub.bak.$(date +%Y%m%d%H%M%S)"

  local current_cmdline
  current_cmdline=$(grep "^GRUB_CMDLINE_LINUX_DEFAULT=" /etc/default/grub | cut -d'"' -f2)

  local new_cmdline="${current_cmdline} splash quiet"
  # Remove duplicate words and trim whitespace
  new_cmdline=$(echo "${new_cmdline}" | tr ' ' '\n' | sort -u | tr '\n' ' ' | xargs)

  sudo sed -i "s/^GRUB_CMDLINE_LINUX_DEFAULT=\".*\"/GRUB_CMDLINE_LINUX_DEFAULT=\"${new_cmdline}\"/" /etc/default/grub

  echo "Regenerating GRUB configuration..."
  sudo grub-mkconfig -o /boot/grub/grub.cfg
}

#######################################
# Adds splash screen kernel parameters for a Unified Kernel Image (UKI) setup.
#######################################
configure_uki() {
  echo "Detected a UKI setup. Configuring kernel parameters..."
  if [ -d "/etc/cmdline.d" ]; then
    if ! grep -qr "splash" /etc/cmdline.d/; then
      echo "splash" | sudo tee /etc/cmdline.d/hypr-splash.conf >/dev/null
    fi
    if ! grep -qr "quiet" /etc/cmdline.d/; then
      echo "quiet" | sudo tee /etc/cmdline.d/hypr-quiet.conf >/dev/null
    fi
  elif [ -f "/etc/kernel/cmdline" ]; then
    sudo cp /etc/kernel/cmdline "/etc/kernel/cmdline.bak.$(date +%Y%m%d%H%M%S)"
    local current_cmdline
    current_cmdline=$(cat /etc/kernel/cmdline)
    local new_cmdline="${current_cmdline} splash quiet"
    new_cmdline=$(echo "${new_cmdline}" | tr ' ' '\n' | sort -u | tr '\n' ' ' | xargs)
    echo "${new_cmdline}" | sudo tee /etc/kernel/cmdline >/dev/null
  fi
}

#######################################
# Main function to detect and configure the bootloader for Plymouth.
#######################################
main() {
  if command -v limine &>/dev/null; then
    echo "Limine bootloader detected. Skipping alternative bootloader configuration."
    exit 0
  fi

  add_plymouth_hook

  if [ -d "/boot/loader/entries" ]; then
    configure_systemd_boot
  elif [ -f "/etc/default/grub" ]; then
    configure_grub
  elif [ -d "/etc/cmdline.d" ] || [ -f "/etc/kernel/cmdline" ]; then
    configure_uki
  else
    echo ""
    echo "WARNING: No supported bootloader (systemd-boot, GRUB, or UKI) was detected."
    echo "Please manually add the following kernel parameters to your bootloader configuration:"
    echo "  - splash (to see the graphical splash screen)"
    echo "  - quiet (for a silent boot)"
    echo ""
  fi
}

main "$@"
