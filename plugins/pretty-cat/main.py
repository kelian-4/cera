#!/usr/bin/env python3
import json
import os
import sys
LANGUAGES_PATH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "languages.json")
COLORS = {
    "normal": "#E8EDF2",
    "comment": "#8FA3B8",
    "keyword1": "#82C0FF",
    "keyword2": "#8CF5D6",
    "string": "#A8E6CF",
    "number": "#C9A8FF",
    "type": "#FFD08A",
    "function": "#F9A8D4",
    "constant": "#FF8FA3",
    "operator": "#B4C2D4",
    "preprocessor": "#D4E27A",
    "variable": "#FFC6A5",
    "escape": "#FFB86B",
}
COLOR_KEYWORD = COLORS["keyword1"]
COLOR_LINENO = "#7BA3C9"
COLOR_NORMAL = COLORS["normal"]
COLOR_ERROR = "#FF8A8A"
DIGITS = "0123456789"
HEX_DIGITS = "0123456789abcdefABCDEF"
OCTAL_DIGITS = "01234567"
DOLLAR_SPECIALS = "@?#!*-$<^"
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
def load_languages():
    with open(LANGUAGES_PATH, "r", encoding="utf-8") as f:
        raw = json.load(f)
    return [prepare_language(entry) for entry in raw]


def prepare_language(entry):
    case_insensitive = "case_insensitive" in entry["flags"]
    normalize = str.lower if case_insensitive else str
    words = {}
    for keyword in entry["keywords"]:
        second = keyword.endswith("|")
        body = keyword[:-1] if second else keyword
        words.setdefault(normalize(body), "keyword2" if second else "keyword1")
    for word in entry["types"]:
        words.setdefault(normalize(word), "type")
    for word in entry["constants"]:
        words.setdefault(normalize(word), "constant")
    language = dict(entry)
    language["flagset"] = set(entry["flags"])
    language["words"] = words
    language["normalize"] = normalize
    return language


def select_language(languages, path):
    dot = path.rfind(".")
    extension = path[dot:] if dot != -1 else None
    for language in languages:
        for pattern in language["filematch"]:
            if pattern.startswith("."):
                if extension == pattern:
                    return language
            elif pattern in path:
                return language
    return None


def is_ident_start(c):
    return c.isalpha() or c == "_"


def is_ident_char(c, extra):
    return c.isalnum() or c == "_" or c in extra


def next_non_space(text, start):
    for c in text[start:]:
        if not c.isspace():
            return c
    return None


def escape_length(text, i):
    n = len(text)
    if i + 1 >= n:
        return 1
    following = text[i + 1]
    length = 2
    if following == "x":
        while length < 4 and i + length < n and text[i + length] in HEX_DIGITS:
            length += 1
    elif following in "uU":
        if i + 2 < n and text[i + 2] == "{":
            while i + length < n:
                c = text[i + length]
                length += 1
                if c == "}":
                    break
        else:
            limit = 6 if following == "u" else 10
            while length < limit and i + length < n and text[i + length] in HEX_DIGITS:
                length += 1
    elif following in OCTAL_DIGITS:
        while length < 4 and i + length < n and text[i + length] in OCTAL_DIGITS:
            length += 1
    return length


def scan_number(text, start, css_values):
    n = len(text)
    j = start
    if text[j] == "0" and j + 1 < n and text[j + 1] in "xXbBoO":
        j += 2
        while j < n and (text[j] in HEX_DIGITS or text[j] == "_"):
            j += 1
    else:
        while j < n and (text[j] in DIGITS or text[j] == "_"):
            j += 1
        if j + 1 < n and text[j] == "." and text[j + 1] in DIGITS:
            j += 1
            while j < n and (text[j] in DIGITS or text[j] == "_"):
                j += 1
        if j < n and text[j] in "eE":
            k = j + 1
            if k < n and text[k] in "+-":
                k += 1
            if k < n and text[k] in DIGITS:
                j = k
                while j < n and text[j] in DIGITS:
                    j += 1
    while j < n and (text[j].isalnum() or text[j] == "_"):
        j += 1
    if css_values and j < n and text[j] == "%":
        j += 1
    return j


def ident_end(text, start, extra):
    n = len(text)
    j = start
    while j < n and is_ident_char(text[j], extra):
        if text[j] in "?!" and j + 1 < n and text[j + 1] == "=":
            break
        j += 1
    while j > start + 1 and text[j - 1] == "-":
        j -= 1
    return j


def classify_word(language, word, following, preceding):
    flags = language["flagset"]
    kind = language["words"].get(language["normalize"](word))
    if kind:
        return kind
    if "functions" in flags and following == "(":
        return "function"
    if "paren_calls" in flags and preceding == "(":
        return "function"
    if "capitalized_types" in flags and word[0].isupper() and any(c.islower() for c in word):
        return "type"
    if (
        "upper_constants" in flags
        and len(word) >= 2
        and all(c.isupper() or c in DIGITS or c == "_" for c in word)
        and any(c.isupper() for c in word)
    ):
        return "constant"
    return "normal"


def annotation_end(text, i, language):
    prefixes = language["annotation_prefixes"]
    if not prefixes or (i > 0 and is_ident_char(text[i - 1], "")):
        return None
    n = len(text)
    for prefix in prefixes:
        if not text.startswith(prefix, i):
            continue
        j = i + len(prefix)
        if not (j < n and is_ident_start(text[j])):
            continue
        while j < n:
            c = text[j]
            dotted = c == "." and j + 1 < n and is_ident_start(text[j + 1])
            if is_ident_char(c, "") or dotted:
                j += 1
            else:
                break
        return j
    return None


def variable_end(text, i, language):
    c = text[i]
    if c not in language["variable_prefixes"]:
        return None
    preceding = text[i - 1] if i > 0 else None
    if preceding is not None and is_ident_char(preceding, language["ident_chars"]):
        return None
    if c == "%" and not (preceding is None or preceding.isspace() or preceding in "(,"):
        return None
    n = len(text)
    j = i + 1
    if c == "@" and j < n and text[j] == "@":
        j += 1
    following = text[j] if j < n else None
    if following is not None and is_ident_start(following):
        while j < n and is_ident_char(text[j], ""):
            j += 1
        return j
    if following == "{":
        close = text.find("}", j)
        return close + 1 if close != -1 else None
    if following == "(" and c == "$":
        k = j + 1
        while k < n and is_ident_char(text[k], ""):
            k += 1
        if k > j + 1 and k < n and text[k] == ")":
            return k + 1
        return j + 1
    if following is not None and c == "$" and (following in DIGITS or following in DOLLAR_SPECIALS):
        return j + 1
    return None


def char_literal_end(text, i):
    n = len(text)
    if i + 1 < n and text[i + 1] == "\\":
        limit = min(i + 9, n)
        for k in range(i + 3, limit):
            if text[k] == "'":
                return k + 1
        return None
    if i + 2 < n and text[i + 1] != "'" and text[i + 2] == "'":
        return i + 3
    return None


def hex_color_end(text, i):
    n = len(text)
    j = i + 1
    while j < n and text[j] in HEX_DIGITS:
        j += 1
    digits = j - i - 1
    followed_by_ident = j < n and is_ident_char(text[j], "-")
    if digits in (3, 4, 6, 8) and not followed_by_ident:
        return j
    return None


def entity_end(text, i):
    n = len(text)
    j = i + 1
    if j < n and text[j] == "#":
        j += 1
    start = j
    while j < n and j - i <= 10 and text[j].isalnum():
        j += 1
    if j > start and j < n and text[j] == ";":
        return j + 1
    return None


def markup_tag_start(text, i):
    n = len(text)
    if i + 1 >= n:
        return False
    following = text[i + 1]
    if following.isalpha():
        return True
    if following in "/!":
        return i + 2 < n and text[i + 2].isalpha()
    return following == "?"


def key_value_span(text, first):
    if first is None:
        return None
    n = len(text)
    start = first
    while start + 1 < n and text[start] == "-" and text[start + 1] == " ":
        start += 2
        while start < n and text[start] == " ":
            start += 1
    end = start
    while end < n and (text[end].isalnum() or text[end] in "_.-/@"):
        end += 1
    if end == start:
        return None
    j = end
    while j < n and text[j] == " ":
        j += 1
    if j < n and text[j] == ":" and (j + 1 >= n or text[j + 1] == " "):
        return (start, end)
    if j < n and text[j] == "=":
        return (start, end)
    return None


def highlight_row(text, language, in_comment, open_string):
    n = len(text)
    kinds = ["normal"] * n
    flags = language["flagset"]
    extra = language["ident_chars"]
    markup = "markup" in flags
    strings = "strings" in flags
    comment_start = language["multiline_comment_start"]
    comment_end = language["multiline_comment_end"]
    single = language["singleline_comment_start"]
    alternate = language["alt_comment_start"]
    ml_strings = language["multiline_strings"]
    quotes = language["quotes"]
    directive = language["directive_prefix"]
    operators = language["operators"]

    def fill(start, end, kind):
        for k in range(start, min(end, n)):
            kinds[k] = kind

    def starts(pattern, at):
        return bool(pattern) and text.startswith(pattern, at)

    if not in_comment and open_string == 0:
        for prefix, kind in language["line_rules"]:
            if starts(prefix, 0):
                return [kind.lower()] * n, False, 0

    first = next((k for k, c in enumerate(text) if not c.isspace()), None)
    in_string = None
    in_tag = False
    i = 0
    starts_with_directive = bool(directive) and first is not None and starts(directive, first)
    if "key_values" in flags and not starts_with_directive and not in_comment and open_string == 0:
        span = key_value_span(text, first)
        if span:
            fill(span[0], span[1], "type")
            i = span[1]

    while i < n:
        c = text[i]
        preceding = text[i - 1] if i > 0 else None
        preceding_is_ident = preceding is not None and is_ident_char(preceding, extra)

        if open_string != 0:
            end = ml_strings[open_string - 1][1]
            if c == "\\" and i + 1 < n:
                length = escape_length(text, i)
                fill(i, i + length, "escape")
                i += length
                continue
            if starts(end, i):
                fill(i, i + len(end), "string")
                i += len(end)
                open_string = 0
                continue
            kinds[i] = "string"
            i += 1
            continue

        if in_comment:
            if starts(comment_end, i):
                fill(i, i + len(comment_end), "comment")
                i += len(comment_end)
                in_comment = False
            else:
                kinds[i] = "comment"
                i += 1
            continue

        if in_string is not None:
            quote, string_start = in_string
            if c == "\\" and i + 1 < n:
                length = escape_length(text, i)
                fill(i, i + length, "escape")
                i += length
                continue
            kinds[i] = "string"
            if c == quote:
                in_string = None
                if "key_strings" in flags and next_non_space(text, i + 1) == ":":
                    fill(string_start, i + 1, "type")
            i += 1
            continue

        if comment_start and comment_end and starts(comment_start, i):
            fill(i, i + len(comment_start), "comment")
            i += len(comment_start)
            in_comment = True
            continue

        shell_hash = single == "#" and preceding in ("$", "{")
        if not shell_hash and (starts(single, i) or starts(alternate, i)):
            fill(i, n, "comment")
            break

        started = False
        for index, (begin, _end) in enumerate(ml_strings):
            begins_with_ident = bool(begin) and is_ident_char(begin[0], "")
            if starts(begin, i) and not (begins_with_ident and preceding_is_ident):
                fill(i, i + len(begin), "string")
                i += len(begin)
                open_string = index + 1
                started = True
                break
        if started:
            continue

        if markup:
            if in_tag:
                closes = c == ">" or (c in "/?" and i + 1 < n and text[i + 1] == ">")
                if closes:
                    length = 1 if c == ">" else 2
                    fill(i, i + length, "keyword1")
                    i += length
                    in_tag = False
                    continue
            elif c == "<" and markup_tag_start(text, i):
                j = i + 1
                if j < n and text[j] in "/!?":
                    j += 1
                while j < n and is_ident_char(text[j], "-:."):
                    j += 1
                fill(i, j, "keyword1")
                i = j
                in_tag = True
                continue
            else:
                if c == "&":
                    end = entity_end(text, i)
                    if end is not None:
                        fill(i, end, "escape")
                        i = end
                        continue
                i += 1
                continue

        if first == i and directive and starts(directive, i):
            j = i + len(directive)
            while j < n and text[j] == " ":
                j += 1
            word_start = j
            while j < n and is_ident_char(text[j], ""):
                j += 1
            if j > word_start:
                fill(i, j, "preprocessor")
                word = text[word_start:j]
                i = j
                if word in ("include", "import", "include_next"):
                    k = i
                    while k < n and text[k] == " ":
                        k += 1
                    if k < n and text[k] == "<":
                        close = text.find(">", k)
                        if close != -1:
                            fill(k, close + 1, "string")
                            i = close + 1
                continue

        end = annotation_end(text, i, language)
        if end is not None:
            fill(i, end, "preprocessor")
            i = end
            continue

        if "css_values" in flags and c == "#":
            end = hex_color_end(text, i)
            if end is not None:
                fill(i, end, "number")
                i = end
                continue

        end = variable_end(text, i, language)
        if end is not None:
            fill(i, end, "variable")
            i = end
            continue

        tick_blocked = (
            c == "'"
            and "tick_strings" in flags
            and preceding is not None
            and (is_ident_char(preceding, "") or preceding in ")]}.'")
        )
        if strings and c in quotes and (not markup or in_tag) and not tick_blocked:
            in_string = (c, i)
            kinds[i] = "string"
            i += 1
            continue

        if "char_literals" in flags and c == "'":
            end = char_literal_end(text, i)
            if end is not None:
                fill(i, end, "string")
                i = end
                continue

        number_start = c in DIGITS or (
            c == "." and i + 1 < n and text[i + 1] in DIGITS and preceding != "."
        )
        if "numbers" in flags and number_start and not preceding_is_ident:
            end = scan_number(text, i, "css_values" in flags)
            fill(i, end, "number")
            i = end
            continue

        if is_ident_start(c) and not preceding_is_ident:
            end = ident_end(text, i, extra)
            word = text[i:end]
            following = next_non_space(text, end)
            if in_tag:
                kind = "type" if following == "=" else "normal"
            else:
                kind = classify_word(language, word, following, preceding)
            fill(i, end, kind)
            i = end
            continue

        if c in operators:
            kinds[i] = "operator"
        i += 1

    return kinds, in_comment, open_string


def classify_lines(lines, language):
    result = []
    in_comment = False
    open_string = 0
    for line in lines:
        kinds, in_comment, open_string = highlight_row(line, language, in_comment, open_string)
        result.append(kinds)
    return result


def segments_for(text, kinds):
    if not text:
        return [{"text": "", "color": COLOR_NORMAL}]
    segments = []
    start = 0
    for k in range(1, len(text) + 1):
        if k == len(text) or kinds[k] != kinds[start]:
            segments.append({"text": text[start:k], "color": COLORS[kinds[start]]})
            start = k
    return segments


def highlight_file(lines, language):
    texts = [line.rstrip("\n") for line in lines]
    if language is None:
        return [[{"text": text, "color": COLOR_NORMAL}] for text in texts]
    return [segments_for(text, kinds) for text, kinds in zip(texts, classify_lines(texts, language))]


def render(highlighted, top, screen_rows, screen_cols, path, error=None):
    out_lines = []
    if error:
        out_lines.append({"segments": [{"text": error, "color": COLOR_ERROR, "bold": True}]})
        send({"type": "render", "lines": out_lines})
        return
    total = len(highlighted)
    gutter_width = max(3, len(str(total))) + 1
    visible = screen_rows - 1
    header = f" {path}  ({total} lignes) -- Echap pour revenir"
    out_lines.append({"segments": [{"text": header[:screen_cols], "color": COLOR_KEYWORD, "bold": True}]})
    for i in range(visible):
        line_idx = top + i
        if line_idx >= total:
            out_lines.append({"segments": [{"text": "", "color": COLOR_NORMAL}]})
            continue
        lineno = str(line_idx + 1).rjust(gutter_width - 1) + " "
        segments = [{"text": lineno, "color": COLOR_LINENO}]
        segments.extend(highlighted[line_idx])
        out_lines.append({"segments": segments})
    send({"type": "render", "lines": out_lines})
def ansi_fg(hex_color, bold=False):
    hex_color = hex_color.lstrip("#")
    r, g, b = int(hex_color[0:2], 16), int(hex_color[2:4], 16), int(hex_color[4:6], 16)
    prefix = "\x1b[1m" if bold else ""
    return f"{prefix}\x1b[38;2;{r};{g};{b}m"
ANSI_RESET = "\x1b[0m"
def load_languages_or_none():
    try:
        return load_languages()
    except (OSError, ValueError):
        return []


def cat_mode(paths):
    exit_code = 0
    languages = load_languages_or_none()
    for path in paths:
        try:
            with open(path, "r", encoding="utf-8", errors="replace") as f:
                lines = f.readlines()
        except OSError as e:
            sys.stderr.write(f"cera -a: {path}: {e.strerror or e}\n")
            exit_code = 1
            continue
        highlighted = highlight_file(lines, select_language(languages, path))
        gutter_width = max(3, len(str(len(lines)))) + 1
        for i, line_segments in enumerate(highlighted):
            lineno = str(i + 1).rjust(gutter_width - 1) + " "
            sys.stdout.write(ansi_fg(COLOR_LINENO))
            sys.stdout.write(lineno)
            sys.stdout.write(ANSI_RESET)
            for segment in line_segments:
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
    highlighted = highlight_file(lines, select_language(load_languages_or_none(), path))
    render(highlighted, top, screen_rows, screen_cols, path, error)
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
            render(highlighted, top, screen_rows, screen_cols, path, error)
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
            render(highlighted, top, screen_rows, screen_cols, path, error)
if __name__ == "__main__":
    main()