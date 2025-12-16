#!/bin/bash
set -euo pipefail

# Iterate through all themes
for theme_dir in themes/*; do
  [ -d "$theme_dir" ] || continue
  theme=$(basename "$theme_dir")

  palette_file="$theme_dir/palette.md"
  if [ ! -f "$palette_file" ]; then
    echo "WARNING: No palette.md found for theme $theme"
    continue
  fi

  echo "Checking theme: $theme"

  # Check hyprland.conf
  hypr_conf="$theme_dir/hyprland.conf"
  if [ -f "$hypr_conf" ]; then
    # Extract border colors lines
    # Look for hex codes starting with # or inside rgb/rgba
    # We will grep for anything looking like a hex code: 6 or 8 hex digits
    # Hyprland can use rgb(xxxxxx) or rgba(xxxxxxff) or 0xffxxxxxx

    # Simple strategy: find all 6-digit hex strings in the file that are part of color definitions
    # But filtering only relevant lines

    border_lines=$(grep "col.*_border" "$hypr_conf" || true)

    # Extract hex codes (simplistic)
    for color in $(echo "$border_lines" | grep -oE '([0-9a-fA-F]{6}|[0-9a-fA-F]{8})'); do
        # ignore pure numbers that might be matched accidentally if they look like hex, but 6-8 chars usually implies color in this context
        # normalize to lowercase
        color_lower=$(echo "$color" | tr '[:upper:]' '[:lower:]')

        # Check if this hex exists in palette.md (searching for the 6-digit version is safer as alpha might vary)
        # Taking first 6 chars
        color_base=${color_lower:0:6}

        if ! grep -qi "$color_base" "$palette_file"; then
             echo "  MISMATCH in hyprland.conf: Color $color (base $color_base) not found in palette.md"
        fi
    done
  fi

  # Check colors directory
  if [ -d "$theme_dir/colors" ]; then
      for file in "$theme_dir/colors"/*; do
          [ -f "$file" ] || continue
              # Look for hex codes
              for color in $(grep -oE '#[0-9a-fA-F]{6}' "$file" | sed 's/#//'); do
                  color_lower=$(echo "$color" | tr '[:upper:]' '[:lower:]')
                   if ! grep -qi "$color_lower" "$palette_file"; then
                        echo "  MISMATCH in $(basename "$file"): Color #$color not found in palette.md"
                   fi
              done
      done
  fi

done
