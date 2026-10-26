#!/usr/bin/env python3
import difflib
import re
import sys

RESET = "\x1b[0m"
BOLD = "\x1b[1m"
RED = "\x1b[31m"
GREEN = "\x1b[32m"
CYAN = "\x1b[36m"


_CONTROL_CHARS = re.compile("[\x00-\x08\x0a-\x1f\x7f-\x9f]")


def clean(text):
    """Remplace les caracteres de controle (dont ESC) par '?' : un nom de fichier ou un contenu
    ne doit pas pouvoir envoyer de sequence d'echappement au terminal."""
    return _CONTROL_CHARS.sub("?", text)


def read_lines(path):
    with open(path, "r", encoding="utf-8", errors="replace") as f:
        return f.read().splitlines()


def colorize(line):
    if line.startswith("---") or line.startswith("+++"):
        return BOLD + line + RESET
    if line.startswith("@@"):
        return CYAN + line + RESET
    if line.startswith("+"):
        return GREEN + line + RESET
    if line.startswith("-"):
        return RED + line + RESET
    return line


def diff_mode(paths):
    reference = paths[0]
    try:
        reference_lines = read_lines(reference)
    except OSError as e:
        sys.stderr.write(f"cera -d: {reference}: {e.strerror or e}\n")
        return 2
    exit_code = 0
    for path in paths[1:]:
        try:
            other_lines = read_lines(path)
        except OSError as e:
            sys.stderr.write(f"cera -d: {path}: {e.strerror or e}\n")
            exit_code = 2
            continue
        changes = list(
            difflib.unified_diff(
                reference_lines, other_lines, fromfile=reference, tofile=path, lineterm=""
            )
        )
        for line in changes:
            sys.stdout.write(colorize(clean(line)) + "\n")
        if changes and exit_code == 0:
            exit_code = 1
    return exit_code


def main():
    if len(sys.argv) > 1 and sys.argv[1] == "--diff":
        sys.exit(diff_mode(sys.argv[2:]))
    sys.exit(2)


if __name__ == "__main__":
    main()
