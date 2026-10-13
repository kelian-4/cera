#!/usr/bin/env python3
import json
import sys

NAME = "python"


def send(message):
    sys.stdout.write(json.dumps(message) + "\n")
    sys.stdout.flush()


def render(text):
    segment = {"text": text, "color": "#82C0FF", "bold": True}
    send({"type": "render", "lines": [{"segments": [segment]}]})


def handle_key(key):
    if key == "q":
        send({"type": "close"})
    elif key == "o":
        send({"type": "open_file", "path": "/tmp/cera-plugin-test.txt"})
    elif key == "s":
        send({"type": "status", "message": f"status {NAME}", "kind": "error"})
    else:
        render(f"key {NAME} {key}")


def main():
    mode = "fullscreen"
    sys.stdout.write(f"debug: starting {NAME}\n")
    sys.stdout.flush()
    for raw in sys.stdin:
        try:
            message = json.loads(raw)
        except json.JSONDecodeError:
            continue
        kind = message.get("type")
        if kind == "init":
            mode = message.get("mode", "fullscreen")
            render(f"hello {NAME} {mode} {message['screen_rows']}x{message['screen_cols']}")
        elif kind == "resize":
            render(f"resize {NAME} {message['screen_rows']}x{message['screen_cols']}")
        elif kind == "key":
            handle_key(message["key"])
        elif kind == "click":
            render(f"click {NAME} {message['row']}")
        elif kind == "focus":
            render(f"focus {NAME} {'true' if message['focused'] else 'false'}")
        elif kind == "shutdown":
            return


if __name__ == "__main__":
    main()
