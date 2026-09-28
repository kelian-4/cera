#!/usr/bin/env bash
# Installe les plugins fournis avec Cera dans le repertoire de plugins de
# l'utilisateur (par defaut ~/.config/cera/plugins, ou $XDG_CONFIG_HOME/cera/plugins
# si defini), pour qu'ils soient decouverts par l'editeur.
#
# Usage : ./plugins/install.sh
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

echo "Termine. Les plugins installes sont actives par defaut dans config.json (champ \"plugins.enabled\")."
