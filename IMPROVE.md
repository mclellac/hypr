# Repository Improvements

## Critical
- [ ] Convert legacy single-line `windowrule` and `windowrulev2` syntax (e.g., in `themes/kanagawa/hyprland.conf` and `default/hypr/apps/browser.conf`) to the standard `windowrule {}` block syntax required by Hyprland v0.53+.
- [ ] Add `hypr_PATH` validation to all `bin/` scripts to ensure they are run from the correct context.
- [ ] Remove hardcoded user paths (e.g., `/home/dhh/`) from configuration files (e.g., `config/xournalpp/settings.xml`).

## Recommended
- [ ] Update themes (`material`, `material-bright`) to remove deprecated `col.shadow` syntax.
- [ ] Replace hardcoded `alacritty` commands in `bin/` scripts (e.g., `hypr-menu`) with a configurable terminal variable or wrapper.
- [ ] Add `set -euo pipefail` to all shell scripts for better error handling and safety.
- [ ] Improve `install.sh` error handling with `trap` to provide user-friendly messages on failure.

## Nice to Have
- [ ] Standardize header comments in all `bin/` scripts.
- [ ] Add documentation for `hypr-menu` and other custom utilities.
- [ ] Verify `bin/arch-update` dependencies and logic.
