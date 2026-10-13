[Français](../README.md) | **English**

# Cera

[![CI](https://github.com/kelian-4/cera/actions/workflows/ci.yml/badge.svg)](https://github.com/kelian-4/cera/actions/workflows/ci.yml)

Cera is a text editor for the terminal, written in Rust.

## What is Cera?

Cera is an easy-to-pick-up editor: classic `Ctrl` shortcuts (`Ctrl-S` saves, `Ctrl-Q` quits), mouse support, tabs, and syntax highlighting. It is a more modern and intuitive alternative to classic terminal editors,
with a plugin system whose plugins can be written in C++, C, Rust, JS or Python.

The full list of features and shortcuts is in the editor: `Ctrl-D`, or `cera --help` for the options.

## Installation

### With Nix

```
nix run github:kelian-4/cera       # try it without installing
nix build                          # build into ./result
nix develop                        # development shell
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
Plugins are installed in `~/.config/cera/plugins` or in `plugins.directory` from `config.json`: see [PLUGINS.en.md](PLUGINS.en.md).

### From source

You only need Rust installed:

```
cargo build --release
./target/release/cera
./plugins/install.sh      # copies the plugins to ~/.config/cera/plugins
```

The bundled plugins require `python3`.

## Usage

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

The `-a`, `-f`, `-g` and `-d` options are provided by plugins: they only work if the plugin is enabled.

## Documentation

- **In the editor**: `Ctrl-D` opens the full documentation (shortcuts, active plugins); `Ctrl-H` shows or hides the help bar.
- **On the command line**: `cera --help`.
- **Plugins**: [PLUGINS.en.md](PLUGINS.en.md) explains how to use and write them.

## Configuration

On first launch, Cera creates `~/.config/cera/config.json`. This file contains every available key with its default value: tab width, colors, interface language, enabled plugins, mouse, line numbers. Edit it directly; a missing key keeps its default value.

## Translations

Cera's interface is available in French, English and Spanish. The language is chosen in this order:

1. the `language` key in `config.json` (`"fr"`, `"en"` or `"es"`);
2. the system language (the `LC_ALL`, then `LANG` variables);
3. English.

A text that is missing from a translation is shown in English. The description of plugins (`Ctrl-D`, `cera --help`) is in the language of their manifest: it is not translated.

To fix a translation or add a language, see "Translations" in [CONTRIBUTING.en.md](CONTRIBUTING.en.md).

## Repository layout

```
src/                  editor source code
├─ editor/            main loop, buffers, undo, clipboard
├─ terminal/          raw mode, key reading, mouse, bracketed paste
├─ syntax/            highlighting engine and one file per language
├─ plugin/            protocol, subprocesses, plugin discovery
├─ view/              screen rendering, built-in help
├─ io/                configuration, file reading/writing, JSON
└─ i18n/              translations (fr, en, es)
plugins/              bundled plugins (see PLUGINS.en.md)
tests/                integration tests and example plugins in 5 languages
flake.nix             Nix package and development environment
```

## Contributing

See [CONTRIBUTING.en.md](CONTRIBUTING.en.md).

## Reporting a problem

First check whether the problem has already been reported in the [issues](https://github.com/kelian-4/cera/issues). Otherwise, open one and include:

- your system and your terminal;
- the Cera version (the commit or the package version);
- the relevant contents of your `config.json` and the list of active plugins (`cera --help`);
- the steps to reproduce the problem.

## License

Cera is distributed under the GNU GPL version 3 or later: see [LICENSE](../../LICENSE).
