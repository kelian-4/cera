[Français](../../PLUGINS.md) | **English**

# Plugins

A plugin is an **independent executable**, in any language, launched by Cera as a subprocess. Cera and the plugin exchange JSON messages on the plugin's standard input and output.

This document explains how to use one, then how to write one.

## Quick start

Create the folder `~/.config/cera/plugins/hello/` with two files.

`plugin.json`:

```json
{
  "name": "Hello",
  "description": "Minimal example",
  "help": "Ctrl-W: shows a message, q to close",
  "command": "python3",
  "args": ["main.py"],
  "shortcut": "^W"
}
```

`main.py`:

```python
#!/usr/bin/env python3
import json
import sys


def send(message):
    sys.stdout.write(json.dumps(message) + "\n")
    sys.stdout.flush()


for raw in sys.stdin:
    try:
        message = json.loads(raw)
    except json.JSONDecodeError:
        continue
    kind = message.get("type")
    if kind == "init":
        send({"type": "render", "lines": [{"segments": [{"text": "Hello, q to close", "bold": True}]}]})
    elif kind == "key" and message["key"] == "q":
        send({"type": "close"})
    elif kind == "shutdown":
        break
```

Add `"hello"` to `plugins.enabled` in `config.json`, restart Cera and press `Ctrl-W`. That's it.

## Three ways to use a plugin

| Mode | Declaration | Behavior |
|---|---|---|
| Fullscreen | `"mode": "fullscreen"` (default value) | The plugin's view replaces the whole screen, the editor is suspended |
| Panel | `"mode": "panel"` | Persistent panel on the left, the editor stays usable on the right |
| Command line | fixed Cera option (`-a`, `-f`, `-g`, `-d`) | Cera launches the plugin with `--cat`, `--find`, `--grep` or `--diff` and passes it the arguments |

The command-line options are **built into Cera**: a third-party plugin cannot add one without changing Cera's code. For the four bundled plugins, Cera runs `<command> <manifest args> --cat|--find|--grep|--diff <user arguments>`, lets the plugin write directly to standard output, and exits with the plugin's exit code.

## Installation and activation

On first launch, when `config.json` does not exist yet, Cera creates it with the bundled plugins in `plugins.enabled` and copies them to `~/.config/cera/plugins/` (Nix, `.deb`, `.rpm` packages, `.tar.gz` archive). An existing `config.json` is never modified and a plugin that is already present is never overwritten.

From source, or after changing a plugin:

```
./plugins/install.sh
```

copies each plugin from the repository to `~/.config/cera/plugins/` (or `$XDG_CONFIG_HOME/cera/plugins/`). Cera looks for plugins in this folder, or in the one set by `plugins.directory` (`config.json`).

A plugin is only active if it is in `plugins.enabled`. If this list is absent, all bundled plugins are active; an explicit list restricts the active plugins. An existing `config.json` is never modified by an update: add the new plugins to it by hand.


## The `plugin.json` manifest

| Field | Role |
|---|---|
| `name` | Displayed name (`Ctrl-D` doc, `--help`) |
| `description` | One sentence |
| `help` | **Minimal documentation, mandatory for bundled plugins**: one short line (ideally under 60 characters) saying how to use it. It appears in the "Plugins" category of the `Ctrl-D` doc and in `cera --help`. If absent, `description` is used. A single text, without translation |
| `command` | The executable. A plain name (`python3`, `node`) is searched in the `PATH`; a path containing `/` is resolved relative to the plugin folder |
| `args` | Arguments passed to `command`. An argument that does not start with `/` is resolved relative to the plugin folder (typically `"main.py"`) |
| `shortcut` | `Ctrl` shortcut that launches the plugin, for example `"^E"`. Optional: without it, the plugin can only be used through a command-line option |
| `mode` | `"fullscreen"` (default) or `"panel"` |

A plugin's identifier is the name of its folder: it is the one you put in `plugins.enabled`.

**Shortcuts**: the `Ctrl` letters already taken are `a c d e f h k l n p q r s v x y z`. `b g o t u w` remain free. `i`, `j` and `m` cannot be used (Tab, line feed, Enter).

## Messages from Cera to the plugin

- `{"type":"init","mode":"fullscreen","screen_rows":24,"screen_cols":80,"args":["/path"]}`
  Sent once at startup. In panel mode, the message also contains `"panel_width"` (30 by default). `args` contains a single element: the file open in Cera, or the current directory if no file is open.
- `{"type":"key","key":"Down","focused":true}`
  A key press. `key` is `Up`, `Down`, `Left`, `Right`, `PageUp`, `PageDown`, `Home`, `End`, `Delete`, `Backspace`, `Enter`, `Tab`, `Escape`, or a single character (`"a"`, `"3"`, `"é"`…). Combinations with `Ctrl` are not forwarded.
- `{"type":"click","row":3}` (panel mode): mouse click in the panel. `row` is the index of the clicked line **in the list of lines the plugin sent**: 0 is the first line of its `render`. Do not subtract anything for a possible header: there is none in this area.
- `{"type":"resize","screen_rows":30,"screen_cols":100}`: the terminal changed size.
- `{"type":"focus","focused":false}` (panel mode): the panel gains or loses focus.
- `{"type":"shutdown"}`: Cera asks the plugin to terminate cleanly.

## Messages from the plugin to Cera

- `{"type":"render","lines":[...]}`: replaces the plugin's display. A line is `{"segments":[...]}`; a segment is `{"text":"...","color":"#RRGGBB","bold":true}` (`color` and `bold` are optional; without `color`, the text takes the theme's normal color). In panel mode, the message can contain `"panel_width"` to request another width (bounded between 10 columns and the screen width minus 10).
- `{"type":"open_file","path":"/path/file"}`: asks Cera to open this file for editing (closes the fullscreen view; removes focus from the panel). Only the path is transmitted: it is not possible to specify a line.
- `{"type":"status","message":"...","kind":"info"}`: shows a message in the status bar. `kind` is `info` or `error`.
- `{"type":"close"}`: closes the plugin's view (fullscreen) or the panel.

Any output line that is not recognized JSON is silently ignored. What the plugin writes to its error output is not read by Cera: use it for debugging.

## Lifecycle

### Fullscreen

1. Cera launches the plugin and sends it `init`.
2. The plugin sends `render` messages, which replace the whole screen.
3. Cera forwards every key (`key`) and every size change (`resize`) to it.
4. The view ends when the plugin sends `open_file` or `close`, or when the user presses `Escape` (which always closes the view, even if the plugin no longer responds).
5. Cera sends `shutdown`, then terminates the process after a short delay if needed.

### Panel

1. The shortcut opens the panel, which takes focus. It does not close on its own.
2. Keys are only forwarded while the panel has focus. `Escape` removes focus (the plugin receives `focus` with `false`) and does not close the panel.
3. A click in the panel gives it focus (`focus` with `true`) then sends `click`. A click in the editor removes its focus.
4. The shortcut gives focus back to an open panel that has none, and **closes** it if it already has focus.
5. `open_file` opens the file and removes focus, but leaves the panel open. `close` closes the panel.
6. If the process ends, the panel closes and the status bar reports the error.

Only one panel is open at a time: opening another closes the first. The width is 30 columns by default.

## Tips

- Flush the output after every message; otherwise Cera sees nothing.
- Ignore messages whose type you do not know (the protocol may grow).
- Quit when you receive `shutdown` or when the input closes.
- Avoid third-party libraries: the bundled plugins only use Python's standard library.

## Tested languages

The protocol is verified automatically (`tests/plugin_languages.rs`) with an example plugin in each language below, in fullscreen mode and in panel mode:

| Language | Example | Launched with |
|---|---|---|
| Python | `tests/plugin_examples/hello.py` | `python3` |
| JavaScript | `tests/plugin_examples/hello.js` | `node` |
| C | `tests/plugin_examples/hello.c` | binary compiled with `cc` |
| C++ | `tests/plugin_examples/hello.cpp` | binary compiled with `c++` |
| Rust | `tests/plugin_examples/hello.rs` | binary compiled with `rustc` |

Each example handles `init`, `key` (including a non-ASCII character), `resize`, `click`, `focus` and `shutdown`, and emits `render` (with color and bold), `status`, `open_file` and `close`. It also writes a line that is not JSON at startup, to check that it is ignored. They are good starting points.

## Testing a plugin

In isolation, without launching Cera:

```
printf '%s\n%s\n' \
  '{"type":"init","mode":"panel","screen_rows":10,"screen_cols":30,"panel_width":28,"args":["/tmp"]}' \
  '{"type":"shutdown"}' | python3 main.py
```

Every output line must be valid JSON. For interactive behavior, test with Cera in a pseudo-terminal (see [CONTRIBUTING.en.md](CONTRIBUTING.en.md)). Run `./plugins/install.sh` again after every change: Cera runs the copy in `~/.config/cera/plugins`, not the one in the repository.

## Bundled plugins

| Plugin | Usage | Description |
|---|---|---|
| `file-explorer` | `Ctrl-E`, `cera -e [directory]` | Tree panel: Up/Down to move, `Enter` to open a file or fold a directory, Right/Left to expand/collapse |
| `pretty-cat` | `cera -a file...` | Shows files read-only, with line numbers and highlighting for the editor's 70 languages. Its language definitions are in `languages.json`, next to `main.py` |
| `finder` | `cera -f name` | Recursively finds, from the current directory, the files whose name contains `name` (case-insensitive); relative paths, one per line; exit code 1 if nothing is found |
| `grep` | `cera -g pattern [path...]` | Searches for literal, case-sensitive text in files and directories (recursive); output `path:line:text`; binary files ignored; exit code 1 if nothing is found |
| `diff` | `cera -d f1 f2 ...` | Compares each file to the first one, as a colored unified diff; exit code 0 (identical), 1 (differences) or 2 (read error) |

## Current limitations

- A plugin cannot modify the text of the open file, nor change the theme.
- `open_file` only transmits a path, without a line number.
- Only one fullscreen view and one panel at a time.
- The command-line options are fixed.
