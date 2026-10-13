#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if [ -n "${XDG_CONFIG_HOME:-}" ]; then
    DEST_BASE="$XDG_CONFIG_HOME/cera/plugins"
else
    DEST_BASE="$HOME/.config/cera/plugins"
fi

mkdir -p "$DEST_BASE"

for plugin_dir in "$SCRIPT_DIR"/*/; do
    name="$(basename "$plugin_dir")"
    if [ ! -f "$plugin_dir/plugin.json" ]; then
        continue
    fi
    echo "Installation de '$name' -> $DEST_BASE/$name"
    rm -rf "$DEST_BASE/$name"
    cp -r "$plugin_dir" "$DEST_BASE/$name"
done

echo "Termine. Un plugin n'est actif que s'il est dans \"plugins.enabled\" de config.json (sans cette liste, tous les plugins par defaut sont actifs)."
