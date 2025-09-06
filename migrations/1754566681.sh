echo "Make new Osaka Jade theme available as new default"

if [[ ! -L ~/.config/hypr/themes/osaka-jade ]]; then
  rm -rf ~/.config/hypr/themes/osaka-jade
  git -C ~/.local/share/hypr checkout -f themes/osaka-jade
  ln -nfs ~/.local/share/hypr/themes/osaka-jade ~/.config/hypr/themes/osaka-jade
fi
