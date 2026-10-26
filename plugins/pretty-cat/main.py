#!/usr/bin/env python3
import json
import os
import re
import sys
_CONTROL_CHARS = re.compile("[\x00-\x08\x0a-\x1f\x7f-\x9f]")
def clean(text):
    # ESC et autres caracteres de controle -> "?", pour ne rien envoyer d'interpretable au terminal
    return _CONTROL_CHARS.sub("?", text)
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
    languages = [prepare_language(entry) for entry in raw]
    REGISTRY[:] = languages
    return languages


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


REGISTRY = []
SQL_STARTERS = ("select", "insert", "update", "delete", "create", "drop", "alter")
SQL_MARKERS = (" from ", " into ", " set ", " values", " table ", " where ", " join ")
ASCII_LETTERS = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"


class State:
    __slots__ = ("comment", "string", "embed", "embed_end", "inner_comment", "inner_string", "undecided")

    def __init__(self, comment=False, string=0, embed=0, embed_end=0, inner_comment=False, inner_string=0, undecided=False):
        self.comment = comment
        self.string = string
        self.embed = embed
        self.embed_end = embed_end
        self.inner_comment = inner_comment
        self.inner_string = inner_string
        self.undecided = undecided

    def copy(self):
        return State(self.comment, self.string, self.embed, self.embed_end, self.inner_comment, self.inner_string, self.undecided)


def ascii_lower(text):
    return "".join(chr(ord(c) + 32) if "A" <= c <= "Z" else c for c in text)


def ascii_upper(text):
    return "".join(chr(ord(c) - 32) if "a" <= c <= "z" else c for c in text)


def language_by_name(name):
    wanted = ascii_lower(name)
    if not wanted:
        return None
    for index, language in enumerate(REGISTRY):
        if language["filetype"] == wanted:
            return index + 1, language
    for index, language in enumerate(REGISTRY):
        if wanted in language["aliases"]:
            return index + 1, language
    return None


def language_at(index_plus_one):
    if index_plus_one < 1 or index_plus_one > len(REGISTRY):
        return None
    return REGISTRY[index_plus_one - 1]


def sniff_language(content, allowed):
    start = next((k for k, c in enumerate(content) if not c.isspace()), None)
    if start is None:
        return None
    rest = content[start:]
    if "html" in allowed and rest[0] == "<" and len(rest) > 1 and (rest[1].isalpha() or rest[1] in "!/?"):
        return "html"
    if "sql" in allowed:
        word_end = 0
        while word_end < len(rest) and rest[word_end] in ASCII_LETTERS:
            word_end += 1
        word = rest[:word_end]
        lower = ascii_lower(rest)
        uniform_case = word == ascii_upper(word) or word == ascii_lower(word)
        after = rest[word_end] if word_end < len(rest) else None
        starts_like_sql = ascii_lower(word) in SQL_STARTERS
        has_marker = any(marker in lower for marker in SQL_MARKERS)
        if starts_like_sql and uniform_case and after is not None and after.isspace() and has_marker:
            return "sql"
    return None


def matches_closing_tag(text, at, tag):
    if text[at:at + 2] != "</":
        return False
    idx = at + 2
    for expected in tag:
        if idx < len(text) and ascii_lower(text[idx]) == ascii_lower(expected):
            idx += 1
        else:
            return False
    return not (idx < len(text) and is_ident_char(text[idx], "-"))


def region_terminator(text, start, state, host):
    n = len(text)
    if state.embed_end == 0:
        if state.string < 1 or state.string > len(host["multiline_strings"]):
            return None
        end = host["multiline_strings"][state.string - 1][1]
        if not end:
            return None
        found = text.find(end, start)
        return found if found != -1 else None
    if state.embed_end > len(host["embed_tags"]):
        return None
    tag = host["embed_tags"][state.embed_end - 1][0]
    for p in range(start, n):
        if matches_closing_tag(text, p, tag):
            return p
    return None


def run_region(text, start, state, kinds, host):
    inner = language_at(state.embed)
    if inner is None:
        state.embed = 0
        state.embed_end = 0
        return start
    terminator = region_terminator(text, start, state, host)
    stop = terminator if terminator is not None else len(text)
    inner_state = State(comment=state.inner_comment, string=state.inner_string)
    inner_kinds, inner_result = highlight_row(text[start:stop], inner, inner_state, False)
    for offset, kind in enumerate(inner_kinds):
        kinds[start + offset] = kind
    if terminator is not None:
        state.embed = 0
        state.embed_end = 0
        state.inner_comment = False
        state.inner_string = 0
    else:
        state.inner_comment = inner_result.comment
        state.inner_string = inner_result.string
    return stop


def resolve_undecided_string(text, host, state):
    first = next((k for k, c in enumerate(text) if not c.isspace()), None)
    if first is None:
        return
    state.undecided = False
    if state.string < 1 or state.string > len(host["multiline_strings"]):
        return
    end = host["multiline_strings"][state.string - 1][1]
    found = text.find(end) if end else -1
    terminator = found if found != -1 else None
    if terminator is not None and terminator <= first:
        return
    content_end = terminator if terminator is not None else len(text)
    name = sniff_language(text[:content_end], host["string_embeds"])
    if name:
        found_language = language_by_name(name)
        if found_language:
            state.embed = found_language[0]
            state.embed_end = 0


def start_string_embed(text, start, state, kinds, host, delimiter):
    n = len(text)
    if state.string < 1 or state.string > len(host["multiline_strings"]):
        return start
    end = host["multiline_strings"][state.string - 1][1]
    found = text.find(end, start) if end else -1
    terminator = found if found != -1 else None
    if "fences" in host["flagset"] and delimiter in ("```", "~~~"):
        if terminator is not None:
            return start
        info = text[start:].strip().lstrip("{.")
        word = ""
        for c in info:
            if c.isalnum() or c in "+#-_":
                word += c
            else:
                break
        for k in range(start, n):
            kinds[k] = "string"
        found_language = language_by_name(word)
        if found_language:
            state.embed = found_language[0]
            state.embed_end = 0
        return n
    if not host["string_embeds"]:
        return start
    content_end = terminator if terminator is not None else n
    if all(c.isspace() for c in text[start:content_end]):
        if terminator is None:
            state.undecided = True
        return start
    name = sniff_language(text[start:content_end], host["string_embeds"])
    if name:
        found_language = language_by_name(name)
        if found_language:
            state.embed = found_language[0]
            state.embed_end = 0
            return run_region(text, start, state, kinds, host)
    return start


def start_tag_embed(text, start, state, kinds, host, tag):
    rule = next((k for k, (name, _) in enumerate(host["embed_tags"]) if name == tag), None)
    if rule is None:
        return start
    found_language = language_by_name(host["embed_tags"][rule][1])
    if not found_language:
        return start
    state.embed = found_language[0]
    state.embed_end = rule + 1
    return run_region(text, start, state, kinds, host)


def single_line_string_embed(text, opening, quote, kinds, host):
    n = len(text)
    close = opening + 1
    while close < n and text[close] != quote:
        if text[close] == "\\":
            close += 1
        close += 1
    close = min(close, n)
    name = sniff_language(text[opening + 1:close], host["string_embeds"])
    if not name:
        return None
    found_language = language_by_name(name)
    if not found_language:
        return None
    kinds[opening] = "string"
    inner_kinds, _ = highlight_row(text[opening + 1:close], found_language[1], State(), False)
    for offset, kind in enumerate(inner_kinds):
        kinds[opening + 1 + offset] = kind
    if close < n:
        kinds[close] = "string"
        return close + 1
    return n


def highlight_row(text, language, incoming, allow_embeds=True):
    n = len(text)
    state = incoming.copy()
    open_tag = None
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

    if not state.comment and state.string == 0 and state.embed == 0:
        for prefix, kind in language["line_rules"]:
            if starts(prefix, 0):
                return [kind.lower()] * n, State()

    first = next((k for k, c in enumerate(text) if not c.isspace()), None)
    in_string = None
    in_tag = False
    i = 0
    starts_with_directive = bool(directive) and first is not None and starts(directive, first)
    if (
        "key_values" in flags
        and not starts_with_directive
        and not state.comment
        and state.string == 0
        and state.embed == 0
    ):
        span = key_value_span(text, first)
        if span:
            fill(span[0], span[1], "type")
            i = span[1]

    if allow_embeds:
        if state.undecided and state.embed == 0 and state.string != 0:
            resolve_undecided_string(text, language, state)
        if state.embed != 0:
            i = run_region(text, 0, state, kinds, language)

    while i < n:
        c = text[i]
        preceding = text[i - 1] if i > 0 else None
        preceding_is_ident = preceding is not None and is_ident_char(preceding, extra)

        if state.string != 0:
            end = ml_strings[state.string - 1][1]
            if c == "\\" and i + 1 < n:
                length = escape_length(text, i)
                fill(i, i + length, "escape")
                i += length
                continue
            if starts(end, i):
                fill(i, i + len(end), "string")
                i += len(end)
                state.string = 0
                continue
            kinds[i] = "string"
            i += 1
            continue

        if state.comment:
            if starts(comment_end, i):
                fill(i, i + len(comment_end), "comment")
                i += len(comment_end)
                state.comment = False
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
            state.comment = True
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
                state.string = index + 1
                if allow_embeds:
                    i = start_string_embed(text, i, state, kinds, language, begin)
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
                    tag = open_tag
                    open_tag = None
                    if allow_embeds and c == ">" and tag is not None:
                        i = start_tag_embed(text, i, state, kinds, language, tag)
                    continue
            elif c == "<" and markup_tag_start(text, i):
                j = i + 1
                closing_tag = j < n and text[j] == "/"
                if j < n and text[j] in "/!?":
                    j += 1
                name_start = j
                while j < n and is_ident_char(text[j], "-:."):
                    j += 1
                fill(i, j, "keyword1")
                open_tag = None if closing_tag else ascii_lower(text[name_start:j])
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
            if allow_embeds and language["string_embeds"]:
                embedded_end = single_line_string_embed(text, i, c, kinds, language)
                if embedded_end is not None:
                    i = embedded_end
                    continue
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

    return kinds, state


def classify_lines(lines, language):
    result = []
    state = State()
    for line in lines:
        kinds, state = highlight_row(line, language, state)
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
                sys.stdout.write(clean(segment["text"]))
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