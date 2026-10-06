#!/usr/bin/env python3
import json
import os
import sys
COLOR_DIR = "#82C0FF"
COLOR_FILE = "#E8EDF2"
COLOR_HEADER = "#8CF5D6"
COLOR_SELECTED_BG_MARK = "#FFFF66"
COLOR_ERROR = "#FF8A8A"
IGNORED_DIRS = {".git", "node_modules", "__pycache__", "target", ".venv"}
def send(msg):
    sys.stdout.write(json.dumps(msg, ensure_ascii=False) + "\n")
    sys.stdout.flush()
def read_message():
    line = sys.stdin.readline()
    if not line:
        return None
    line = line.strip()
    if not line:
        return {}
    try:
        return json.loads(line)
    except json.JSONDecodeError:
        return {}
class Node:
    __slots__ = ("path", "name", "is_dir", "depth", "expanded")
    def __init__(self, path, name, is_dir, depth):
        self.path = path
        self.name = name
        self.is_dir = is_dir
        self.depth = depth
        self.expanded = False
def list_dir(path):
    try:
        entries = sorted(
            os.listdir(path),
            key=lambda n: (not os.path.isdir(os.path.join(path, n)), n.lower()),
        )
    except OSError:
        return []
    result = []
    for name in entries:
        if name in IGNORED_DIRS or name.startswith('.'):
            continue
        full = os.path.join(path, name)
        result.append((full, name, os.path.isdir(full)))
    return result
class Explorer:
    def __init__(self, root):
        self.root = root
        self.nodes = []
        self.selected = 0
        self.scroll_top = 0
        self.error = None
        self.rebuild()
    def rebuild(self):
        previously_expanded = {n.path for n in self.nodes if n.is_dir and n.expanded}
        self.nodes = []
        if not os.path.isdir(self.root):
            self.error = f"Repertoire introuvable : {self.root}"
            return
        self._add_children(self.root, 0, previously_expanded)
    def _add_children(self, path, depth, previously_expanded):
        for full, name, is_dir in list_dir(path):
            node = Node(full, name, is_dir, depth)
            if is_dir and full in previously_expanded:
                node.expanded = True
            self.nodes.append(node)
            if node.is_dir and node.expanded:
                self._add_children(full, depth + 1, previously_expanded)
    def current(self):
        if not self.nodes:
            return None
        return self.nodes[self.selected]
    def move(self, delta):
        if not self.nodes:
            return
        self.selected = max(0, min(len(self.nodes) - 1, self.selected + delta))
    def toggle_or_open(self):
        node = self.current()
        if node is None:
            return None
        if node.is_dir:
            node.expanded = not node.expanded
            self.rebuild()
            return None
        return node.path
    def expand(self):
        node = self.current()
        if node and node.is_dir and not node.expanded:
            node.expanded = True
            self.rebuild()
    def collapse(self):
        node = self.current()
        if node and node.is_dir and node.expanded:
            node.expanded = False
            self.rebuild()
        elif node and node.depth > 0:
            for i in range(self.selected - 1, -1, -1):
                if self.nodes[i].depth < node.depth:
                    self.selected = i
                    break
    def adjust_scroll(self, visible_rows):
        if self.selected < self.scroll_top:
            self.scroll_top = self.selected
        elif self.selected >= self.scroll_top + visible_rows:
            self.scroll_top = self.selected - visible_rows + 1
def render(explorer, screen_rows, screen_cols, focused=True):
    out_lines = []
    header = f" {explorer.root}"
    out_lines.append({"segments": [{"text": header[:screen_cols], "color": COLOR_HEADER, "bold": True}]})
    if explorer.error:
        out_lines.append({"segments": [{"text": explorer.error, "color": COLOR_ERROR}]})
        send({"type": "render", "lines": out_lines})
        return
    visible_rows = screen_rows - 1
    explorer.adjust_scroll(visible_rows)
    for i in range(visible_rows):
        idx = explorer.scroll_top + i
        if idx >= len(explorer.nodes):
            out_lines.append({"segments": [{"text": "", "color": COLOR_FILE}]})
            continue
        node = explorer.nodes[idx]
        indent = "  " * node.depth
        if node.is_dir:
            icon = "v " if node.expanded else "> "
            text = f"{indent}{icon}{node.name}/"
            color = COLOR_DIR
        else:
            text = f"{indent}  {node.name}"
            color = COLOR_FILE
        is_selected = idx == explorer.selected
        prefix = "> " if is_selected else "  "
        full_text = (prefix + text)[:screen_cols]
        if is_selected:
            segment_color = COLOR_SELECTED_BG_MARK if focused else COLOR_DIR
        else:
            segment_color = color
        out_lines.append({"segments": [{"text": full_text, "color": segment_color, "bold": is_selected}]})
    send({"type": "render", "lines": out_lines})
def main():
    init = read_message()
    if init.get("type") != "init":
        return
    screen_rows = init.get("screen_rows", 24)
    screen_cols = init.get("screen_cols", 80)
    panel_mode = init.get("mode") == "panel"
    args = init.get("args", [])
    start_path = args[0] if args else "."
    if os.path.isfile(start_path):
        start_path = os.path.dirname(os.path.abspath(start_path)) or "."
    start_path = os.path.abspath(start_path)
    explorer = Explorer(start_path)
    focused = True
    render(explorer, screen_rows, screen_cols, focused)
    while True:
        msg = read_message()
        if msg is None:
            break
        msg_type = msg.get("type")
        if msg_type == "shutdown":
            break
        elif msg_type == "focus":
            focused = bool(msg.get("focused", True))
            render(explorer, screen_rows, screen_cols, focused)
        elif msg_type == "resize":
            screen_rows = msg.get("screen_rows", screen_rows)
            screen_cols = msg.get("screen_cols", screen_cols)
            render(explorer, screen_rows, screen_cols, focused)
        elif msg_type == "click":
            row = msg.get("row", 0)
            idx = explorer.scroll_top + row
            if 0 <= idx < len(explorer.nodes):
                explorer.selected = idx
                opened = explorer.toggle_or_open()
                if opened:
                    send({"type": "open_file", "path": opened})
            render(explorer, screen_rows, screen_cols, focused)
        elif msg_type == "key":
            key = msg.get("key", "")
            if key in ("Down", "j"):
                explorer.move(1)
            elif key in ("Up", "k"):
                explorer.move(-1)
            elif key == "Enter":
                opened = explorer.toggle_or_open()
                if opened:
                    send({"type": "open_file", "path": opened})
                    if not panel_mode:
                        break
            elif key == "Right":
                explorer.expand()
            elif key == "Left":
                explorer.collapse()
            elif key in ("q", "Q"):
                send({"type": "close"})
                break
            render(explorer, screen_rows, screen_cols, focused)
if __name__ == "__main__":
    main()