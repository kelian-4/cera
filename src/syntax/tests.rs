use super::language_samples::highlight_lines;
use super::*;
use crate::io::config::ThemeColors;
use crate::view::colors::highlight_to_rgb;

fn highlight(file: &str, lines: &[&str]) -> Vec<Vec<Highlight>> {
    let database = build_database();
    let syntax = select_for_filename(&database, file).expect("syntax for file");
    highlight_lines(syntax, lines)
}

fn kind_of(row: &[Highlight], line: &str, token: &str) -> Vec<Highlight> {
    let byte_start = line.find(token).expect("token in line");
    let start = line[..byte_start].chars().count();
    row[start..start + token.chars().count()].to_vec()
}

fn assert_token(file: &str, line: &str, token: &str, expected: Highlight) {
    let rows = highlight(file, &[line]);
    let kinds = kind_of(&rows[0], line, token);
    assert!(
        kinds.iter().all(|k| *k == expected),
        "{file}: {token:?} in {line:?} expected {expected:?}, got {kinds:?}"
    );
}

#[test]
fn test_syntax_selection() {
    let db = build_database();
    for (file, filetype) in [
        ("main.c", "c"),
        ("main.cpp", "cpp"),
        ("script.py", "python"),
        ("app.ts", "typescript"),
        ("app.js", "javascript"),
        ("main.rs", "rust"),
        ("main.go", "go"),
        ("flake.nix", "nix"),
        ("/home/u/code/Makefile", "makefile"),
        ("/home/u/code/CMakeLists.txt", "cmake"),
        ("/srv/app/Dockerfile", "dockerfile"),
        ("/home/u/.bashrc", "shell"),
        ("/etc/systemd/system/a.service", "ini"),
        ("Cargo.toml", "toml"),
    ] {
        let selected = select_for_filename(&db, file);
        assert!(selected.is_some(), "no syntax for {file}");
        assert_eq!(selected.unwrap().filetype, filetype, "file {file}");
    }
    assert!(select_for_filename(&db, "data.xyz").is_none());
}

#[test]
fn test_update_syntax_keyword() {
    let db = build_database();
    let py = select_for_filename(&db, "script.py").unwrap();

    let mut buf = Buffer::new();
    buf.insert_row(0, "def foo():");
    update_syntax(&mut buf, 0, Some(py));

    assert_eq!(buf.row(0).hl()[0], Highlight::Keyword1);
}

#[test]
fn test_update_syntax_propagates_multiline_comment_state() {
    let db = build_database();
    let c = select_for_filename(&db, "a.c").unwrap();
    let mut buf = Buffer::new();
    buf.insert_row(0, "/* open");
    buf.insert_row(1, "still");
    buf.insert_row(2, "end */ int x;");
    update_syntax(&mut buf, 0, Some(c));
    assert_eq!(buf.row(1).hl()[0], Highlight::MlComment);
    assert_eq!(buf.row(2).hl()[0], Highlight::MlComment);
    assert_eq!(buf.row(2).hl()[7], Highlight::Type);
}

#[test]
fn test_update_syntax_propagates_multiline_string_state() {
    let db = build_database();
    let py = select_for_filename(&db, "a.py").unwrap();
    let mut buf = Buffer::new();
    buf.insert_row(0, "x = \"\"\"a");
    buf.insert_row(1, "def");
    buf.insert_row(2, "\"\"\"");
    buf.insert_row(3, "def");
    for row in 0..buf.num_rows() {
        update_syntax(&mut buf, row, Some(py));
    }
    assert_eq!(buf.row(1).hl()[0], Highlight::String);
    assert_eq!(buf.row(3).hl()[0], Highlight::Keyword1);
}

#[test]
fn test_numbers_hex_binary_float_exponent_and_suffix() {
    let line = "0xFF 0b101 1_000 3.14 1e-5 2.5f 7u8 .5";
    let rows = highlight("a.rs", &[line]);
    for token in [
        "0xFF", "0b101", "1_000", "3.14", "1e-5", "2.5f", "7u8", ".5",
    ] {
        assert!(
            kind_of(&rows[0], line, token)
                .iter()
                .all(|k| *k == Highlight::Number),
            "{token}"
        );
    }
}

#[test]
fn test_digits_inside_identifiers_are_not_numbers() {
    assert_token("a.c", "int var1 = 2;", "var1", Highlight::Normal);
    assert_token("a.c", "int var1 = 2;", "2", Highlight::Number);
}

#[test]
fn test_keywords_inside_identifiers_are_not_highlighted() {
    assert_token("a.c", "interval iffy my_if", "interval", Highlight::Normal);
    assert_token("a.c", "interval iffy my_if", "iffy", Highlight::Normal);
    assert_token("a.c", "interval iffy my_if", "my_if", Highlight::Normal);
}

#[test]
fn test_string_escapes_are_highlighted() {
    let line = r#"let s = "a\x41\u{1F600}\n";"#;
    let rows = highlight("a.rs", &[line]);
    for token in [r"\x41", r"\u{1F600}", r"\n"] {
        assert!(
            kind_of(&rows[0], line, token)
                .iter()
                .all(|k| *k == Highlight::Escape),
            "{token}"
        );
    }
    assert_eq!(kind_of(&rows[0], line, "\"a")[1], Highlight::String);
}

#[test]
fn test_case_insensitive_keywords() {
    assert_token("a.sql", "SeLeCt x FrOm t", "SeLeCt", Highlight::Keyword1);
    assert_token("a.sql", "SeLeCt x FrOm t", "FrOm", Highlight::Keyword1);
}

#[test]
fn test_case_sensitive_languages_do_not_match_other_cases() {
    assert_token("a.c", "RETURN 1", "RETURN", Highlight::Constant);
    assert_token("a.py", "DEF x", "DEF", Highlight::Constant);
}

#[test]
fn test_rust_lifetime_is_not_a_string_but_char_literal_is() {
    let line = "fn f<'a>(x: &'a str) -> char { 'z' }";
    let rows = highlight("a.rs", &[line]);
    assert!(kind_of(&rows[0], line, "'a")
        .iter()
        .all(|k| *k == Highlight::Normal));
    assert!(kind_of(&rows[0], line, "'z'")
        .iter()
        .all(|k| *k == Highlight::String));
}

#[test]
fn test_transpose_tick_is_not_a_string_in_matlab() {
    let line = "y = x' * 2; s = 'ok';";
    let rows = highlight("a.m", &[line]);
    assert!(kind_of(&rows[0], line, "2")
        .iter()
        .all(|k| *k == Highlight::Number));
    assert!(kind_of(&rows[0], line, "'ok'")
        .iter()
        .all(|k| *k == Highlight::String));
}

#[test]
fn test_comment_marker_inside_string_is_not_a_comment() {
    assert_token(
        "a.c",
        "char *u = \"http://x\"; // c",
        "\"http://x\"",
        Highlight::String,
    );
    assert_token(
        "a.c",
        "char *u = \"http://x\"; // c",
        "// c",
        Highlight::Comment,
    );
}

#[test]
fn test_shell_dollar_hash_is_not_a_comment() {
    assert_token("a.sh", "echo $# args # c", "$#", Highlight::Variable);
    assert_token("a.sh", "echo $# args # c", "# c", Highlight::Comment);
}

#[test]
fn test_json_keys_and_values_use_different_colors() {
    let line = r#"{"k": "v", "n": 1}"#;
    let rows = highlight("a.json", &[line]);
    assert!(kind_of(&rows[0], line, "\"k\"")
        .iter()
        .all(|k| *k == Highlight::Type));
    assert!(kind_of(&rows[0], line, "\"v\"")
        .iter()
        .all(|k| *k == Highlight::String));
}

#[test]
fn test_unterminated_string_does_not_leak_to_next_row() {
    let rows = highlight("a.c", &["char *s = \"open", "int x;"]);
    assert_eq!(rows[1][0], Highlight::Type);
}

#[test]
fn test_every_language_is_well_formed() {
    let db = build_database();
    assert!(
        db.len() >= 70,
        "expected at least 70 languages, got {}",
        db.len()
    );
    let mut filetypes = std::collections::HashSet::new();
    let mut extensions = std::collections::HashMap::new();
    for syntax in &db {
        assert!(!syntax.filetype.is_empty());
        assert!(
            filetypes.insert(syntax.filetype),
            "duplicate {}",
            syntax.filetype
        );
        assert!(
            !syntax.filematch.is_empty(),
            "{} has no filematch",
            syntax.filetype
        );
        for pattern in syntax.filematch {
            assert!(!pattern.is_empty());
            if pattern.starts_with('.') {
                if let Some(previous) = extensions.insert(*pattern, syntax.filetype) {
                    panic!("{pattern} is used by {previous} and {}", syntax.filetype);
                }
            }
        }
        for kw in syntax.keywords {
            assert!(
                !kw.trim_end_matches('|').is_empty(),
                "{} empty keyword",
                syntax.filetype
            );
        }
        assert_eq!(
            syntax.multiline_comment_start.is_empty(),
            syntax.multiline_comment_end.is_empty(),
            "{} comment delimiters",
            syntax.filetype
        );
        assert!(syntax.multiline_strings.len() < 255);
    }
}

#[test]
fn test_requested_languages_are_present() {
    let db = build_database();
    let present: std::collections::HashSet<&str> = db.iter().map(|s| s.filetype).collect();
    for wanted in [
        "nix",
        "python",
        "c",
        "cpp",
        "java",
        "csharp",
        "javascript",
        "typescript",
        "go",
        "rust",
        "php",
        "ruby",
        "swift",
        "kotlin",
        "dart",
        "scala",
        "r",
        "matlab",
        "julia",
        "lua",
        "perl",
        "haskell",
        "elixir",
        "erlang",
        "clojure",
        "ocaml",
        "fsharp",
        "shell",
        "powershell",
        "fish",
        "sql",
        "fortran",
        "pascal",
        "ada",
        "cobol",
        "assembly",
        "visualbasic",
        "lisp",
        "scheme",
        "prolog",
        "groovy",
        "zig",
        "nim",
        "crystal",
        "d",
        "tcl",
        "awk",
        "solidity",
        "verilog",
        "vhdl",
        "coffeescript",
        "elm",
        "vim",
        "makefile",
        "cmake",
        "dockerfile",
        "toml",
        "ini",
        "yaml",
        "json",
        "xml",
        "html",
        "css",
        "scss",
        "markdown",
        "latex",
        "diff",
        "hcl",
        "protobuf",
        "graphql",
    ] {
        assert!(present.contains(wanted), "missing language {wanted}");
    }
}

#[test]
fn test_default_theme_gives_every_class_its_own_color() {
    let theme = ThemeColors::default();
    let kinds = [
        Highlight::Normal,
        Highlight::Comment,
        Highlight::Keyword1,
        Highlight::Keyword2,
        Highlight::String,
        Highlight::Number,
        Highlight::Type,
        Highlight::Function,
        Highlight::Constant,
        Highlight::Operator,
        Highlight::Preprocessor,
        Highlight::Variable,
        Highlight::Escape,
        Highlight::Match,
    ];
    let mut colors = std::collections::HashSet::new();
    for kind in kinds {
        assert!(
            colors.insert(highlight_to_rgb(kind, &theme)),
            "{kind:?} shares a color"
        );
    }
    assert_eq!(
        highlight_to_rgb(Highlight::Comment, &theme),
        highlight_to_rgb(Highlight::MlComment, &theme)
    );
}

#[test]
fn test_html_script_and_style_are_colored_with_their_own_language() {
    let rows = highlight(
        "a.html",
        &["<script>", "var x = 1; // c", "</script>", "<p>var 1</p>"],
    );
    let script_row = "var x = 1; // c";
    assert!(kind_of(&rows[1], script_row, "var")
        .iter()
        .all(|k| *k == Highlight::Keyword2));
    assert!(kind_of(&rows[1], script_row, "1")
        .iter()
        .all(|k| *k == Highlight::Number));
    assert!(kind_of(&rows[1], script_row, "// c")
        .iter()
        .all(|k| *k == Highlight::Comment));
    assert!(rows[3].iter().all(|k| *k != Highlight::Keyword2));
    assert!(kind_of(&rows[3], "<p>var 1</p>", "1")
        .iter()
        .all(|k| *k == Highlight::Normal));
}

#[test]
fn test_embedded_language_state_survives_update_syntax_cascade() {
    let db = build_database();
    let html = select_for_filename(&db, "a.html").unwrap();

    let mut script = Buffer::new();
    for (idx, text) in ["<script>", "var x = 1;", "</script>", "var y"]
        .iter()
        .enumerate()
    {
        script.insert_row(idx, text);
    }
    update_syntax(&mut script, 0, Some(html));
    assert_eq!(script.row(1).hl()[0], Highlight::Keyword2);
    assert_eq!(script.row(1).hl_state.embed, script.row(0).hl_state.embed);
    assert_ne!(script.row(1).hl_state.embed, 0);
    assert_eq!(script.row(2).hl_state.embed, 0);

    let mut paragraph = Buffer::new();
    for (idx, text) in ["<p>", "var x = 1;", "</p>", "var y"].iter().enumerate() {
        paragraph.insert_row(idx, text);
    }
    update_syntax(&mut paragraph, 0, Some(html));
    update_syntax(&mut paragraph, 3, Some(html));
    assert_eq!(paragraph.row(1).hl()[0], Highlight::Normal);
    assert_eq!(paragraph.row(1).hl_state.embed, 0);
    assert_eq!(paragraph.row(3).hl()[0], Highlight::Normal);
}

#[test]
fn test_string_that_is_not_html_or_sql_stays_a_plain_string() {
    assert_token(
        "a.py",
        "x = \"a < b and c\"",
        "\"a < b and c\"",
        Highlight::String,
    );
    assert_token(
        "a.py",
        "x = \"Select an item from the list\"",
        "Select",
        Highlight::String,
    );
}

#[test]
fn test_embedded_regions_do_not_nest_beyond_one_level() {
    let rows = highlight(
        "a.py",
        &["x = \"\"\"<script>", "var y = 1;", "</script>\"\"\""],
    );
    assert!(kind_of(&rows[1], "var y = 1;", "var")
        .iter()
        .all(|k| *k == Highlight::Normal));
}

#[test]
fn test_markdown_fence_with_unknown_language_stays_a_string() {
    let rows = highlight("a.md", &["```nosuchlang", "def x", "```"]);
    assert!(rows[1].iter().all(|k| *k == Highlight::String));
}

#[test]
fn test_every_alias_resolves_to_a_real_language() {
    let db = build_database();
    for syntax in &db {
        for alias in syntax.aliases {
            assert!(
                !db.iter().any(|other| other.filetype == *alias),
                "alias {alias} of {} shadows a language name",
                syntax.filetype
            );
        }
    }
    for syntax in &db {
        for (_, language) in syntax.embed_tags {
            assert!(db.iter().any(|other| other.filetype == *language));
        }
        for language in syntax.string_embeds {
            assert!(db.iter().any(|other| other.filetype == *language));
        }
    }
}

#[test]
fn test_update_syntax_cascade_is_iterative_and_does_not_overflow_the_stack() {
    let handle = std::thread::Builder::new()
        .stack_size(256 * 1024)
        .spawn(|| {
            let db = build_database();
            let c = select_for_filename(&db, "a.c").unwrap();
            let mut buf = Buffer::new();
            buf.insert_row(0, "int x;");
            for idx in 1..30_000 {
                buf.insert_row(idx, "still text");
            }
            for row in 0..buf.num_rows() {
                update_syntax(&mut buf, row, Some(c));
            }
            buf.row_mut(0).set_chars("/* open");
            update_syntax(&mut buf, 0, Some(c));
            assert_eq!(buf.row(29_999).hl()[0], Highlight::MlComment);
        })
        .unwrap();
    handle.join().unwrap();
}
