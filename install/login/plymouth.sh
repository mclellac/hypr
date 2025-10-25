#!/bin/bash
#
# Configures Plymouth for a seamless boot splash and auto-login experience.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Sets the default Plymouth theme to 'hypr'.
# Globals:
#   HOME (read-only)
#######################################
set_plymouth_theme() {
  if [ "$(plymouth-set-default-theme)" != "hypr" ]; then
    echo "Setting Plymouth theme to hypr..."
    sudo cp -r "${HOME}/.local/share/hypr/default/plymouth" /usr/share/plymouth/themes/hypr/
    sudo plymouth-set-default-theme -R hypr
  else
    echo "Plymouth theme already set to hypr. Skipping."
  fi
}

#######################################
# Compiles and installs the seamless-login helper utility.
# This C program manages the virtual terminal to prevent console text from
# appearing between the boot splash and the desktop.
#######################################
compile_and_install_seamless_login() {
  if [ -x /usr/local/bin/seamless-login ]; then
    echo "seamless-login utility already installed. Skipping compilation."
    return
  fi

  echo "Compiling and installing seamless-login utility..."
  local -r temp_c_file="/tmp/seamless-login.c"
  local -r temp_executable="/tmp/seamless-login"

  # Write the C source code to a temporary file.
  cat >"${temp_c_file}" <<'CCODE'
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/ioctl.h>
#include <linux/kd.h>
#include <linux/vt.h>
#include <sys/wait.h>
#include <string.h>

int main(int argc, char *argv[]) {
    int vt_fd;
    int vt_num = 1; // TTY1
    char vt_path[32];
    
    if (argc < 2) {
        fprintf(stderr, "Usage: %s <session_command>\n", argv[0]);
        return 1;
    }
    
    snprintf(vt_path, sizeof(vt_path), "/dev/tty%d", vt_num);
    vt_fd = open(vt_path, O_RDWR);
    if (vt_fd < 0) {
        perror("Failed to open VT");
        return 1;
    }
    
    if (ioctl(vt_fd, VT_ACTIVATE, vt_num) < 0) {
        perror("VT_ACTIVATE failed");
        close(vt_fd);
        return 1;
    }
    
    if (ioctl(vt_fd, VT_WAITACTIVE, vt_num) < 0) {
        perror("VT_WAITACTIVE failed");
        close(vt_fd);
        return 1;
    }
    
    if (ioctl(vt_fd, KDSETMODE, KD_GRAPHICS) < 0) {
        perror("KDSETMODE KD_GRAPHICS failed");
        close(vt_fd);
        return 1;
    }
    
    const char *clear_seq = "\33[H\33[2J";
    if (write(vt_fd, clear_seq, strlen(clear_seq)) < 0) {
        perror("Failed to clear VT");
    }
    
    close(vt_fd);
    
    const char *home = getenv("HOME");
    if (home) chdir(home);
    
    execvp(argv[1], &argv[1]);
    perror("Failed to exec session");
    return 1;
}
CCODE

  # Compile the C code.
  gcc -o "${temp_executable}" "${temp_c_file}"

  # Install the compiled program.
  sudo mv "${temp_executable}" /usr/local/bin/seamless-login
  sudo chmod +x /usr/local/bin/seamless-login

  # Clean up the temporary source file.
  rm "${temp_c_file}"
}

#######################################
# Creates the systemd service file for the seamless login process.
# Globals:
#   USER (read-only)
#######################################
create_seamless_login_service() {
  local -r service_file="/etc/systemd/system/hypr-seamless-login.service"
  if [ -f "${service_file}" ]; then
    echo "seamless-login systemd service already exists. Skipping."
    return
  fi

  echo "Creating seamless-login systemd service..."
  sudo tee "${service_file}" >/dev/null <<EOF
[Unit]
Description=hypr Seamless Auto-Login
Documentation=https://github.com/basecamp/hypr
Conflicts=getty@tty1.service
After=systemd-user-sessions.service getty@tty1.service plymouth-quit.service systemd-logind.service
PartOf=graphical.target

[Service]
Type=simple
ExecStart=/usr/local/bin/seamless-login uwsm start -- hyprland.desktop
Restart=always
RestartSec=2
StartLimitIntervalSec=30
StartLimitBurst=2
User=${USER}
TTYPath=/dev/tty1
TTYReset=yes
TTYVHangup=yes
TTYVTDisallocate=yes
StandardInput=tty
StandardOutput=journal
StandardError=journal+console
PAMName=login

[Install]
WantedBy=graphical.target
EOF
}

#######################################
# Configures Plymouth services to wait for the graphical target.
#######################################
configure_plymouth_services() {
  local -r override_dir="/etc/systemd/system/plymouth-quit.service.d"
  local -r override_file="${override_dir}/wait-for-graphical.conf"

  if [ ! -f "${override_file}" ]; then
    echo "Configuring plymouth-quit service..."
    sudo mkdir -p "${override_dir}"
    sudo tee "${override_file}" >/dev/null <<'EOF'
[Unit]
After=multi-user.target
EOF
  fi

  if ! systemctl is-enabled plymouth-quit-wait.service | grep -q "masked"; then
    echo "Masking plymouth-quit-wait service..."
    sudo systemctl mask plymouth-quit-wait.service
    sudo systemctl daemon-reload
  fi
}

#######################################
# Enables and disables the necessary systemd services for seamless login.
#######################################
manage_systemd_services() {
  if ! systemctl is-enabled hypr-seamless-login.service | grep -q "enabled"; then
    echo "Enabling hypr-seamless-login service..."
    sudo systemctl enable hypr-seamless-login.service
  fi

  if ! systemctl is-enabled getty@tty1.service | grep -q "disabled"; then
    echo "Disabling getty@tty1 service..."
    sudo systemctl disable getty@tty1.service
  fi
}

#######################################
# Main function to orchestrate the Plymouth and seamless login setup.
#######################################
main() {
  set_plymouth_theme
  compile_and_install_seamless_login
  create_seamless_login_service
  configure_plymouth_services
  manage_systemd_services
  echo "Plymouth and seamless login configuration complete."
}

main "$@"
