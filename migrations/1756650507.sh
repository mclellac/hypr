echo "Fix JetBrains font setting"

if [[ $(hypr-font-current) == JetBrains* ]]; then
  hypr-font-set "JetBrainsMono Nerd Font"
fi
