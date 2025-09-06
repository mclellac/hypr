echo "Move hypr Package Repository after Arch core/extra/multilib and remove AUR"

sudo cp /etc/pacman.conf /etc/pacman.conf.bak
sudo sed -i '/\[hypr\]/,+2 d' /etc/pacman.conf
sudo sed -i '/\[chaotic-aur\]/,+2 d' /etc/pacman.conf
sudo bash -c 'echo -e "\n[hypr]\nSigLevel = Optional TrustAll\nServer = https://pkgs.hypr.org/\$arch" >> /etc/pacman.conf'
sudo pacman -Syu --noconfirm
