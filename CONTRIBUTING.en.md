[Français](CONTRIBUTING.md) | **English**

# Contributing to Cera

## Getting started

- Get the repository and switch to `dev`: `git clone https://github.com/kelian-4/cera && git switch dev`.
- Tools: Rust (CI uses **1.98.1**) with `rustfmt` and `clippy`. For the complete test suite.
- The code is organized as described in the "Repository layout" section of the [README](README.en.md).

## Translations

The interface texts are in `src/i18n/lang/`: one JSON file per language (`fr.json`, `en.json`, `es.json`), built into the program at compile time. Each file is a flat dictionary: the same key in every language, and the translated text.

```json
{
  "help.save": "save",
  "status.line_cut": "Line cut"
}
```

The key prefix tells where the text appears: `help` (help bar), `doc` (`Ctrl-D` documentation), `status` (status bar), `cli` (command line and `--help`), `ui` and `search`.

The language is chosen by the `language` key in `config.json`, otherwise by `LC_ALL` then `LANG` (`fr_FR.UTF-8` gives `fr`), otherwise English. A text missing from a language is shown in English, and the key name itself if it is missing from English too.

### Fixing a translation

Edit the value in the language file. No code to touch.

### Adding a text

1. Add the key to **all** language files. The English file is the reference: `cargo test i18n` fails if a language lacks a key that is present in `en.json`.
2. To insert a value into the text, write `%s` (only one per text). Keep it in every translation.
3. In the code, use `i18n.t("my.key")`, or `i18n.t1("my.key", value)` when the text contains a `%s`.

The current dictionaries do not use accents: keep this convention for French, Spanish and similar languages. For a language that cannot be written that way, open a discussion first.

### Adding a language

1. Copy `src/i18n/lang/en.json` to `src/i18n/lang/<code>.json` (two-letter code: `de`, `it`…), then translate the values. Change neither the keys nor the `%s`.
2. Register the language in `src/i18n/mod.rs`, in the `EMBEDDED_LANGUAGES` list:
   ```rust
   ("de", include_str!("lang/de.json")),
   ```
3. Run `cargo test i18n`: it checks that the file is valid and contains every key. To try it, put `"language": "de"` in `config.json` or run Cera with `LANG=de_DE.UTF-8`.
4. Add the language to the list in the "Translations" section of the [README](README.en.md).

### What is not translated yet

- the error messages for invalid command-line arguments (`describe_cli_error` in `src/main.rs`): French only;
- the option names and their examples in `cera --help` (`--afficher`, `[dossier]`, `fichier...`);
- the name, description and `help` field of plugins: a single text per manifest.

Translating them is welcome.

## Reporting a problem

See the "Reporting a problem" section of the [README](README.en.md). For a crash, include the program output and, if possible, the contents of the file being edited.

## Developer guide

| To… | Read |
|---|---|
| write a plugin or change the protocol | [PLUGINS.en.md](PLUGINS.en.md) |
| add a highlighting language | "Adding a highlighting language" below |
| the displayed texts | `src/i18n/lang/{fr,en,es}.json` |

## Branches and pull requests

- `dev`: working branch. Pull requests target `dev`.
- A pull request does one thing. Do not mix cosmetic changes to unrelated files into it.
- Do not change a behavior, a shortcut or an option unless it was requested. If in doubt, open a discussion first.

## Tests

Before each commit your changes. (Yes, I really have to spell it out)

- Unit tests are in the modules (`#[cfg(test)]`), integration tests are in `tests/`.
- A test that must fail without your fix is checked like this: remove the fix, see it fail, put it back.
- `CERA_REQUIRE_PLUGIN_LANGS=1 cargo test` makes all external tools mandatory (`python3`, `node`, `cc`, `c++`, `rustc`). Without this variable, the test for a language whose tool is missing is skipped. CI enables it.

### Testing interactive behavior

A key, a click, a paste, a rendering or a plugin's lifecycle are **never done based on reading the code alone**: they are checked in a terminal. `tests/escape_key.rs` and `tests/paste.rs` give the model: a Python script runs Cera in a `pty`, sends bytes, then reads the screen or the saved file.

- Run these scripts in the background, with a safety alarm (`signal.alarm`) in the script, and always kill the Cera process at the end.
- Read the **last frame** of the screen (`text.split('\x1b[H')[-1]`), not the whole stream.
- Plugins are read from `~/.config/cera/plugins`: after changing a plugin, run `./plugins/install.sh` again (or `HOME=<test directory> bash plugins/install.sh`) before testing.

## Code rules

- **No explanatory comments** in the code: clear variable and function names replace them. Only shebangs are kept. `.md` files are not concerned.
- **Every text shown to the user** goes through `src/i18n/lang/{fr,en,es}.json` (flat keys). Add the key to all three files. The current dictionaries do not use accents: keep this convention.

## Adding a highlighting language

Each language has its own file `src/syntax/<id>_syntax.rs`.

1. Create the file based on an existing language (extensions, keywords, types, constants, comments, multi-line strings, flags…).
2. Declare it in `src/syntax/mod.rs` (`pub mod` and order in `build_database`). An extension must belong to a single language; a pattern without a dot (`Makefile`) is searched in the whole name.
3. Add its entry to `plugins/pretty-cat/languages.json`: this is the copy read by `pretty-cat`.
4. Add a sample in `src/syntax/language_samples.rs`: one or more lines and the expected colors of a few words.
5. Run `cargo test syntax`: the parity tests (`src/syntax/parity_tests.rs`) check that `languages.json` is identical to the Rust database and that `pretty-cat` colors like the editor.

A keyword followed by `|` gets the second keyword color (built-in functions and constants).

The engine exists in **two versions that must stay identical**: `src/syntax/engine.rs` (editor) and `plugins/pretty-cat/main.py` (plugin). Any change to one is carried over to the other; the parity tests fail otherwise.

## Adding a plugin

You are free to create your own plugins and to submit them to the project or not. But the project's plugins are under the same license as the project and remain the property of the project.

Choose a free shortcut. The `Ctrl` letters already in use are `a c d e f h k l n p q r s v x y z`; `b g o t u w` remain. `i`, `j` and `m` cannot be used (Tab, line feed, Enter). Never reuse an existing shortcut.
