echo "Add hypr Package Repository"

hypr-refresh-pacman-mirrorlist

if ! grep -q "hypr" /etc/pacman.conf; then
  sudo sed -i '/^\[core\]/i [hypr]\nSigLevel = Optional TrustAll\nServer = https:\/\/pkgs.hypr.org\/$arch\n' /etc/pacman.conf
  sudo systemctl restart systemd-timesyncd
  sudo pacman -Syu --noconfirm
fi
