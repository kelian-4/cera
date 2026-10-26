#!/usr/bin/env python3
import os
import re
import sys

_CONTROL_CHARS = re.compile("[\x00-\x08\x0a-\x1f\x7f-\x9f]")


def clean(text):
    """Remplace les caracteres de controle (dont ESC) par '?' : un nom de fichier ou un contenu
    ne doit pas pouvoir envoyer de sequence d'echappement au terminal."""
    return _CONTROL_CHARS.sub("?", text)


def printable(text):
    return clean(text) if sys.stdout.isatty() else text


def find_mode(name):
    needle = name.lower()
    found = False
    for root, dirs, files in os.walk("."):
        dirs.sort()
        for filename in sorted(files):
            if needle in filename.lower():
                sys.stdout.write(printable(os.path.normpath(os.path.join(root, filename))) + "\n")
                found = True
    return 0 if found else 1


def main():
    if len(sys.argv) > 1 and sys.argv[1] == "--find":
        sys.exit(find_mode(" ".join(sys.argv[2:])))
    sys.exit(2)


if __name__ == "__main__":
    main()
