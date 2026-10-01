#!/usr/bin/env python3
"""
Plugin Cera "Pretty Cat" : affiche un fichier en lecture seule, avec
numeros de ligne et une coloration syntaxique basique (mots-cles, chaines,
commentaires) selon l'extension du fichier.

Deux modes :
- Protocole plugin (par defaut) : un message JSON par ligne, sur stdin
  (recu) et stdout (envoye). Voir le README du dossier plugins/ pour le
  detail du protocole. C'est ce mode qu'utilise Cera en interne (raccourci
  ^A).
- `main.py --cat fichier...` : mode non interactif utilise par
  `cera -a`, qui ecrit directement le contenu colore des fichiers sur
  stdout, comme `cat`, sans protocole JSON.
"""
import json
import sys
import re

KEYWORDS = {
    ".py": {"def", "class", "import", "from", "return", "if", "elif", "else",
            "for", "while", "in", "not", "and", "or", "is", "None", "True",
            "False", "try", "except", "finally", "with", "as", "lambda",
            "yield", "pass", "break", "continue", "raise", "global", "self"},
    ".rs": {"fn", "let", "mut", "struct", "enum", "impl", "trait", "pub",
             "use", "mod", "return", "if", "else", "for", "while", "loop",
             "match", "in", "self", "Self", "true", "false", "None", "Some",
             "Ok", "Err", "const", "static", "as", "ref", "move"},
    ".c": {"int", "char", "float", "double", "void", "return", "if", "else",
            "for", "while", "struct", "typedef", "static", "const", "sizeof",
            "break", "continue", "switch", "case", "default", "unsigned",
            "long", "short", "signed"},
    ".cpp": {"int", "char", "float", "double", "void", "return", "if", "else",
              "for", "while", "struct", "class", "public", "private",
              "protected", "static", "const", "sizeof", "break", "continue",
              "switch", "case", "default", "namespace", "using", "template",
              "new", "delete", "this", "nullptr", "true", "false"},
    ".js": {"function", "const", "let", "var", "return", "if", "else", "for",
             "while", "class", "extends", "new", "this", "true", "false",
             "null", "undefined", "typeof", "import", "export", "default"},
}

COLOR_KEYWORD = "#82C0FF"
COLOR_STRING = "#A8E6CF"
COLOR_COMMENT = "#8FA3B8"
COLOR_NUMBER = "#C9A8FF"
COLOR_LINENO = "#7BA3C9"
COLOR_NORMAL = "#E8EDF2"
COLOR_ERROR = "#FF8A8A"

COMMENT_PREFIXES = {
    ".py": "#", ".rs": "//", ".c": "//", ".cpp": "//", ".js": "//",
    ".sh": "#", ".rb": "#",
}

STRING_RE = re.compile(r'"[^"]*"|\'[^\']*\'')
NUMBER_RE = re.compile(r'\b\d+\.?\d*\b')
WORD_RE = re.compile(r'[A-Za-z_][A-Za-z0-9_]*')


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


def ext_of(path):
    for ext in KEYWORDS:
        if path.endswith(ext):
            return ext
    return None


def highlight_line(line, ext):
    """Decoupe une ligne en segments colores (mots-cles/chaines/nombres/
    commentaires/normal). Approche simple par tokenisation ligne par
    ligne, suffisante pour un survol rapide d'un fichier."""
    segments = []
    comment_prefix = COMMENT_PREFIXES.get(ext)
    if comment_prefix:
        idx = line.find(comment_prefix)
        if idx != -1:
            code_part, comment_part = line[:idx], line[idx:]
            segments.extend(highlight_code(code_part, ext))
            segments.append({"text": comment_part, "color": COLOR_COMMENT})
            return segments
    return highlight_code(line, ext)


def highlight_code(text, ext):
    segments = []
    keywords = KEYWORDS.get(ext, set())
    pos = 0
    tokens = []
    for m in STRING_RE.finditer(text):
        tokens.append((m.start(), m.end(), "string"))
    for m in NUMBER_RE.finditer(text):
        tokens.append((m.start(), m.end(), "number"))
    for m in WORD_RE.finditer(text):
        if m.group() in keywords:
            tokens.append((m.start(), m.end(), "keyword"))
    tokens.sort()

    last_end = 0
    for start, end, kind in tokens:
        if start < last_end:
            continue
        if start > last_end:
            segments.append({"text": text[last_end:start], "color": COLOR_NORMAL})
        color = {"string": COLOR_STRING, "number": COLOR_NUMBER, "keyword": COLOR_KEYWORD}[kind]
        segments.append({"text": text[start:end], "color": color})
        last_end = end
    if last_end < len(text):
        segments.append({"text": text[last_end:], "color": COLOR_NORMAL})
    if not segments:
        segments.append({"text": "", "color": COLOR_NORMAL})
    return segments


def render(lines, top, screen_rows, screen_cols, path, error=None):
    out_lines = []
    if error:
        out_lines.append({"segments": [{"text": error, "color": COLOR_ERROR, "bold": True}]})
        send({"type": "render", "lines": out_lines})
        return

    total = len(lines)
    gutter_width = max(3, len(str(total))) + 1
    ext = ext_of(path)

    visible = screen_rows - 1  # une ligne reservee a l'en-tete
    header = f" {path}  ({total} lignes) -- Echap pour revenir"
    out_lines.append({"segments": [{"text": header[:screen_cols], "color": COLOR_KEYWORD, "bold": True}]})

    for i in range(visible):
        line_idx = top + i
        if line_idx >= total:
            out_lines.append({"segments": [{"text": "", "color": COLOR_NORMAL}]})
            continue
        line_text = lines[line_idx].rstrip("\n")
        lineno = str(line_idx + 1).rjust(gutter_width - 1) + " "
        segments = [{"text": lineno, "color": COLOR_LINENO}]
        segments.extend(highlight_line(line_text, ext))
        out_lines.append({"segments": segments})

    send({"type": "render", "lines": out_lines})


def ansi_fg(hex_color, bold=False):
    hex_color = hex_color.lstrip("#")
    r, g, b = int(hex_color[0:2], 16), int(hex_color[2:4], 16), int(hex_color[4:6], 16)
    prefix = "\x1b[1m" if bold else ""
    return f"{prefix}\x1b[38;2;{r};{g};{b}m"


ANSI_RESET = "\x1b[0m"


def cat_mode(paths):
    """Mode non interactif, utilise par `cera -a` : ecrit directement sur
    stdout le contenu colore des fichiers donnes, numeros de ligne inclus,
    puis quitte -- comme `cat`, sans protocole JSON ni lecture de stdin.
    Code de sortie 0 si tous les fichiers ont pu etre lus, 1 sinon (encore
    comme `cat`)."""
    exit_code = 0
    for path in paths:
        try:
            with open(path, "r", encoding="utf-8", errors="replace") as f:
                lines = f.readlines()
        except OSError as e:
            sys.stderr.write(f"cera -a: {path}: {e.strerror or e}\n")
            exit_code = 1
            continue

        ext = ext_of(path)
        gutter_width = max(3, len(str(len(lines)))) + 1
        for i, raw_line in enumerate(lines):
            line_text = raw_line.rstrip("\n")
            lineno = str(i + 1).rjust(gutter_width - 1) + " "
            sys.stdout.write(ansi_fg(COLOR_LINENO))
            sys.stdout.write(lineno)
            sys.stdout.write(ANSI_RESET)
            for segment in highlight_line(line_text, ext):
                sys.stdout.write(ansi_fg(segment["color"], segment.get("bold", False)))
                sys.stdout.write(segment["text"])
                sys.stdout.write(ANSI_RESET)
            sys.stdout.write("\n")
    return exit_code


def main():
    if len(sys.argv) > 1 and sys.argv[1] == "--cat":
        sys.exit(cat_mode(sys.argv[2:]))

    init = read_message()
    if init.get("type") != "init":
        return
    screen_rows = init.get("screen_rows", 24)
    screen_cols = init.get("screen_cols", 80)
    args = init.get("args", [])
    path = args[0] if args else ""

    lines = []
    error = None
    try:
        with open(path, "r", encoding="utf-8", errors="replace") as f:
            lines = f.readlines()
        if not lines:
            lines = [""]
    except OSError as e:
        error = f"Impossible de lire {path}: {e}"

    top = 0
    render(lines, top, screen_rows, screen_cols, path, error)

    while True:
        msg = read_message()
        if msg is None:
            break
        msg_type = msg.get("type")

        if msg_type == "shutdown":
            break
        elif msg_type == "resize":
            screen_rows = msg.get("screen_rows", screen_rows)
            screen_cols = msg.get("screen_cols", screen_cols)
            render(lines, top, screen_rows, screen_cols, path, error)
        elif msg_type == "key":
            key = msg.get("key", "")
            visible = screen_rows - 1
            max_top = max(0, len(lines) - visible)
            if key in ("Down", "j"):
                top = min(top + 1, max_top)
            elif key in ("Up", "k"):
                top = max(top - 1, 0)
            elif key == "PageDown":
                top = min(top + visible, max_top)
            elif key == "PageUp":
                top = max(top - visible, 0)
            elif key == "Home":
                top = 0
            elif key == "End":
                top = max_top
            elif key in ("q", "Q"):
                send({"type": "close"})
                break
            render(lines, top, screen_rows, screen_cols, path, error)


if __name__ == "__main__":
    main()
