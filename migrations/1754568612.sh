echo "Update Waybar config to fix path issue with update-available icon click"

if grep -q "alacritty --class hypr --title hypr -e hypr-update" ~/.config/waybar/config.jsonc; then
  sed -i 's|\("on-click": "alacritty --class hypr --title hypr -e \)hypr-update"|\1hypr-update"|' ~/.config/waybar/config.jsonc
  hypr-restart-waybar
fi
