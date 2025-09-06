echo "Update Waybar CSS to dim unused workspaces"

if ! grep -q "#workspaces button\.empty" ~/.config/waybar/style.css; then
  hypr-refresh-config waybar/style.css
  hypr-restart-waybar
fi
