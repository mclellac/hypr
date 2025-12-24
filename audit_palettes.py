#!/usr/bin/env python3
import os
import re
import collections

THEMES_DIR = "themes"

# Regexes for finding colors
HEX_COLOR_RE = re.compile(r"#([0-9a-fA-F]{3,4}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})\b")
HYPR_COLOR_RE = re.compile(r"0x([0-9a-fA-F]{8})\b")
RGB_RE = re.compile(r"rgba?\s*\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)(?:\s*,\s*([0-9.]+))?\s*\)")

# Regexes for finding variable definitions
WAYBAR_DEF_RE = re.compile(r"@define-color\s+([a-zA-Z0-9_-]+)\s+")
KITTY_DEF_RE = re.compile(r"^([a-zA-Z0-9_-]+)\s+")
TOML_INI_DEF_RE = re.compile(r'([a-zA-Z0-9_-]+)\s*=\s*')
HYPR_DEF_RE = re.compile(r"\$([a-zA-Z0-9_-]+)\s*=\s*")
BTOP_DEF_RE = re.compile(r'theme\[([a-zA-Z0-9_-]+)\]\s*=\s*')

# INI/TOML Section Detection
INI_SECTION_RE = re.compile(r"^\[(.+)\]$")

def canonicalize_hex(h):
    h = h.lower()
    if len(h) == 4: return f"#{h[1]*2}{h[2]*2}{h[3]*2}"
    if len(h) == 5: return f"#{h[1]*2}{h[2]*2}{h[3]*2}{h[4]*2}"
    if len(h) == 9 and h.endswith("ff"): return h[:-2]
    return h

def hypr_to_hex(h_str):
    alpha = h_str[0:2].lower()
    rgb = h_str[2:].lower()
    return f"#{rgb}" if alpha == "ff" else f"#{rgb}{alpha}"

def rgb_to_hex(r, g, b, a=None):
    r, g, b = int(r), int(g), int(b)
    if a is not None:
        ai = int(float(a) * 255)
        if ai >= 255: return f"#{r:02x}{g:02x}{b:02x}"
        return f"#{r:02x}{g:02x}{b:02x}{ai:02x}"
    return f"#{r:02x}{g:02x}{b:02x}"

def get_app_name(filename):
    lower = filename.lower()
    if "waybar" in lower: return "Waybar"
    if "kitty" in lower: return "Kitty"
    if "alacritty" in lower: return "Alacritty"
    if "hyprland" in lower: return "Hyprland"
    if "hyprlock" in lower: return "Hyprlock"
    if "mako" in lower: return "Mako"
    if "walker" in lower: return "Walker"
    if "btop" in lower: return "Btop"
    if "chromium" in lower: return "Chromium"
    if "neovim" in lower or ".lua" in lower: return "Neovim"
    if "swayosd" in lower: return "SwayOSD"
    if "wofi" in lower: return "Wofi"
    if "rofi" in lower: return "Rofi"
    return "Other"

def analyze_file(filepath):
    colors = [] # list of dicts: {hex, name}
    try:
        with open(filepath, 'r') as f:
            lines = f.readlines()
    except:
        return "Unknown", []

    fname = os.path.basename(filepath)
    app = get_app_name(fname)
    current_section = None

    # Pre-scan for chromium
    if fname == "chromium.theme":
        content = "".join(lines).strip()
        if re.match(r"^\d+,\d+,\d+$", content):
            r, g, b = content.split(",")
            h = rgb_to_hex(r,g,b)
            colors.append({'hex': h, 'name': 'chromium_frame'})
            return app, colors

    for line in lines:
        line_s = line.strip()

        # INI/TOML Section Detection
        # Handle Alacritty (TOML) and Mako (INI) sections
        if (app == "Mako" or app == "Alacritty") and line_s.startswith("[") and line_s.endswith("]"):
            m = INI_SECTION_RE.match(line_s)
            if m:
                current_section = m.group(1)
            continue

        # Skip comment-only lines
        if line_s.startswith(("//", "#", ";")) and app != "Chromium":
            continue

        found_hex = []

        # Hex
        for m in HEX_COLOR_RE.finditer(line):
            found_hex.append((m.group(0), "hex"))

        # Hyprland
        for m in HYPR_COLOR_RE.finditer(line):
             found_hex.append((m.group(1), "hypr"))

        # RGB
        for m in RGB_RE.finditer(line):
            found_hex.append((m.groups(), "rgb"))

        if not found_hex:
            continue

        # Extract variable name if definition
        var_name = None

        if (app == "Waybar" or app == "Walker") and "@define-color" in line:
            m = WAYBAR_DEF_RE.search(line)
            if m: var_name = m.group(1)

        elif app == "Kitty":
             m = KITTY_DEF_RE.match(line)
             if m: var_name = m.group(1)

        elif (app == "Hyprland" or app == "Hyprlock") and line.strip().startswith("$"):
             m = HYPR_DEF_RE.search(line)
             if m: var_name = m.group(1)

        elif app == "Btop":
            m = BTOP_DEF_RE.search(line)
            if m: var_name = m.group(1)

        elif app in ["Alacritty", "Mako"] and "=" in line:
             m = TOML_INI_DEF_RE.search(line)
             if m: var_name = m.group(1)

        # Append section to name for Mako/Alacritty to avoid duplicates
        if var_name and current_section:
            var_name = f"{var_name} ({current_section})"

        for val, type_ in found_hex:
            final_hex = ""
            if type_ == "hex": final_hex = canonicalize_hex(val)
            elif type_ == "hypr": final_hex = canonicalize_hex(hypr_to_hex(val))
            elif type_ == "rgb": final_hex = canonicalize_hex(rgb_to_hex(*val))

            name = var_name if var_name else "unknown"
            colors.append({'hex': final_hex, 'name': name})

    return app, colors

def generate_table(app, colors):
    if not colors: return ""

    md = f"### {app}\n\n"
    md += "| Name | Hex |\n"
    md += "|---|---|\n"

    # Sort by name
    colors.sort(key=lambda x: (x['name'] == 'unknown', x['name']))

    for c in colors:
        name = c['name']
        hex_val = c['hex']

        name_s = f"`{name}`"
        # Ensure max length to fit 80 chars
        # | `name` | #hex |
        # 4 + len + 3 + len + 2
        # Max name len = 80 - 4 - 3 - 2 - 7 = 64
        if len(name_s) > 60:
             name_s = name_s[:57] + "..."

        md += f"| {name_s} | {hex_val} |\n"

    md += "\n"
    return md

def main():
    if not os.path.exists(THEMES_DIR):
        print("Themes directory not found.")
        return

    themes = sorted([d for d in os.listdir(THEMES_DIR) if os.path.isdir(os.path.join(THEMES_DIR, d))])

    for theme in themes:
        theme_dir = os.path.join(THEMES_DIR, theme)
        palette_file = os.path.join(theme_dir, "palette.md")

        app_colors = collections.defaultdict(list)

        for root, dirs, files in os.walk(theme_dir):
            files.sort()
            for file in files:
                if file == "palette.md": continue
                if file.endswith((".png", ".jpg", ".lock", ".pyc")): continue

                filepath = os.path.join(root, file)
                app, colors = analyze_file(filepath)
                if colors:
                    app_colors[app].extend(colors)

        with open(palette_file, 'w') as f:
            f.write(f"# {theme.capitalize()} Palette\n\n")

            for app in sorted(app_colors.keys()):
                unique_colors = []
                seen = set()
                # Deduplicate based on name and hex
                for c in app_colors[app]:
                    key = (c['name'], c['hex'])
                    if key not in seen:
                        unique_colors.append(c)
                        seen.add(key)

                if unique_colors:
                    f.write(generate_table(app, unique_colors))

        print(f"Updated {theme}")

if __name__ == "__main__":
    main()
