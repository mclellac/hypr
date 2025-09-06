echo "Replace buggy native Zoom client with webapp"

if hypr-pkg-present zoom; then
  hypr-pkg-drop zoom
  hypr-webapp-install "Zoom" https://app.zoom.us/wc/home https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/png/zoom.png
fi
