use super::language_samples::{highlight_lines, SAMPLES};
use super::*;
use crate::io::json;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

const PYTHON_BRIDGE: &str = r#"
import importlib.util
import sys

sys.dont_write_bytecode = True
spec = importlib.util.spec_from_file_location("pretty_cat", sys.argv[1])
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
languages = module.load_languages()
codes = {
    "normal": "n", "comment": "c", "keyword1": "k", "keyword2": "K", "string": "s",
    "number": "d", "type": "t", "function": "f", "constant": "C", "operator": "o",
    "preprocessor": "p", "variable": "v", "escape": "e",
}
for path in sys.argv[2:]:
    sys.stdout.write("=== " + path + "\n")
    language = module.select_language(languages, path)
    if language is None:
        sys.stdout.write("NONE\n")
        continue
    with open(path, "r", encoding="utf-8", errors="replace") as f:
        lines = [line.rstrip("\n") for line in f.readlines()]
    for kinds in module.classify_lines(lines, language):
        sys.stdout.write("".join(codes[k] for k in kinds) + "\n")
"#;

fn pretty_cat_path() -> String {
    format!("{}/plugins/pretty-cat/main.py", env!("CARGO_MANIFEST_DIR"))
}

fn code(kind: Highlight) -> char {
    match kind {
        Highlight::Normal | Highlight::Match => 'n',
        Highlight::Comment | Highlight::MlComment => 'c',
        Highlight::Keyword1 => 'k',
        Highlight::Keyword2 => 'K',
        Highlight::String => 's',
        Highlight::Number => 'd',
        Highlight::Type => 't',
        Highlight::Function => 'f',
        Highlight::Constant => 'C',
        Highlight::Operator => 'o',
        Highlight::Preprocessor => 'p',
        Highlight::Variable => 'v',
        Highlight::Escape => 'e',
    }
}

fn python_available() -> bool {
    Command::new("python3")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn require_python() -> bool {
    if python_available() {
        return true;
    }
    assert!(
        std::env::var_os("CERA_REQUIRE_PLUGIN_LANGS").is_none(),
        "python3 is required but not available"
    );
    eprintln!("SKIP: python3 not available");
    false
}

fn collect_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut paths: Vec<PathBuf> = entries.filter_map(|e| e.ok()).map(|e| e.path()).collect();
    paths.sort();
    for path in paths {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if name == "target" || name == ".git" || name == "__pycache__" {
            continue;
        }
        if path.is_dir() {
            collect_files(&path, out);
        } else if std::fs::metadata(&path)
            .map(|m| m.len() < 120_000)
            .unwrap_or(false)
        {
            out.push(path);
        }
    }
}

fn run_python(paths: &[String]) -> Vec<(String, Vec<String>)> {
    let output = Command::new("python3")
        .arg("-c")
        .arg(PYTHON_BRIDGE)
        .arg(pretty_cat_path())
        .args(paths)
        .output()
        .expect("python3 should start");
    assert!(
        output.status.success(),
        "pretty-cat bridge failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8_lossy(&output.stdout).into_owned();
    let mut results: Vec<(String, Vec<String>)> = Vec::new();
    for line in text.lines() {
        if let Some(path) = line.strip_prefix("=== ") {
            results.push((path.to_string(), Vec::new()));
        } else if let Some(current) = results.last_mut() {
            current.1.push(line.to_string());
        }
    }
    results
}

fn rust_codes(database: &[Syntax], path: &str, text: &str) -> Option<Vec<String>> {
    let syntax = select_for_filename(database, path)?;
    let lines: Vec<&str> = text.lines().collect();
    Some(
        highlight_lines(syntax, &lines)
            .into_iter()
            .map(|row| row.into_iter().map(code).collect())
            .collect(),
    )
}

fn compare(database: &[Syntax], paths: &[String]) -> Vec<String> {
    let python = run_python(paths);
    assert_eq!(python.len(), paths.len(), "bridge answered for every file");
    let mut failures = Vec::new();
    for (path, python_rows) in python {
        let text = std::fs::read_to_string(&path).unwrap_or_default();
        match rust_codes(database, &path, &text) {
            None => {
                if python_rows != ["NONE"] {
                    failures.push(format!(
                        "{path}: pretty-cat selected a language, editor did not"
                    ));
                }
            }
            Some(rust_rows) => {
                if python_rows == ["NONE"] {
                    failures.push(format!(
                        "{path}: editor selected a language, pretty-cat did not"
                    ));
                    continue;
                }
                let lines: Vec<&str> = text.lines().collect();
                if python_rows.len() != rust_rows.len() {
                    failures.push(format!("{path}: line count differs"));
                    continue;
                }
                for (idx, (py, rs)) in python_rows.iter().zip(rust_rows.iter()).enumerate() {
                    if py != rs {
                        failures.push(format!(
                            "{path}:{} {:?}\n  editor     {rs}\n  pretty-cat {py}",
                            idx + 1,
                            lines[idx]
                        ));
                        break;
                    }
                }
            }
        }
    }
    failures
}

#[test]
fn pretty_cat_colors_every_language_sample_like_the_editor() {
    if !require_python() {
        return;
    }
    let database = build_database();
    let dir = std::env::temp_dir().join(format!("cera_parity_samples_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let mut paths = Vec::new();
    for (idx, sample) in SAMPLES.iter().enumerate() {
        let folder = dir.join(idx.to_string());
        std::fs::create_dir_all(&folder).unwrap();
        let file = folder.join(sample.file);
        std::fs::write(&file, sample.lines.join("\n") + "\n").unwrap();
        paths.push(file.to_string_lossy().into_owned());
    }
    let failures = compare(&database, &paths);
    let _ = std::fs::remove_dir_all(&dir);
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
}

#[test]
fn pretty_cat_colors_the_projects_own_files_like_the_editor() {
    if !require_python() {
        return;
    }
    let database = build_database();
    let mut files = Vec::new();
    collect_files(Path::new(env!("CARGO_MANIFEST_DIR")), &mut files);
    let paths: Vec<String> = files
        .iter()
        .map(|p| p.to_string_lossy().into_owned())
        .filter(|p| select_for_filename(&database, p).is_some())
        .collect();
    assert!(paths.len() > 100, "corpus too small: {}", paths.len());
    let failures = compare(&database, &paths);
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
}

#[test]
fn pretty_cat_language_data_matches_the_editor() {
    let path = format!(
        "{}/plugins/pretty-cat/languages.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let text = std::fs::read_to_string(path).expect("languages.json");
    let parsed = json::parse(&text).value.expect("valid json");
    let entries = parsed.as_array().expect("array");
    let database = build_database();
    assert_eq!(entries.len(), database.len());

    let strings = |value: Option<&json::Value>| -> Vec<String> {
        value
            .and_then(|v| v.as_array())
            .map(|a| a.iter().map(|v| v.as_string("")).collect())
            .unwrap_or_default()
    };
    let owned = |items: &[&str]| -> Vec<String> { items.iter().map(|s| s.to_string()).collect() };
    let flag_table = [
        ("numbers", HighlightFlags::NUMBERS),
        ("strings", HighlightFlags::STRINGS),
        ("functions", HighlightFlags::FUNCTIONS),
        ("capitalized_types", HighlightFlags::CAPITALIZED_TYPES),
        ("upper_constants", HighlightFlags::UPPER_CONSTANTS),
        ("case_insensitive", HighlightFlags::CASE_INSENSITIVE),
        ("paren_calls", HighlightFlags::PAREN_CALLS),
        ("markup", HighlightFlags::MARKUP),
        ("key_strings", HighlightFlags::KEY_STRINGS),
        ("key_values", HighlightFlags::KEY_VALUES),
        ("char_literals", HighlightFlags::CHAR_LITERALS),
        ("tick_strings", HighlightFlags::TICK_STRINGS),
        ("css_values", HighlightFlags::CSS_VALUES),
    ];

    for (entry, syntax) in entries.iter().zip(database.iter()) {
        let name = syntax.filetype;
        let field = |key: &str| entry.get(key).map(|v| v.as_string("")).unwrap_or_default();
        assert_eq!(field("filetype"), name);
        assert_eq!(
            strings(entry.get("filematch")),
            owned(syntax.filematch),
            "{name} filematch"
        );
        assert_eq!(
            strings(entry.get("keywords")),
            owned(syntax.keywords),
            "{name} keywords"
        );
        assert_eq!(
            strings(entry.get("types")),
            owned(syntax.types),
            "{name} types"
        );
        assert_eq!(
            strings(entry.get("constants")),
            owned(syntax.constants),
            "{name} constants"
        );
        assert_eq!(
            field("singleline_comment_start"),
            syntax.singleline_comment_start,
            "{name}"
        );
        assert_eq!(
            field("alt_comment_start"),
            syntax.alt_comment_start,
            "{name}"
        );
        assert_eq!(
            field("multiline_comment_start"),
            syntax.multiline_comment_start,
            "{name}"
        );
        assert_eq!(
            field("multiline_comment_end"),
            syntax.multiline_comment_end,
            "{name}"
        );
        assert_eq!(field("quotes"), syntax.quotes, "{name} quotes");
        assert_eq!(field("directive_prefix"), syntax.directive_prefix, "{name}");
        assert_eq!(
            strings(entry.get("annotation_prefixes")),
            owned(syntax.annotation_prefixes),
            "{name} annotations"
        );
        assert_eq!(
            field("variable_prefixes"),
            syntax.variable_prefixes,
            "{name}"
        );
        assert_eq!(field("ident_chars"), syntax.ident_chars, "{name}");
        assert_eq!(field("operators"), syntax.operators, "{name}");

        let json_strings: Vec<(String, String)> = entry
            .get("multiline_strings")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .map(|pair| {
                        let pair = pair.as_array().unwrap();
                        (pair[0].as_string(""), pair[1].as_string(""))
                    })
                    .collect()
            })
            .unwrap_or_default();
        let rust_strings: Vec<(String, String)> = syntax
            .multiline_strings
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect();
        assert_eq!(json_strings, rust_strings, "{name} multiline strings");

        let json_rules: Vec<(String, String)> = entry
            .get("line_rules")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .map(|pair| {
                        let pair = pair.as_array().unwrap();
                        (pair[0].as_string(""), pair[1].as_string("").to_lowercase())
                    })
                    .collect()
            })
            .unwrap_or_default();
        let rust_rules: Vec<(String, String)> = syntax
            .line_rules
            .iter()
            .map(|(p, k)| (p.to_string(), format!("{k:?}").to_lowercase()))
            .collect();
        assert_eq!(json_rules, rust_rules, "{name} line rules");

        let json_flags: BTreeSet<String> = strings(entry.get("flags")).into_iter().collect();
        let rust_flags: BTreeSet<String> = flag_table
            .iter()
            .filter(|(_, flag)| syntax.flags.contains(*flag))
            .map(|(label, _)| label.to_string())
            .collect();
        assert_eq!(json_flags, rust_flags, "{name} flags");
    }
}
