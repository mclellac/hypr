echo "Add minimal starship prompt to terminal"

if hypr-cmd-missing starship; then
  hypr-pkg-add starship
  cp $hypr_PATH/config/starship.toml ~/.config/starship.toml
fi
