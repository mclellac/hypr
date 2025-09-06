echo "Allow updating of timezone by right-clicking on the clock (or running hypr-cmd-tzupdate)"

if hypr-cmd-missing tzupdate; then
  $hypr_PATH/install/config/timezones.sh
  hypr-refresh-waybar
fi
