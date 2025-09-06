echo "Add Catppuccin Latte light theme"

if [[ ! -L "~/.config/hypr/themes/catppuccin-latte" ]]; then
  ln -snf ~/.local/share/hypr/themes/catppuccin-latte ~/.config/hypr/themes/
fi
