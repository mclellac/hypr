#!/usr/bin/env python3
import os
import re

THEMES_DIR = "themes"

def get_blue_from_kitty(theme_path):
    kitty_conf = os.path.join(theme_path, "kitty.conf")
    if os.path.exists(kitty_conf):
        with open(kitty_conf, 'r') as f:
            for line in f:
                # color4 #xxxxxx or color4 #...
                m = re.match(r"^color4\s+(#[0-9a-fA-F]+)", line.strip())
                if m:
                    return m.group(1)
    return None

def update_mako(theme_name):
    theme_path = os.path.join(THEMES_DIR, theme_name)
    mako_files = ["mako", "mako.ini"]
    mako_path = None
    for f in mako_files:
        p = os.path.join(theme_path, f)
        if os.path.exists(p):
            mako_path = p
            break

    if not mako_path:
        print(f"No mako config found for {theme_name}")
        return

    # Check if rule exists
    with open(mako_path, 'r') as f:
        content = f.read()

    if "[category=mpd]" in content:
        print(f"Skipping {theme_name}: Rule already exists")
        return

    # Get Blue
    blue = get_blue_from_kitty(theme_path)
    if not blue:
        # Fallback to amateur's blue if we can't determine
        blue = "#00aaff"
        print(f"Warning: Could not find color4 for {theme_name}, using {blue}")

    # Append rule
    # Ensure newline before block
    new_rule = f"\n[category=mpd]\nborder-color={blue}\ndefault-timeout=2000\ngroup-by=category\n"
    if not content.endswith("\n"):
        new_rule = "\n" + new_rule

    with open(mako_path, 'a') as f:
        f.write(new_rule)

    print(f"Updated {theme_name} with border-color={blue}")

def main():
    if not os.path.exists(THEMES_DIR):
        print("Themes directory not found.")
        return

    themes = sorted([d for d in os.listdir(THEMES_DIR) if os.path.isdir(os.path.join(THEMES_DIR, d))])
    for theme in themes:
        if theme == "amateur": continue
        update_mako(theme)

if __name__ == "__main__":
    main()
