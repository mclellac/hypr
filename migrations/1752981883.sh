echo "Replace wofi with walker as the default launcher"

if hypr-cmd-missing walker; then
  hypr-pkg-add walker-bin libqalculate

  hypr-pkg-drop wofi
  rm -rf ~/.config/wofi

  mkdir -p ~/.config/walker
  cp -r ~/.local/share/hypr/config/walker/* ~/.config/walker/
fi
