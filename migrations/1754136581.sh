echo "Start screensaver automatically after 1 minute and stop before locking"

if ! grep -q "hypr-launch-screensaver" ~/.config/hypr/hypridle.conf; then
  hypr-refresh-hypridle
  hypr-refresh-hyprlock
fi
