#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -eE

HYPR_PATH="$HOME/.local/share/hypr"
HYPR_INSTALL="$HYPR_PATH/install"
export PATH="$HYPR_PATH/bin:$PATH"

# Packaging
#source $HYPR_INSTALL/packages.sh
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
source $HYPR_INSTALL/config/xcompose.sh
source $HYPR_INSTALL/config/mise-ruby.sh
source $HYPR_INSTALL/config/docker.sh
source $HYPR_INSTALL/config/mimetypes.sh
source $HYPR_INSTALL/config/localdb.sh
