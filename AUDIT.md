# Hyprland Configuration Audit Report

This report outlines the findings from a deep audit of the Hyprland configuration, focusing on compatibility with Hyprland v0.53+, modernization, system stability, and code cleanliness.

## 1. Critical: Hyprland v0.53+ Compatibility

These issues can cause the configuration to fail or throw errors in newer Hyprland versions.

### 1.1 Single-Line `windowrule` Syntax
**Severity: Critical**
Hyprland v0.53+ mandates the block syntax (`windowrule { ... }`). Single-line `windowrule` definitions mixed with block syntax are deprecated or forbidden.

*   **File:** `default/hypr/apps/browser.conf`
*   **Issues:**
    *   `windowrule = match:tag chromium-based-browser, opacity 1.0 1.0`
    *   `windowrule = match:class ^(crx_.*)$, opacity 1.0 1.0`
    *   `windowrule = match:tag firefox-based-browser, opacity 1.0 1.0`
*   **Status:** [x] Converted to block syntax.

### 1.2 Deprecated `col.shadow`
**Severity: High**
The `col.shadow` variable is deprecated in favor of the `shadow { ... }` configuration block.

*   **Files:** `themes/material/hyprland.conf`, `themes/material-bright/hyprland.conf` (commented out)
*   **Status:** [x] Updated `themes/material/hyprland.conf` and `themes/material-bright/hyprland.conf` to use `shadow { ... }` block syntax.

### 1.3 `windowrulev2` Usage
**Severity: Policy Violation**
The use of `windowrulev2` is explicitly forbidden.

*   **File:** `config/hypr/windows.conf` (found in comments)
*   **Recommendation:** Ensure no `windowrulev2` syntax exists in the codebase, even in comments, to prevent accidental uncommenting and usage.

## 2. Major: System Architecture & Modernization

These issues affect the installation reliability and system integration.

### 2.1 Repository Location Dependency
**Severity: Critical**
The configuration extensively references `~/.local/share/hypr/` for sourcing default configurations, scripts, and assets. However, the `install.sh` script does not enforce installing the repository to this location. If a user clones the repo elsewhere, the configuration will break.

*   **Files:** `config/hypr/hyprland.conf`, `default/hypr/bindings.conf`, `bin/*` scripts.
*   **Status:** [x] Updated `install.sh` to symlink the repository to `~/.local/share/hypr` if not already there.

### 2.2 Inconsistent `$mainMod` Definition
**Severity: Major**
There is a conflict in the modifier key definition, which leads to confusing keybindings.

*   **Conflict:**
    *   `config/hypr/hyprland.conf` defines `$mainMod = ALT`.
    *   `default/hypr/bindings.conf` uses hardcoded `SUPER` for many bindings (e.g., `bindd = SUPER, return, ...`).
*   **Status:** [x] Replaced hardcoded `SUPER` with `$mainMod` in `default/hypr/bindings.conf` and updated binding definitions to `bindd`.

### 2.3 `UWSM` (Universal Wayland Session Manager) Integration
**Severity: Moderate**
While `uwsm` is used for most autostart entries, some remain legacy `exec-once` calls.

*   **File:** `config/hypr/autostart.conf`
*   **Issues:**
    *   `exec-once = /usr/lib/polkit-gnome/polkit-gnome-authentication-agent-1`
    *   `exec-once = hypr-cmd-first-run`
*   **Status:** [x] Wrapped all `exec-once` commands in `uwsm app --` in `default/hypr/autostart.conf`.

## 3. Minor: Cleanliness & Maintenance

These issues affect the maintainability and "cleanliness" of the configuration.

### 3.1 Hardcoded User Paths
**Severity: Moderate**
Several configuration files contain hardcoded paths to specific user home directories, which will break for other users.

*   **Files:**
    *   `config/xournalpp/settings.xml`: `/home/dhh/`
    *   `applications/*.desktop`: `/home/mclellac/`
*   **Status:** [x] Removed hardcoded paths in `config/xournalpp/settings.xml` and replaced hardcoded paths in `.desktop` files with placeholders that are dynamically resolved during installation in `install/config/config.sh`.

### 3.2 Keybinding Documentation (`bind` vs `bindd`)
**Severity: Minor**
Hyprland supports `bindd` to provide descriptions for the help menu. Some files still use plain `bind`.

*   **File:** `config/hypr/bindings.conf` (Workspace switching), `default/hypr/bindings.conf` (`SUPER, V`).
*   **Status:** [x] Converted remaining `bind` calls to `bindd` in `config/hypr/bindings.conf` and `default/hypr/bindings.conf`.

## 4. Summary of Recommended Actions

1.  **Refactor Syntax:** Fix `default/hypr/apps/browser.conf` to use `windowrule` blocks.
2.  **Fix Installation Logic:** Ensure `~/.local/share/hypr` exists and is populated/symlinked during install.
3.  **Sanitize Paths:** Remove hardcoded `/home/user` paths.
4.  **Unify Bindings:** Use `$mainMod` consistently and ensure all bindings have descriptions.
5.  **Modernize Autostart:** Wrap remaining `exec-once` commands in `uwsm app --`.

This audit confirms that while the configuration is feature-rich, it requires significant structural cleanup to be "stable" and "clean" for general distribution.
