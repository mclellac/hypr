# `bin/` Scripts: Coding Style Audit and Improvement Plan

This document provides a detailed audit of the shell scripts located in the `bin/` directory. It covers the existing coding conventions, style, and provides a list of actionable recommendations for improving the robustness, readability, and maintainability of the codebase.

## 1. Overview of Scripting Architecture

The scripts in `bin/` form a cohesive, namespaced API for managing the Hyprland environment. The `hypr-` prefix is used consistently, and the `hypr-menu` script acts as a central dispatcher, providing a user-friendly, interactive layer over the underlying functionality. This is a strong architectural pattern.

The scripts are written in `bash` and heavily leverage common Unix utilities (`grep`, `sed`, `find`) and modern tools like `fzf` and `walker` to create powerful interactive workflows.

## 2. Coding Conventions

The following conventions have been observed across the scripts.

### Variable Naming and Scoping

*   **Convention:** There is a mixed strategy. Environment or global-like variables are typically `UPPER_CASE` (e.g., `THEMES_DIR`). Variables within functions are often `lower_case` (e.g., `prompt`, `options`).
*   **Inconsistency:** The use of the `local` keyword is sporadic. In `hypr-menu`, most variables (`theme`, `profile`, `options`) are implicitly global, which can lead to unexpected behavior and makes the code harder to reason about. For example, a variable set in one menu function could accidentally affect another.

### Function Definitions

*   **Convention:** Functions are consistently defined using the `my_func() { ... }` syntax, which is standard and portable.

### Conditionals and Comparisons

*   **Convention:** The modern `[[ ... ]]` test construct is used almost exclusively over the older `[ ... ]`. This is a good practice as it's more robust and less error-prone.
*   **Example:** `if [[ -n "$pkg_names" ]]`

### Pipelines and Command Substitution

*   **Convention:** Pipelines are used extensively to chain commands. Command substitution is done with `$(...)`, which is preferred over backticks.
*   **Example:** `theme=$(menu "Theme" "$(hypr-theme-list)")`

## 3. Code Style

### Formatting and Readability

*   Indentation is generally consistent (2 spaces).
*   The code is readable, but large functions with nested `case` statements, like those in `hypr-menu`, can be difficult to navigate.

### Use of Nerd Fonts

*   Many menu options contain Nerd Font icons (e.g., `󰸌 Theme`). While this provides a visually rich menu, it can make the source code harder to read and edit in environments without proper font support.

### Monolithic Scripts

*   `hypr-menu` is a prime example of a monolithic script. It contains the logic for dozens of menus and sub-menus in a single file, making it over 400 lines long. This reduces maintainability.

## 4. Error Handling

*   **Inconsistency:** The approach to error handling is not uniform. Some scripts, like `hypr-pkg-install`, start with `set -e` to exit on error. Many others, including the critical `hypr-menu` and `hypr-theme-set`, do not. This means an error in one command might not stop the script, leading to unpredictable results.
*   **Cancellation Logic:** Menu cancellation is handled by checking for a magic string `"CNCLD"` or an empty string (`-z "$theme"`). This logic is repeated in every menu function and makes the code verbose.

## 5. TODO: Suggested Improvements

This section lists concrete, actionable steps to improve the quality of the `bin/` scripts.

1.  **Standardize Error Handling:**
    *   **Action:** Add `set -euo pipefail` to the top of every script.
        *   `set -e`: Exit immediately if a command exits with a non-zero status.
        *   `set -u`: Treat unset variables as an error.
        *   `set -o pipefail`: The return value of a pipeline is the status of the last command to exit with a non-zero status.
    *   **Benefit:** Makes scripts far more robust and prevents them from continuing in a partially failed state.

2.  **Enforce Strict Variable Scoping:**
    *   **Action:** Mandate the use of `local` for all variables defined inside a function. Perform a full audit of `hypr-menu` to apply this rule.
    *   **Benefit:** Prevents variable collisions and makes functions self-contained and predictable.

3.  **Refactor `hypr-menu` into Modules:**
    *   **Action:** Break `hypr-menu` apart. Create a `menus/` subdirectory in `bin/`. Move related menu functions into separate files (e.g., `bin/menus/install.sh`, `bin/menus/setup.sh`). The main `hypr-menu` script would then source these files.
    *   **Benefit:** Dramatically improves maintainability, readability, and separation of concerns.

4.  **Quote All Variable Expansions:**
    *   **Action:** Enforce that every variable expansion is double-quoted (e.g., `"$1"`, `"$theme"`).
    *   **Benefit:** Prevents word splitting and globbing issues, making the scripts work correctly with filenames or inputs containing spaces or special characters.

5.  **Improve the `menu` Function:**
    *   **Action:** Refactor the `menu` function to handle cancellation internally. Instead of returning a magic string, it should return a non-zero exit code if the user cancels.
    *   **Example:** `if ! theme=$(menu "Theme" ...); then show_main_menu; return; fi`
    *   **Benefit:** Simplifies all calling code, removing repetitive `if [[ "$theme" == "CNCLD" || -z "$theme" ]]` blocks.

6.  **Abstract Repetitive Logic:**
    *   **Action:** Consolidate the various `install*` and `aur_install*` helper functions in `hypr-menu` into a single, more generic function that can handle packages, AUR packages, and optional post-install actions.
    *   **Benefit:** Reduces code duplication and centralizes installation logic.

7.  **Add Dependency Checks:**
    *   **Action:** At the start of critical scripts (`hypr-menu`, `hypr-pkg-install`), add checks to ensure required commands like `fzf`, `walker`, and `yay` are available on the `$PATH`.
    *   **Example:** `command -v fzf &>/dev/null || { echo "fzf not found" >&2; exit 1; }`
    *   **Benefit:** Provides clearer error messages to the user if the environment is not set up correctly.