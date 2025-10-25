#!/bin/bash
#
# Configures mise for Ruby development, ensuring compatibility with gcc-14.

# Exit immediately if a command exits with a non-zero status.
set -euo pipefail

#######################################
# Configures mise settings for Ruby.
#######################################
main() {
  if command -v mise &>/dev/null; then
    echo "Configuring mise for Ruby..."

    # Set Ruby build options to use gcc-14 for compatibility.
    mise settings set ruby.ruby_build_opts "CC=gcc-14 CXX=g++-14"

    # Configure mise to automatically trust and use .ruby-version files.
    mise settings add idiomatic_version_file_enable_tools ruby

    echo "mise Ruby configuration complete."
  else
    echo "Warning: mise command not found. Skipping Ruby configuration."
  fi
}

main "$@"
