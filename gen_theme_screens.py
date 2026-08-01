"""
Generate screenshots for all themes.

This script cycles through all available themes, applies each one,
and takes a screenshot in a designated workspace.
"""

import subprocess
import time
import json
from pathlib import Path

# switch to workspace 9 and open the apps you want displayed in the screenshot
# launch this script from any other workspace and it will cycle through the
# themes and take a screenshot of it after a few seconds.


def run_cmd(cmd):
    """Run a shell command and return the result."""
    return subprocess.run(cmd, capture_output=True, text=True)


def hyprctl(args):
    """Execute a hyprctl command with the given arguments."""
    return run_cmd(["hyprctl"] + args)


def get_themes():
    """Return a list of available theme names."""
    themes_dir = Path("themes")
    return [d.name for d in themes_dir.iterdir() if d.is_dir()]


def get_current_theme():
    """Get the name of the currently active theme."""
    try:
        result = run_cmd(["bin/hypr-theme-current"])
        return result.stdout.strip()
    except Exception:
        return None


def get_current_workspace():
    """Return the ID of the currently active workspace."""
    try:
        res = hyprctl(["activeworkspace", "-j"])
        data = json.loads(res.stdout)
        return data["id"]
    except Exception:
        return 1


def set_theme(theme_name):
    """Apply the specified theme."""
    print(f"Applying theme: {theme_name}")
    subprocess.run(["bin/hypr-theme-set", theme_name])


def main():
    """Cycle through themes and capture screenshots."""
    themes = get_themes()
    original_theme = get_current_theme()
    original_workspace = get_current_workspace()

    # We use workspace 9 as a 'stage'
    target_workspace = 9

    print(f"Detected {len(themes)} themes.")
    print("--- PREPARATION ---")
    print(f"1. Please switch to Workspace {target_workspace}.")
    print(
        "2. Open and arrange the windows you want to showcase (e.g. terminal, file manager)."
    )
    print("3. Switch back to this terminal when ready.")
    print(
        f"\nThe script will switch to Workspace {target_workspace} and start in 10 seconds..."
    )

    for i in range(10, 0, -1):
        print(f"{i}...", end=" ", flush=True)
        time.sleep(1)
    print("\nStarting!")

    try:
        # Switch to the 'stage' workspace
        hyprctl(["dispatch", "workspace", str(target_workspace)])
        time.sleep(2)

        for theme in themes:
            screenshot_path = Path("themes") / theme / "screenshot.png"

            set_theme(theme)

            # Wait for theme to apply and UI to refresh (wallpapers can take a moment)
            print(f"Waiting for {theme} UI to refresh...")
            time.sleep(7)

            print(f"Capturing {theme}...")
            subprocess.run(["grim", str(screenshot_path)])
            print(f"Saved: {screenshot_path}")
            print("-" * 20)

    except KeyboardInterrupt:
        print("\nInterrupted by user.")
    finally:
        print(f"Restoring original workspace: {original_workspace}")
        hyprctl(["dispatch", "workspace", str(original_workspace)])

        if original_theme:
            print(f"Restoring original theme: {original_theme}")
            set_theme(original_theme)

        print("\nScreenshot generation complete!")


if __name__ == "__main__":
    main()
