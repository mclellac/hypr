echo "Install swayOSD to show volume status"

if hypr-cmd-missing swayosd-server; then
  hypr-pkg-add swayosd
  setsid uwsm app -- swayosd-server &>/dev/null &
fi
