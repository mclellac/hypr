#!/bin/bash

hypr_MIGRATIONS_STATE_PATH=~/.local/state/hypr/migrations
mkdir -p $hypr_MIGRATIONS_STATE_PATH

for file in ~/.local/share/hypr/migrations/*.sh; do
  touch "$hypr_MIGRATIONS_STATE_PATH/$(basename "$file")"
done
