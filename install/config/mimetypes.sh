#!/bin/bash
#
# Sets default applications for various MIME types.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Refreshes the application database.
#######################################
refresh_database() {
  echo "Refreshing application database..."
  hypr-refresh-applications
  if command -v update-desktop-database &>/dev/null; then
    update-desktop-database "${HOME}/.local/share/applications"
  fi
}

#######################################
# Sets the default image viewer.
#######################################
set_image_defaults() {
  echo "Setting default image viewer..."
  local -r desktop_file="imv.desktop"
  local -ra mime_types=(
    "image/png"
    "image/jpeg"
    "image/gif"
    "image/webp"
    "image/bmp"
    "image/tiff"
  )
  local mime_type
  for mime_type in "${mime_types[@]}"; do
    xdg-mime default "${desktop_file}" "${mime_type}"
  done
}

#######################################
# Sets the default PDF viewer.
#######################################
set_pdf_defaults() {
  echo "Setting default PDF viewer..."
  xdg-mime default org.gnome.Evince.desktop application/pdf
}

#######################################
# Sets the default web browser.
#######################################
set_browser_defaults() {
  echo "Setting default web browser..."
  local -r desktop_file="chromium.desktop"
  xdg-settings set default-web-browser "${desktop_file}"
  xdg-mime default "${desktop_file}" x-scheme-handler/http
  xdg-mime default "${desktop_file}" x-scheme-handler/https
}

#######################################
# Sets the default video player.
#######################################
set_video_defaults() {
  echo "Setting default video player..."
  local -r desktop_file="mpv.desktop"
  local -ra mime_types=(
    "video/mp4"
    "video/x-msvideo"
    "video/x-matroska"
    "video/x-flv"
    "video/x-ms-wmv"
    "video/mpeg"
    "video/ogg"
    "video/webm"
    "video/quicktime"
    "video/3gpp"
    "video/3gpp2"
    "video/x-ms-asf"
    "video/x-ogm+ogg"
    "video/x-theora+ogg"
    "application/ogg"
  )
  local mime_type
  for mime_type in "${mime_types[@]}"; do
    xdg-mime default "${desktop_file}" "${mime_type}"
  done
}

#######################################
# Main function to orchestrate MIME type configuration.
#######################################
main() {
  refresh_database
  set_image_defaults
  set_pdf_defaults
  set_browser_defaults
  set_video_defaults
  echo "MIME type configuration complete."
}

main "$@"
