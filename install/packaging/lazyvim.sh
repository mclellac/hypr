#!/bin/bash

NVIM_CONFIG_DIR="$HOME/.config/nvim"
HYPR_NVIM_CONFIG_DIR="$HYPR_PATH/config/nvim"

# If nvim config doesn't exist, clone LazyVim starter
if [[ ! -d "$NVIM_CONFIG_DIR" ]]; then
  echo "Cloning LazyVim starter configuration..."
  git clone https://github.com/LazyVim/starter "$NVIM_CONFIG_DIR"
  rm -rf "$NVIM_CONFIG_DIR/.git"
fi

# Ensure the 'user' directory exists for custom configs
mkdir -p "$NVIM_CONFIG_DIR/lua/user"

# Remove the theme file if it exists, as it's managed by theme.sh
# and can cause issues on re-runs.
rm -f "$NVIM_CONFIG_DIR/lua/plugins/theme.lua"

# Copy hypr's base nvim configuration, but don't overwrite existing files
echo "Copying hypr nvim configuration..."
cp -n -R "$HYPR_NVIM_CONFIG_DIR/"* "$NVIM_CONFIG_DIR/"

# Create a placeholder for the colorscheme file that will be managed by hypr-theme-set
# This ensures that a colorscheme is set, even before the user picks a theme.
if [[ ! -f "$NVIM_CONFIG_DIR/lua/user/colorscheme.lua" ]]; then
  echo "Creating default colorscheme file..."
  cat << EOF > "$NVIM_CONFIG_DIR/lua/user/colorscheme.lua"
return {
  {
    "LazyVim/LazyVim",
    opts = {
      colorscheme = "tokyonight",
    },
  },
}
EOF
fi


# Ensure relative numbers are disabled by default as per original script
if ! grep -q "vim.opt.relativenumber = false" "$NVIM_CONFIG_DIR/lua/config/options.lua"; then
  echo "Disabling relative numbers in Neovim..."
  echo "vim.opt.relativenumber = false" >>"$NVIM_CONFIG_DIR/lua/config/options.lua"
fi

echo "Neovim configuration setup complete."