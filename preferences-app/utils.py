"""
Utilities for the Hyprland Preferences Application.
This module provides functions to interact with configuration files and system settings.
"""

import re
import subprocess
from pathlib import Path

# Paths relative to the installed user locations
USER_CONFIG_DIR = Path.home() / ".config/hypr"
LOCAL_SHARE_DIR = Path.home() / ".local/share/hypr"

# File locations
# Default looknfeel is in the shared location
LOOKNFEEL_CONF = LOCAL_SHARE_DIR / "default/hypr/looknfeel.conf"

# The active hyprland config is in the user's config directory
HYPRLAND_CONF = USER_CONFIG_DIR / "hyprland.conf"

# Bindings are typically in the shared location, but we also check user overrides
BINDINGS_DIR = LOCAL_SHARE_DIR / "default/hypr/bindings"
USER_BINDINGS_CONF = USER_CONFIG_DIR / "bindings.conf"

# Scripts are in the shared location
BIN_DIR = LOCAL_SHARE_DIR / "bin"

def get_repo_path():
    """Returns the path to the local share directory (the 'repo' location)."""
    return LOCAL_SHARE_DIR

def list_fonts():
    """Lists available monospace fonts using bin/hypr-font-list."""
    script = BIN_DIR / "hypr-font-list"
    try:
        # Check if script exists, if not, try falling back to local relative path for dev (optional)
        if not script.exists():
            # Fallback for dev environment where install might not be perfect
            dev_script = Path(__file__).parent.parent / "bin/hypr-font-list"
            if dev_script.exists():
                script = dev_script

        result = subprocess.run([str(script)], capture_output=True, text=True, check=True)
        return [line.strip() for line in result.stdout.splitlines() if line.strip()]
    except subprocess.CalledProcessError as e:
        print(f"Error listing fonts: {e}")
        return []

def set_font(font_name):
    """Sets the system font using bin/hypr-font-set."""
    script = BIN_DIR / "hypr-font-set"
    try:
        if not script.exists():
            dev_script = Path(__file__).parent.parent / "bin/hypr-font-set"
            if dev_script.exists():
                script = dev_script

        subprocess.run([str(script), font_name], check=True)
        return True
    except subprocess.CalledProcessError as e:
        print(f"Error setting font: {e}")
        return False

def read_file(filepath):
    """Reads file content."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            return f.readlines()
    except FileNotFoundError:
        return []

def write_file(filepath, lines):
    """Writes content to file."""
    # Ensure directory exists if writing to user config for first time?
    # Usually it exists.
    with open(filepath, 'w', encoding='utf-8') as f:
        f.writelines(lines)

def get_looknfeel_value(key_path):
    """
    Reads a value from looknfeel.conf.
    key_path: list of keys, e.g., ["general", "gaps_in"] or ["decoration", "shadow", "enabled"]
    """
    lines = read_file(LOOKNFEEL_CONF)
    if not lines:
        # Fallback for dev environment
        dev_looknfeel = Path(__file__).parent.parent / "default/hypr/looknfeel.conf"
        if dev_looknfeel.exists():
            lines = read_file(dev_looknfeel)

    context = []

    for line in lines:
        stripped = line.strip()
        if not stripped or stripped.startswith('#'):
            continue

        if stripped.endswith('{'):
            block_name = stripped[:-1].strip()
            context.append(block_name)
        elif stripped == '}':
            if context:
                context.pop()
        else:
            if '=' in stripped:
                key, val = [x.strip() for x in stripped.split('=', 1)]

                if len(key_path) == len(context) + 1:
                    match = True
                    for i, ctx in enumerate(context):
                        if ctx != key_path[i]:
                            match = False
                            break
                    if match and key == key_path[-1]:
                        return val
    return None

def set_looknfeel_value(key_path, value):
    """
    Updates a value in looknfeel.conf.
    key_path: list of keys, e.g., ["general", "gaps_in"]
    value: str (will be written as is)
    """
    target_file = LOOKNFEEL_CONF
    if not target_file.exists():
        dev_looknfeel = Path(__file__).parent.parent / "default/hypr/looknfeel.conf"
        if dev_looknfeel.exists():
            target_file = dev_looknfeel

    lines = read_file(target_file)
    new_lines = []
    context = []
    modified = False

    for line in lines:
        stripped = line.strip()

        if modified:
            new_lines.append(line)
            continue

        if stripped.endswith('{') and not stripped.startswith('#'):
            block_name = stripped[:-1].strip()
            context.append(block_name)
            new_lines.append(line)
            continue
        if stripped == '}' and not stripped.startswith('#'):
            if context:
                context.pop()
            new_lines.append(line)
            continue
        if '=' in stripped and not stripped.startswith('#'):
            key, _ = [x.strip() for x in stripped.split('=', 1)]

            if len(key_path) == len(context) + 1:
                match = True
                for i, ctx in enumerate(context):
                    if ctx != key_path[i]:
                        match = False
                        break
                if match and key == key_path[-1]:
                    indent = line[:line.find(key)]
                    new_lines.append(f"{indent}{key} = {value}\n")
                    modified = True
                    continue

        new_lines.append(line)

    if modified:
        write_file(target_file, new_lines)
        return True
    return False

def get_main_mod():
    """Reads $mainMod from hyprland.conf"""
    target_file = HYPRLAND_CONF
    if not target_file.exists():
        # Fallback to dev repo config/hypr/hyprland.conf
        dev_hyprland = Path(__file__).parent.parent / "config/hypr/hyprland.conf"
        if dev_hyprland.exists():
            target_file = dev_hyprland

    lines = read_file(target_file)
    for line in lines:
        stripped = line.strip()
        if stripped.startswith('$mainMod') and '=' in stripped:
            return stripped.split('=', 1)[1].strip()
    return "SUPER"

def set_main_mod(new_mod):
    """Updates $mainMod in hyprland.conf"""
    target_file = HYPRLAND_CONF
    if not target_file.exists():
        dev_hyprland = Path(__file__).parent.parent / "config/hypr/hyprland.conf"
        if dev_hyprland.exists():
            target_file = dev_hyprland

    lines = read_file(target_file)
    new_lines = []
    for line in lines:
        stripped = line.strip()
        if stripped.startswith('$mainMod') and '=' in stripped:
            new_lines.append(f"$mainMod = {new_mod}\n")
        else:
            new_lines.append(line)
    write_file(target_file, new_lines)

def get_keybindings():
    """
    Scans binding files and returns list of bindings.
    Returns: list of dicts {file, line, mods, key, desc, dispatch, arg}
    """
    bindings = []

    # Determine search locations
    bindings_dir = BINDINGS_DIR
    if not bindings_dir.exists():
        dev_bindings = Path(__file__).parent.parent / "default/hypr/bindings"
        if dev_bindings.exists():
            bindings_dir = dev_bindings

    files_to_scan = list(bindings_dir.glob("*.conf"))

    user_conf = USER_BINDINGS_CONF
    if user_conf.exists():
        files_to_scan.append(user_conf)

    # Also verify if config/hypr/bindings.conf exists (repo default bindings.conf)
    repo_bindings = USER_CONFIG_DIR / "bindings.conf"
    if not repo_bindings.exists():
        repo_bindings = Path(__file__).parent.parent / "config/hypr/bindings.conf"

    if repo_bindings.exists() and repo_bindings not in files_to_scan:
        files_to_scan.append(repo_bindings)

    for filepath in files_to_scan:
        lines = read_file(filepath)
        for i, line in enumerate(lines):
            stripped = line.strip()
            if stripped.startswith('bind'):
                parts = [p.strip() for p in stripped.split(',')]
                cmd = parts[0].split('=')[0].strip()

                try:
                    first_part_val = parts[0].split('=', 1)[1].strip()
                except IndexError:
                    continue

                mods = first_part_val
                key = parts[1] if len(parts) > 1 else ""

                desc = ""
                dispatcher = ""
                arg = ""

                if cmd == 'bindd':
                    desc = parts[2] if len(parts) > 2 else ""
                    dispatcher = parts[3] if len(parts) > 3 else ""
                    arg = ",".join(parts[4:]) if len(parts) > 4 else ""
                else:
                    dispatcher = parts[2] if len(parts) > 2 else ""
                    arg = ",".join(parts[3:]) if len(parts) > 3 else ""

                bindings.append({
                    "file": str(filepath),
                    "line": i,
                    "original": line,
                    "cmd": cmd,
                    "mods": mods,
                    "key": key,
                    "desc": desc,
                    "dispatcher": dispatcher,
                    "arg": arg
                })
    return bindings

def update_keybinding(filepath, line_idx, new_mods, new_key):
    """
    Updates the keybinding at the specific file and line.
    """
    lines = read_file(filepath)
    if line_idx >= len(lines):
        return False

    line = lines[line_idx]

    match = re.match(r"(bind[dm]?\s*=\s*)([^,]*)(\s*,\s*)([^,]+)(.*)", line)
    if match:
        prefix = match.group(1)
        # old_mods = match.group(2)
        separator1 = match.group(3)
        # old_key = match.group(4)
        rest = match.group(5)

        new_line = f"{prefix}{new_mods}{separator1}{new_key}{rest}\n"
        lines[line_idx] = new_line
        write_file(filepath, lines)
        return True
    return False
