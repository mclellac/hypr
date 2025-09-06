#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -eE

HYPR_PATH="$HOME/.local/share/hypr"
HYPR_INSTALL="$HYPR_PATH/install"
export PATH="$HYPR_PATH/bin:$PATH"

# Preparation
source $HYPR_INSTALL/preflight/show-env.sh
source $HYPR_INSTALL/preflight/trap-errors.sh
source $HYPR_INSTALL/preflight/guard.sh
source $HYPR_INSTALL/preflight/chroot.sh
source $HYPR_INSTALL/preflight/pacman.sh
source $HYPR_INSTALL/preflight/migrations.sh
source $HYPR_INSTALL/preflight/first-run-mode.sh

# Packaging
source $HYPR_INSTALL/packages.sh
source $HYPR_INSTALL/packaging/fonts.sh
source $HYPR_INSTALL/packaging/lazyvim.sh
source $HYPR_INSTALL/packaging/webapps.sh
source $HYPR_INSTALL/packaging/tuis.sh

# Configuration
source $HYPR_INSTALL/config/config.sh
source $HYPR_INSTALL/config/theme.sh
source $HYPR_INSTALL/config/branding.sh
source $HYPR_INSTALL/config/git.sh
source $HYPR_INSTALL/config/gpg.sh
source $HYPR_INSTALL/config/timezones.sh
source $HYPR_INSTALL/config/increase-sudo-tries.sh
source $HYPR_INSTALL/config/increase-lockout-limit.sh
source $HYPR_INSTALL/config/ssh-flakiness.sh
source $HYPR_INSTALL/config/detect-keyboard-layout.sh
source $HYPR_INSTALL/config/xcompose.sh
source $HYPR_INSTALL/config/mise-ruby.sh
source $HYPR_INSTALL/config/docker.sh
source $HYPR_INSTALL/config/mimetypes.sh
source $HYPR_INSTALL/config/localdb.sh
source $HYPR_INSTALL/config/sudoless-asdcontrol.sh
source $HYPR_INSTALL/config/hardware/network.sh
source $HYPR_INSTALL/config/hardware/fix-fkeys.sh
source $HYPR_INSTALL/config/hardware/bluetooth.sh
source $HYPR_INSTALL/config/hardware/printer.sh
source $HYPR_INSTALL/config/hardware/usb-autosuspend.sh
source $HYPR_INSTALL/config/hardware/ignore-power-button.sh
source $HYPR_INSTALL/config/hardware/nvidia.sh
source $HYPR_INSTALL/config/hardware/fix-f13-amd-audio-input.sh

# Login
source $HYPR_INSTALL/login/plymouth.sh
source $HYPR_INSTALL/login/limine-snapper.sh
source $HYPR_INSTALL/login/alt-bootloaders.sh

# Finishing
source $HYPR_INSTALL/reboot.sh
