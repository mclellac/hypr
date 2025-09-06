echo "Add new hypr Menu icon to Waybar"

mkdir -p ~/.local/share/fonts
cp ~/.local/share/hypr/config/hypr.ttf ~/.local/share/fonts/
fc-cache

echo
gum confirm "Replace current Waybar config (backup will be made)?" && hypr-refresh-waybar
