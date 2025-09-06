echo "Add UWSM env"

export hypr_PATH="$HOME/.local/share/hypr"
export PATH="$hypr_PATH/bin:$PATH"

mkdir -p "$HOME/.config/uwsm/"
hypr-refresh-config uwsm/env

echo -e "\n\e[31mhypr bins have been added to PATH (and hypr_PATH is now system-wide).\nYou must immediately relaunch Hyprland or most hypr cmds won't work.\nPlease run hypr > Update again after the quick relaunch is complete.\e[0m"
echo

mkdir -p ~/.local/state/hypr/migrations
gum confirm "Ready to relaunch Hyprland? (All applications will be closed)" &&
  touch ~/.local/state/hypr/migrations/1751134560.sh &&
  uwsm stop
