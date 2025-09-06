echo "Update Waybar for new hypr menu"

if ! grep -q "" ~/.config/waybar/config.jsonc; then
  hypr-refresh-waybar
fi
