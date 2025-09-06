echo "Enable battery low notifications for laptops"

if ls /sys/class/power_supply/BAT* &>/dev/null && [[ ! -f ~/.local/share/hypr/config/systemd/user/hypr-battery-monitor.service ]]; then
  mkdir -p ~/.config/systemd/user

  cp ~/.local/share/hypr/config/systemd/user/hypr-battery-monitor.* ~/.config/systemd/user/

  systemctl --user daemon-reload
  systemctl --user enable --now hypr-battery-monitor.timer || true
fi
