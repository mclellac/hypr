#!/bin/bash
#
# Automates the installation and configuration of NVIDIA drivers for Hyprland
# on Arch Linux, following the official Hyprland wiki recommendations.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Determines the appropriate NVIDIA driver package based on the GPU model.
# Outputs:
#   The name of the pacman package for the NVIDIA driver.
#######################################
select_driver_package() {
  # Turing (16xx, 20xx), Ampere (30xx), Ada (40xx), and newer GPUs
  # recommend the open-source kernel modules.
  if lspci | grep -i 'nvidia' | grep -q -E "RTX [2-9][0-9]|GTX 16"; then
    echo "nvidia-open-dkms"
  else
    echo "nvidia-dkms"
  fi
}

#######################################
# Determines the appropriate kernel headers package based on the installed kernel.
# Outputs:
#   The name of the pacman package for the kernel headers.
#######################################
select_kernel_headers() {
  if pacman -Q linux-zen &>/dev/null; then
    echo "linux-zen-headers"
  elif pacman -Q linux-lts &>/dev/null; then
    echo "linux-lts-headers"
  elif pacman -Q linux-hardened &>/dev/null; then
    echo "linux-hardened-headers"
  else
    echo "linux-headers" # Default for the standard linux kernel
  fi
}

#######################################
# Enables the multilib repository in pacman.conf if it is not already enabled.
#######################################
enable_multilib_repo() {
  if ! grep -q "^\[multilib\]" /etc/pacman.conf; then
    echo "Enabling multilib repository..."
    sudo sed -i '/^#\s*\[multilib\]/,/^#\s*Include/ s/^#\s*//' /etc/pacman.conf
    echo "Refreshing package database after enabling multilib..."
    sudo pacman -Syu --noconfirm
  fi
}

#######################################
# Installs all necessary NVIDIA-related packages.
# Arguments:
#   $1: The kernel headers package name.
#   $2: The NVIDIA driver package name.
#######################################
install_packages() {
  local -r kernel_headers="$1"
  local -r nvidia_driver_package="$2"

  local -ra packages_to_install=(
    "${kernel_headers}"
    "${nvidia_driver_package}"
    "nvidia-utils"
    "lib32-nvidia-utils"
    "egl-wayland"
    "libva-nvidia-driver" # For VA-API hardware acceleration
    "qt5-wayland"
    "qt6-wayland"
  )

  echo "Installing NVIDIA packages..."
  sudo pacman -S --needed --noconfirm "${packages_to_install[@]}"
}

#######################################
# Configures modprobe for early Kernel Mode Setting (KMS).
#######################################
configure_modprobe() {
  echo "Configuring modprobe for early KMS..."
  echo "options nvidia_drm modeset=1" | sudo tee /etc/modprobe.d/nvidia.conf >/dev/null
}

#######################################
# Adds NVIDIA modules to mkinitcpio.conf for early loading.
#######################################
configure_mkinitcpio() {
  local -r mkinitcpio_conf="/etc/mkinitcpio.conf"
  local -r nvidia_modules="nvidia nvidia_modeset nvidia_uvm nvidia_drm"

  echo "Configuring mkinitcpio to load NVIDIA modules..."

  # Create a backup before modifying.
  sudo cp "${mkinitcpio_conf}" "${mkinitcpio_conf}.backup"

  # First, remove any existing nvidia modules to prevent duplication.
  sudo sed -i -E 's/ nvidia_drm//g; s/ nvidia_uvm//g; s/ nvidia_modeset//g; s/ nvidia//g;' "${mkinitcpio_conf}"

  # Then, add the required modules to the beginning of the MODULES array.
  sudo sed -i -E "s/^(MODULES=\()/\\1${nvidia_modules} /" "${mkinitcpio_conf}"

  # Clean up potential double spaces that may result from the removal.
  sudo sed -i -E 's/  +/ /g' "${mkinitcpio_conf}"

  echo "Rebuilding initramfs..."
  sudo mkinitcpio -P
}

#######################################
# Appends NVIDIA-specific environment variables to the Hyprland configuration.
#######################################
update_hyprland_config() {
  local -r hyprland_conf="${HOME}/.config/hypr/hyprland.conf"
  if [[ -f "${hyprland_conf}" ]]; then
    echo "Adding NVIDIA environment variables to hyprland.conf..."
    cat >>"${hyprland_conf}" <<'EOF'

# NVIDIA environment variables
env = NVD_BACKEND,direct
env = LIBVA_DRIVER_NAME,nvidia
env = __GLX_VENDOR_LIBRARY_NAME,nvidia
EOF
  fi
}

#######################################
# Main function to orchestrate the NVIDIA setup process.
#######################################
main() {
  # Check if an NVIDIA GPU is present.
  if ! lspci | grep -i 'nvidia' &>/dev/null; then
    echo "No NVIDIA GPU detected. Skipping NVIDIA setup."
    exit 0
  fi

  echo "NVIDIA GPU detected. Starting setup..."

  local kernel_headers
  kernel_headers=$(select_kernel_headers)

  local nvidia_driver
  nvidia_driver=$(select_driver_package)

  enable_multilib_repo
  install_packages "${kernel_headers}" "${nvidia_driver}"
  configure_modprobe
  configure_mkinitcpio
  update_hyprland_config

  echo "NVIDIA setup complete. A reboot is required for changes to take effect."
}

main "$@"
