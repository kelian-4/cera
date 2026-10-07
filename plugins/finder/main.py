#!/usr/bin/env python3
import os
import sys


def find_mode(name):
    needle = name.lower()
    found = False
    for root, dirs, files in os.walk("."):
        dirs.sort()
        for filename in sorted(files):
            if needle in filename.lower():
                sys.stdout.write(os.path.normpath(os.path.join(root, filename)) + "\n")
                found = True
    return 0 if found else 1


def main():
    if len(sys.argv) > 1 and sys.argv[1] == "--find":
        sys.exit(find_mode(" ".join(sys.argv[2:])))
    sys.exit(2)


if __name__ == "__main__":
    main()
