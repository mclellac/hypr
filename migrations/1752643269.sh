echo "Add new matte black theme"

if [[ ! -L "~/.config/hypr/themes/matte-black" ]]; then
  ln -snf ~/.local/share/hypr/themes/matte-black ~/.config/hypr/themes/
fi
