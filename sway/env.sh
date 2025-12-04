# Environment Variables
# Sway cannot set session environment variables directly in the config.
# These should be set before starting Sway (e.g. in ~/.profile or a wrapper script).

export MESA_LOADER_DRIVER_OVERRIDE=vmwgfx
export LIBGL_ALWAYS_SOFTWARE=1
export XCURSOR_SIZE=24
export HYPRCURSOR_SIZE=24

# Force all apps to use Wayland
export GDK_BACKEND=wayland,x11,*
export QT_QPA_PLATFORM=wayland;xcb
export QT_STYLE_OVERRIDE=kvantum
export SDL_VIDEODRIVER=wayland
export MOZ_ENABLE_WAYLAND=1
export ELECTRON_OZONE_PLATFORM_HINT=wayland
export OZONE_PLATFORM=wayland

# Input Method
export QT_IM_MODULE=fcitx
export XMODIFIERS=@im=fcitx
export SDL_IM_MODULE=fcitx
export GLFW_IM_MODULE=ibus
export INPUT_METHOD=fcitx

export QT_QPA_PLATFORMTHEME=kde
export XDG_MENU_PREFIX=plasma-

# Wayland Tearing/Cursors
export WLR_DRM_NO_ATOMIC=1
export WLR_NO_HARDWARE_CURSORS=1

# Desktop
export XDG_CURRENT_DESKTOP=sway
export XDG_SESSION_TYPE=wayland
export XDG_SESSION_DESKTOP=sway
export QT_WAYLAND_DISABLE_WINDOWDECORATION=1

export XCOMPOSEFILE=~/.XCompose
