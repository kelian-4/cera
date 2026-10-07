#!/usr/bin/env python3
import os
import sys


def is_binary(path):
    with open(path, "rb") as f:
        return b"\0" in f.read(8192)


def grep_file(path, pattern):
    found = False
    with open(path, "r", encoding="utf-8", errors="replace") as f:
        for number, line in enumerate(f, 1):
            if pattern in line:
                text = line.rstrip("\r\n")
                sys.stdout.write(f"{path}:{number}:{text}\n")
                found = True
    return found


def grep_safely(path, pattern):
    try:
        if is_binary(path):
            return False
        return grep_file(path, pattern)
    except BrokenPipeError:
        raise
    except OSError as e:
        sys.stderr.write(f"cera -g: {path}: {e.strerror or e}\n")
        return False


def grep_path(path, pattern):
    if not os.path.isdir(path):
        return grep_safely(path, pattern)
    found = False
    for root, dirs, files in os.walk(path):
        dirs.sort()
        for filename in sorted(files):
            if grep_safely(os.path.normpath(os.path.join(root, filename)), pattern):
                found = True
    return found


def grep_mode(pattern, paths):
    found = False
    for path in paths or ["."]:
        if grep_path(path, pattern):
            found = True
    return 0 if found else 1


def main():
    if len(sys.argv) > 1 and sys.argv[1] == "--grep":
        try:
            code = grep_mode(sys.argv[2], sys.argv[3:])
        except BrokenPipeError:
            os.dup2(os.open(os.devnull, os.O_WRONLY), sys.stdout.fileno())
            code = 0
        sys.exit(code)
    sys.exit(2)


if __name__ == "__main__":
    main()
