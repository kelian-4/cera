[Français](../../README.md) | **English**

# Cera

[![CI](https://github.com/kelian-4/cera/actions/workflows/ci.yml/badge.svg)](https://github.com/kelian-4/cera/actions/workflows/ci.yml)

Cera is a text editor for the terminal, written in Rust.

<details open>
<summary><h2>What is Cera?</h2></summary>

Cera is an easy-to-pick-up editor: classic `Ctrl` shortcuts (`Ctrl-S` saves, `Ctrl-Q` quits), mouse support, tabs, and syntax highlighting. It is a more modern and intuitive alternative to classic terminal editors,
with a plugin system whose plugins can be written in C++, C, Rust, JS or Python.

The full list of features and shortcuts is in the editor: `Ctrl-D`, or `cera --help` for the options.

</details>

<details open>
<summary><h2>Installation</h2></summary>

Cera targets Linux (x86_64 and aarch64). You need an ANSI-compatible terminal; 24-bit colors are recommended (otherwise set `true_color` to `false`, see [Configuration](#configuration)). The bundled plugins require `python3`.

### With Nix

```
nix run github:kelian-4/cera       # try it without installing
nix profile install github:kelian-4/cera   # install for the current user
nix build                          # build into ./result (from the repository)
nix develop                        # development shell (from the repository)
```

In a NixOS configuration that uses flakes:

```nix
inputs.cera = {
  url = "github:kelian-4/cera";
  inputs.nixpkgs.follows = "nixpkgs";
};

# in the list of modules:
{ environment.systemPackages = [ inputs.cera.packages.${system}.default ]; }
```

The Nix package bundles `python3` for the bundled plugins.

### From a release (Debian, Ubuntu, Fedora…)

The packages (x86_64 and aarch64) are on the [releases page](https://github.com/kelian-4/cera/releases):

```
sudo apt install ./cera_0.1.0_amd64.deb         # Debian, Ubuntu (arm64: cera_0.1.0_arm64.deb)
sudo dnf install ./cera-0.1.0-1.x86_64.rpm      # Fedora (aarch64: cera-0.1.0-1.aarch64.rpm)
```

The archive contains the binary and the plugins:

```
tar -xzf cera-0.1.0-linux-x86_64.tar.gz
cd cera-0.1.0-linux-x86_64
./cera                                   # try it in place
install -Dm755 cera ~/.local/bin/cera    # install (make sure ~/.local/bin is in your PATH)
./plugins/install.sh                     # copy the plugins to ~/.config/cera/plugins
```

Each release also contains a `SHA256SUMS` file to verify the downloads:

```
sha256sum -c SHA256SUMS --ignore-missing
```

### On Arch Linux

A `PKGBUILD` is provided in `packaging/aur`. It builds Cera from the `v0.1.0` release archive and runs the tests:

```
git clone https://github.com/kelian-4/cera
cd cera/packaging/aur
makepkg -si
```

### From source

You need Rust (stable toolchain, with `cargo`) and `git`. No other build dependency.

```
git clone https://github.com/kelian-4/cera
cd cera
cargo build --release
cargo test                                       # optional
install -Dm755 target/release/cera ~/.local/bin/cera
./plugins/install.sh                             # copies the plugins to ~/.config/cera/plugins
```

`cargo install --path .` also installs the binary (in `~/.cargo/bin`) but not the plugins: run `./plugins/install.sh` afterwards.

### First launch

On first launch, Cera creates `~/.config/cera/config.json` (or `$XDG_CONFIG_HOME/cera/config.json`) and, when it finds the bundled plugins next to itself (`../share/cera/plugins` or `plugins/`), copies them to `~/.config/cera/plugins` and enables them: see [PLUGINS.en.md](PLUGINS.en.md). Check the installation with:

```
cera --version
```

### Updating and uninstalling

- **Updating**: install the new version with the same method. Your `config.json` is never modified: an option added by the new version keeps its default value until you write it. The bundled plugins are replaced whenever the version changes (see [PLUGINS.en.md](PLUGINS.en.md)).
- **Uninstalling**: `sudo apt remove cera`, `sudo dnf remove cera` or `sudo pacman -R cera` depending on the method; for a manual install, delete `~/.local/bin/cera`.
- **User data**: the configuration and the copied plugins stay in `~/.config/cera`; delete that folder to start from scratch.

</details>

<details open>
<summary><h2>Usage</h2></summary>

### Without plugin

```
cera [file]
```

### With plugin

```
cera [options] [file ...]
```

| Option | Effect |
|---|---|
| `-e`, `--explorer [directory]` | Opens the file explorer on a directory |
| `-a`, `--afficher file...` | Shows files read-only, with colors |
| `-f`, `--find name` | Finds files by name |
| `-g`, `--grep pattern [path...]` | Searches for text in files and directories |
| `-d`, `--diff file1 file2...` | Compares each file to the first one |
| `-h`, `--help` | Shows the help and the documentation of the active plugins |
| `-v`, `--version` | Shows the version |

The `-a`, `-f`, `-g` and `-d` options are provided by plugins: they only work if the plugin is enabled.

</details>

<details open>
<summary><h2>Documentation</h2></summary>

- **In the editor**: `Ctrl-D` opens the full documentation (shortcuts, active plugins); `Ctrl-H` shows or hides the help bar.
- **On the command line**: `cera --help`.
- **Plugins**: [PLUGINS.en.md](PLUGINS.en.md) explains how to use and write them.

</details>

<details open>
<summary><h2>Configuration</h2></summary>

On first launch, Cera creates `~/.config/cera/config.json` (or `$XDG_CONFIG_HOME/cera/config.json`). This file contains every available key with its default value. Edit it directly and restart Cera. A missing key keeps its default value; a file that is not valid JSON is ignored and all default values apply.

### General options

| Key | Values | Default | Effect |
|---|---|---|---|
| `tab_stop` | integer ≥ 1 | `8` | Display width of a tab, in columns |
| `quit_times` | integer | `1` | With `1`, quitting (`Ctrl-Q`) with unsaved changes asks for confirmation; with `0`, Cera quits without asking |
| `true_color` | `true` / `false` | `true` | `true`: 24-bit colors; `false`: 256 colors, for terminals that do not display 24-bit |
| `show_line_numbers` | `true` / `false` | `true` | Shows the line number column |
| `mouse_enabled` | `true` / `false` | `true` | Enables the mouse (cursor placement, selection, wheel) |
| `language` | `"fr"`, `"en"`, `"es"` or `null` | `null` | Interface language; `null`: system language, then English (see [Translations](#translations)) |
| `indent_guides` | `true` / `false` | `false` | Indentation guides, **experimental**: see [Experimental features](#experimental-features) |
| `format_on_save` | `true` / `false` | `false` | Formatting on save, **experimental**: see [Experimental features](#experimental-features) |

### Plugins

| Key | Values | Default | Effect |
|---|---|---|---|
| `plugins.enabled` | list of identifiers | `["file-explorer", "pretty-cat", "finder", "diff", "grep"]` | Active plugins. An identifier is the name of the plugin's folder. If the key is absent, all bundled plugins are active |
| `plugins.directory` | path or `null` | `null` | Folder to look for plugins in; `null`: `~/.config/cera/plugins` (or `$XDG_CONFIG_HOME/cera/plugins`) |

### Theme

Colors are set in the `theme` section, in `"#RRGGBB"` format. An invalid color is ignored.

| Key | Default | Used for |
|---|---|---|
| `normal` | `#E8EDF2` | regular text |
| `comment` | `#8FA3B8` | comments |
| `keyword1` | `#82C0FF` | main keywords |
| `keyword2` | `#8CF5D6` | secondary keywords |
| `string` | `#A8E6CF` | strings |
| `number` | `#C9A8FF` | numbers |
| `type` | `#FFD08A` | types |
| `function` | `#F9A8D4` | functions |
| `constant` | `#FF8FA3` | constants |
| `operator` | `#B4C2D4` | operators |
| `preprocessor` | `#D4E27A` | preprocessor directives |
| `variable` | `#FFC6A5` | variables |
| `escape` | `#FFB86B` | escape sequences |
| `match` | `#FFFF66` | search results |
| `line_number_fg` | `#7BA3C9` | line numbers |
| `indent_guide` | `#3F4E63` | indentation guides |
| `indent_guide_active` | `#7BA3C9` | guide of the block containing the cursor |
| `error_fg` | `#FF8A8A` | error messages |
| `success_fg` | `#9CE89C` | success messages |

Example:

```json
{
  "tab_stop": 4,
  "true_color": false,
  "indent_guides": true,
  "theme": { "keyword1": "#ff5555" }
}
```

</details>

<details open>
<summary><h2>Experimental features</h2></summary>

Two features are still **experimental**: they are disabled by default, their behavior may still change and they can be wrong on some files. Enable them in `config.json`.

### Indentation guides

```json
{ "indent_guides": true }
```

Cera draws a vertical `│` line at each indentation level, for every supported language.

- The guide of the block that contains the cursor is drawn brighter, over the whole extent of the block. On a header line (`fn main() {`, `if x:`), the block it opens is highlighted.
- The width of a level is detected for each file: a tab if the file is indented with tabs, otherwise the most frequent indentation step. A file with no indented line gets the usual value for its language (2 spaces for YAML, JSON, HTML, Ruby, JavaScript, TypeScript…, 4 otherwise).
- Blank lines extend the guides of the block; the text of multi-line comments and strings does not break them.
- Colors are set with `theme.indent_guide` and `theme.indent_guide_active`.

Limits: detection is heuristic and can be wrong on a file with irregular indentation. The text is never modified, only the display changes.

### Formatting on save

```json
{ "format_on_save": true }
```

On every `Ctrl-S`, Cera reformats the file before writing it. It is not a full formatter like Prettier: it only changes **whitespace**, it neither splits nor joins long lines and does not touch quotes or the inside of a line.

- **All languages**: removes trailing whitespace, keeps at most one blank line in a row (two in Python) and removes blank lines at the start and end of the file.
- **Re-indentation** based on the structure of the file, for:
  - bracket languages: C, C++, C#, Java, JavaScript, TypeScript, Go, Rust, PHP, Kotlin, Dart, D, Groovy, Solidity, Zig, CSS, SCSS, JSON, Protobuf, GraphQL, HCL;
  - keyword languages: Ruby, Crystal, Lua, Julia, shell;
  - indentation-based languages: Python (4 spaces), YAML, Nim, CoffeeScript (2 spaces);
  - HTML and XML.
- Other languages only get the whitespace cleanup. Markdown, diff and LaTeX are not touched.
- Indentation follows the file's own style (tabs or spaces, detected width), except Go (tabs), Python and YAML (spaces). In C and C++, arguments aligned under the parenthesis are kept.
- The content of multi-line strings, heredocs and `pre`, `script` and `style` blocks is never modified.
- Formatting can be undone in one step with `Ctrl-Z`, and the cursor stays in place.

Safeguards: formatting is silently skipped when the file cannot be understood reliably: unbalanced brackets, tags or blocks, JSX, PHP mixed with HTML, an unterminated string, HTML with optional end tags. In those cases only the whitespace cleanup is done, or nothing. A formatting that would change anything other than whitespace is abandoned.

Since the feature is experimental, use it on versioned files (for example with Git) so you can review the changes.

</details>

<details open>
<summary><h2>Translations</h2></summary>

Cera's interface is available in French, English and Spanish. The language is chosen in this order:

1. the `language` key in `config.json` (`"fr"`, `"en"` or `"es"`);
2. the system language (the `LC_ALL`, then `LANG` variables);
3. English.

A text that is missing from a translation is shown in English. The description of plugins (`Ctrl-D`, `cera --help`) is in the language of their manifest: it is not translated.

To fix a translation or add a language, see "Translations" in [CONTRIBUTING.en.md](CONTRIBUTING.en.md).

</details>

<details open>
<summary><h2>Repository layout</h2></summary>

```
src/                  editor source code
├─ editor/            main loop, buffers, undo, clipboard
├─ terminal/          raw mode, key reading, mouse, bracketed paste
├─ syntax/            highlighting engine and one file per language
├─ format/            formatting on save (experimental)
├─ plugin/            protocol, subprocesses, plugin discovery
├─ view/              screen rendering, built-in help
├─ io/                configuration, file reading/writing, JSON
└─ i18n/              translations (fr, en, es)
plugins/              bundled plugins (see PLUGINS.en.md)
tests/                integration tests and example plugins in 5 languages
flake.nix             Nix package and development environment
```

</details>

<details open>
<summary><h2>Contributing</h2></summary>

See [CONTRIBUTING.en.md](CONTRIBUTING.en.md).

</details>

<details open>
<summary><h2>Reporting a problem</h2></summary>

First check whether the problem has already been reported in the [issues](https://github.com/kelian-4/cera/issues). Otherwise, open one and include:

- your system and your terminal;
- the Cera version (the commit or the package version);
- the relevant contents of your `config.json` and the list of active plugins (`cera --help`);
- the steps to reproduce the problem.

</details>

<details open>
<summary><h2>License</h2></summary>

Cera is distributed under the GNU GPL version 3 or later: see [LICENSE](../../LICENSE).

</details>
