echo "Install Impala as new wifi selection TUI"

if hypr-cmd-missing impala; then
  hypr-pkg-add impala
  hypr-refresh-waybar
fi
