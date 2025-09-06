echo "Add the new ristretto theme as an option"

if [[ ! -L ~/.config/hypr/themes/ristretto ]]; then
  ln -nfs ~/.local/share/hypr/themes/ristretto ~/.config/hypr/themes/
fi
