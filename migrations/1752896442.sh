echo "Replace volume control GUI with a TUI"

if hypr-cmd-missing wiremix; then
  hypr-pkg-add wiremix
  hypr-pkg-drop pavucontrol
  hypr-refresh-applications
  hypr-refresh-waybar
fi
