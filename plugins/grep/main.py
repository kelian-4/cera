#!/usr/bin/env python3
import json
import os
import sys


def send(msg):
    sys.stdout.write(json.dumps(msg, ensure_ascii=False) + "\n")
    sys.stdout.flush()


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


def grep_mode(pattern):
    found = False
    for root, dirs, files in os.walk("."):
        dirs.sort()
        for filename in sorted(files):
            path = os.path.normpath(os.path.join(root, filename))
            try:
                if is_binary(path):
                    continue
                if grep_file(path, pattern):
                    found = True
            except BrokenPipeError:
                raise
            except OSError as e:
                sys.stderr.write(f"cera -g: {path}: {e.strerror or e}\n")
    return 0 if found else 1


def main():
    if len(sys.argv) > 1 and sys.argv[1] == "--grep":
        try:
            code = grep_mode(" ".join(sys.argv[2:]))
        except BrokenPipeError:
            os.dup2(os.open(os.devnull, os.O_WRONLY), sys.stdout.fileno())
            code = 0
        sys.exit(code)
    sys.stdin.readline()
    send({"type": "status", "message": "Utilisez : cera -g motif", "kind": "info"})
    send({"type": "close"})


if __name__ == "__main__":
    main()
