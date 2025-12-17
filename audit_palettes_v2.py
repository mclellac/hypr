#!/usr/bin/env python
import os
import re

THEMES_DIR = "themes"

# Regexes
# Hex: #RRGGBB, #RRGGBBAA, #RGB, #RGBA
HEX_COLOR_RE = re.compile(r"#([0-9a-fA-F]{3,4}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})\b")
# Hyprland: 0xAARRGGBB
HYPR_COLOR_RE = re.compile(r"0x([0-9a-fA-F]{8})\b")
# RGB/RGBA: rgb(r, g, b), rgba(r, g, b, a)
# Matches "rgb(0, 0, 0)" or "rgba(0, 0, 0, 0.3)"
RGB_RE = re.compile(
    r"rgba?\s*\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)(?:\s*,\s*([0-9.]+))?\s*\)"
)

# Variable definitions (heuristics)
WAYBAR_DEF_RE = re.compile(r"@define-color\s+([a-zA-Z0-9_-]+)\s+")
KITTY_DEF_RE = re.compile(r"^([a-zA-Z0-9_-]+)\s+#")
TOML_INI_DEF_RE = re.compile(r'([a-zA-Z0-9_-]+)\s*=\s*["\']?#')
HYPR_DEF_RE = re.compile(r"\$([a-zA-Z0-9_-]+)\s*=\s*0x")


def canonicalize_hex(h):
    h = h.lower()
    # Expand short hex #abc -> #aabbcc
    if len(h) == 4:  # #rgb
        r = h[1]
        g = h[2]
        b = h[3]
        return f"#{r}{r}{g}{g}{b}{b}"
    if len(h) == 5:  # #rgba
        r = h[1]
        g = h[2]
        b = h[3]
        a = h[4]
        return f"#{r}{r}{g}{g}{b}{b}{a}{a}"  # keep alpha for now

    # Strip alpha if full FF
    if len(h) == 9 and h.endswith("ff"):  # #RRGGBBff -> #RRGGBB
        return h[:-2]

    return h


def hypr_to_hex(h_str):
    # h_str is "11223344" (AARRGGBB) from 0x...
    # Convert to #RRGGBB[AA]
    alpha = h_str[0:2].lower()
    rgb = h_str[2:].lower()
    if alpha == "ff":
        return f"#{rgb}"
    else:
        return f"#{rgb}{alpha}"


def rgb_to_hex(r, g, b, a=None):
    r = int(r)
    g = int(g)
    b = int(b)
    if a is not None:
        try:
            af = float(a)
            ai = int(af * 255)
            if ai >= 255:
                return f"#{r:02x}{g:02x}{b:02x}"
            return f"#{r:02x}{g:02x}{b:02x}{ai:02x}"
        except:
            return f"#{r:02x}{g:02x}{b:02x}"
    return f"#{r:02x}{g:02x}{b:02x}"


def parse_palette(filepath):
    colors = {}  # name -> hex
    hexes = {}  # hex -> name (first found)
    rows = []

    if not os.path.exists(filepath):
        return colors, hexes, rows

    with open(filepath, "r") as f:
        lines = f.readlines()

    for line in lines:
        if line.strip().startswith("|") and "---" not in line:
            parts = [p.strip() for p in line.strip().split("|")]
            if len(parts) >= 3:
                name = parts[1].strip("`")
                hex_val = parts[2].lower()
                desc = parts[3] if len(parts) > 3 else ""

                if name.lower() == "name" or name == "":
                    continue

                canon_hex = canonicalize_hex(hex_val)

                colors[name] = canon_hex
                if canon_hex not in hexes:
                    hexes[canon_hex] = name

                rows.append({"name": name, "hex": hex_val, "desc": desc})
    return colors, hexes, rows


def audit_theme(theme_name):
    theme_path = os.path.join(THEMES_DIR, theme_name)
    palette_path = os.path.join(theme_path, "palette.md")

    colors, hexes, rows = parse_palette(palette_path)
    found_colors = set()

    # 1. Check chromium.theme explicitly
    chrom_path = os.path.join(theme_path, "chromium.theme")
    if os.path.exists(chrom_path):
        with open(chrom_path, "r") as f:
            content = f.read().strip()
            # Format: R,G,B
            if re.match(r"^\d+,\d+,\d+$", content):
                r, g, b = content.split(",")
                hex_val = rgb_to_hex(r, g, b)
                canon = canonicalize_hex(hex_val)

                if canon not in hexes and canon not in found_colors:
                    name = "chromium_frame_color"
                    rows.append(
                        {
                            "name": name,
                            "hex": hex_val,
                            "desc": "Detected in chromium.theme",
                        }
                    )
                    found_colors.add(canon)
                    hexes[canon] = name

    # 2. Walk all files
    for root, dirs, files in os.walk(theme_path):
        for file in files:
            if file == "palette.md":
                continue
            # Skip obvious binary/lock files
            if file.endswith((".png", ".jpg", ".jpeg", ".lock", ".pyc")):
                continue
            # We explicitly want to check .sh, .py if they contain hardcoded colors,
            # but usually they don't in this repo's themes. Let's include them to be safe as per user request.

            filepath = os.path.join(root, file)
            try:
                with open(filepath, "r") as f:
                    content = f.read()
            except:
                continue

            # Find Hex Colors
            for match in HEX_COLOR_RE.finditer(content):
                raw_hex = match.group(0)
                canon = canonicalize_hex(raw_hex)

                if canon not in hexes and canon not in found_colors:
                    name = f"unknown_{canon.replace('#', '')}"

                    # Try to find a better name
                    with open(filepath, "r") as f_lines:
                        for line in f_lines:
                            if raw_hex in line:
                                if file.endswith(".css"):
                                    m = WAYBAR_DEF_RE.search(line)
                                    if m:
                                        name = m.group(1)
                                elif file.endswith(".conf") and "kitty" in file:
                                    m = KITTY_DEF_RE.match(line.strip())
                                    if m:
                                        n = m.group(1)
                                        if n.startswith("color"):
                                            name = f"term_{n}"
                                        else:
                                            name = n
                                elif file.endswith((".toml", ".ini", ".lua")):
                                    # Lua: name = "#hex"
                                    # TOML: name = "#hex"
                                    m = TOML_INI_DEF_RE.search(line)
                                    if m:
                                        name = m.group(1)
                                break

                    rows.append(
                        {
                            "name": name,
                            "hex": raw_hex.lower(),
                            "desc": f"Detected in {file}",
                        }
                    )
                    found_colors.add(canon)
                    hexes[canon] = name

            # Find Hyprland Colors
            for match in HYPR_COLOR_RE.finditer(content):
                raw_hypr = match.group(1)
                hex_val = hypr_to_hex(raw_hypr)
                canon = canonicalize_hex(hex_val)

                if canon not in hexes and canon not in found_colors:
                    name = f"unknown_{canon.replace('#', '')}"
                    with open(filepath, "r") as f_lines:
                        for line in f_lines:
                            if f"0x{raw_hypr}" in line:
                                m = HYPR_DEF_RE.search(line)
                                if m:
                                    name = m.group(1)
                                break

                    rows.append(
                        {
                            "name": name,
                            "hex": hex_val.lower(),
                            "desc": f"Detected in {file}",
                        }
                    )
                    found_colors.add(canon)
                    hexes[canon] = name

            # Find RGB/RGBA Colors
            for match in RGB_RE.finditer(content):
                r, g, b, a = match.groups()
                hex_val = rgb_to_hex(r, g, b, a)
                canon = canonicalize_hex(hex_val)

                if canon not in hexes and canon not in found_colors:
                    name = f"unknown_{canon.replace('#', '')}"
                    rows.append(
                        {
                            "name": name,
                            "hex": hex_val.lower(),
                            "desc": f"Detected in {file} (from rgb/rgba)",
                        }
                    )
                    found_colors.add(canon)
                    hexes[canon] = name

    # Sort rows
    rows.sort(key=lambda x: x["name"])

    # Write back
    with open(palette_path, "w") as f:
        f.write(f"# {theme_name.capitalize()} Palette\n\n")
        f.write("| Name | Hex | Description |\n")
        f.write("|---|---|---|\n")
        for row in rows:
            name_col = f"`{row['name']}`"
            hex_col = row["hex"]
            desc_col = row["desc"]

            # Calculate available space for description
            # Line structure: | {name_col} | {hex_col} | {desc_col} |
            # Fixed chars: |  |  |  | (4 pipes + 6 spaces = 10 chars)
            # Actually spaces are around the content: | name | hex | desc |

            prefix_len = len(f"| {name_col} | {hex_col} | ")
            suffix_len = len(" |")
            available_len = 80 - prefix_len - suffix_len

            if len(desc_col) > available_len:
                if available_len > 3:
                   desc_col = desc_col[:available_len-3] + "..."
                else:
                   desc_col = desc_col[:available_len] # Very tight, just cut

            f.write(f"| {name_col} | {hex_col} | {desc_col} |\n")

    print(f"Updated {theme_name}")


def main():
    if not os.path.exists(THEMES_DIR):
        print("Themes directory not found.")
        return

    themes = [
        d for d in os.listdir(THEMES_DIR) if os.path.isdir(os.path.join(THEMES_DIR, d))
    ]
    for theme in themes:
        audit_theme(theme)


if __name__ == "__main__":
    main()
