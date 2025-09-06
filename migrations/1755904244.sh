echo "Update fastfetch config with new hypr logo"

hypr-refresh-config fastfetch/config.jsonc

mkdir -p ~/.config/hypr/branding
cp $hypr_PATH/icon.txt ~/.config/hypr/branding/about.txt
