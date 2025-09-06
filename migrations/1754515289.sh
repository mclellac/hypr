echo "Update and restart Walker to resolve stuck hypr menu"

sudo pacman -Syu --noconfirm walker-bin
hypr-restart-walker
