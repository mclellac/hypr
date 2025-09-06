echo "Adding hypr version info to fastfetch"
if ! grep -q "hypr" ~/.config/fastfetch/config.jsonc; then
  cp ~/.local/share/hypr/config/fastfetch/config.jsonc ~/.config/fastfetch/
fi

