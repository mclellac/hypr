#!/bin/bash

yay -S --noconfirm --needed \
    keepassxc \
    asdcontrol-git \
    alacritty \
    avahi \
    bash-completion \
    bat \
    blueberry \
    brightnessctl \
    btop \
    cargo \
    clang \
    cups \
    cups-browsed \
    cups-filters \
    cups-pdf \
    docker \
    docker-buildx \
    docker-compose \
    dust \
    evince \
    eza \
    fastfetch \
    fcitx5 \
    fcitx5-gtk \
    fcitx5-qt \
    fd \
    ffmpegthumbnailer \
    fontconfig \
    fzf \
    gcc14 \
    github-cli \
    gnome-calculator \
    gnome-keyring \
    gnome-themes-extra \
    gum \
    gvfs-mtp \
    gvfs-smb \
    hypridle \
    hyprland \
    hyprland-qtutils \
    hyprlock \
    hyprpicker \
    hyprshot \
    hyprsunset \
    imagemagick \
    impala \
    imv \
    inetutils \
    iwd \
    jq \
    kdenlive \
    kvantum-qt5 \
    lazydocker \
    lazygit \
    less \
    libqalculate \
    libreoffice \
    llvm \
    localsend \
    luarocks \
    mako \
    man \
    mariadb-libs \
    mise \
    mpv \
    nautilus \
    noto-fonts \
    noto-fonts-cjk \
    noto-fonts-emoji \
    noto-fonts-extra \
    nss-mdns \
    nvim \
    obs-studio \
    obsidian \
    chromium \
    pamixer \
    pinta \
    playerctl \
    plocate \
    plymouth \
    polkit-gnome \
    postgresql-libs \
    power-profiles-daemon \
    python-gobject \
    python-poetry-core \
    python-terminaltexteffects \
    qt5-wayland \
    ripgrep \
    satty \
    signal-desktop \
    slurp \
    spotify \
    starship \
    sushi \
    swaybg \
    swayosd \
    system-config-printer \
    tldr \
    tree-sitter-cli \
    ttf-cascadia-mono-nerd \
    ttf-ia-writer \
    ttf-jetbrains-mono-nerd \
    typora \
    tzupdate \
    ufw \
    ufw-docker \
    unzip \
    uwsm \
    go \
    gtk4 \
    gtk-layer-shell \
    protobuf \
    glib2-devel \
    gobject-introspection \
    pkg-config \
    waybar \
    wf-recorder \
    whois \
    wiremix \
    wireplumber \
    wl-clip-persist \
    wl-clipboard \
    wl-screenrec \
    woff2-font-awesome \
    xdg-desktop-portal-gtk \
    xdg-desktop-portal-hyprland \
    xmlstarlet \
    xournalpp \
    yaru-icon-theme \
    yay-bin \
    zoxide

# Install elephant
rm -rf /tmp/elephant
git clone https://github.com/abenz1267/elephant.git /tmp/elephant
cd /tmp/elephant/cmd
go build elephant.go
mkdir -p ~/.local/bin
cp elephant ~/.local/bin/
mkdir -p ~/.config/elephant/providers
cd /tmp/elephant/internal/providers/desktopapplications
go build -buildmode=plugin
cp desktopapplications.so ~/.config/elephant/providers/
cd /tmp/elephant/internal/providers/files
go build -buildmode=plugin
cp files.so ~/.config/elephant/providers/
cd /tmp/elephant/internal/providers/runner
go build -buildmode=plugin
cp runner.so ~/.config/elephant/providers/

# Install walker
rm -rf /tmp/walker
git clone https://github.com/abenz1267/walker.git /tmp/walker
cd /tmp/walker
export PKG_CONFIG_PATH=/usr/lib/pkgconfig
cargo build --release
mkdir -p ~/.local/bin
cp target/release/walker ~/.local/bin/
