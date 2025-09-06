#!/bin/bash

gsettings set org.gnome.desktop.interface gtk-theme "Adwaita-dark"
gsettings set org.gnome.desktop.interface color-scheme "prefer-dark"
gsettings set org.gnome.desktop.interface icon-theme "Yaru-blue"

# Set links for Nautilius action icons
sudo ln -snf /usr/share/icons/Adwaita/symbolic/actions/go-previous-symbolic.svg /usr/share/icons/Yaru/scalable/actions/go-previous-symbolic.svg
sudo ln -snf /usr/share/icons/Adwaita/symbolic/actions/go-next-symbolic.svg /usr/share/icons/Yaru/scalable/actions/go-next-symbolic.svg
sudo gtk-update-icon-cache /usr/share/icons/Yaru

# Setup theme links
mkdir -p ~/.config/hypr/themes
for f in ~/.local/share/hypr/themes/*; do ln -nfs "$f" ~/.config/hypr/themes/; done

# Set initial theme
mkdir -p ~/.config/hypr/current
ln -snf ~/.config/hypr/themes/tokyo-night ~/.config/hypr/current/theme
ln -snf ~/.config/hypr/current/theme/backgrounds/1-scenery-pink-lakeside-sunset-lake-landscape-scenic-panorama-7680x3215-144.png ~/.config/hypr/current/background

# Set specific app links for current theme
ln -snf ~/.config/hypr/current/theme/neovim.lua ~/.config/nvim/lua/plugins/theme.lua

mkdir -p ~/.config/btop/themes
ln -snf ~/.config/hypr/current/theme/btop.theme ~/.config/btop/themes/current.theme

mkdir -p ~/.config/mako
ln -snf ~/.config/hypr/current/theme/mako.ini ~/.config/mako/config
