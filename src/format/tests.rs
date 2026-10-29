use super::*;
use crate::syntax;

fn buffer_for(filetype: &str, src: &str) -> Buffer {
    let mut b = Buffer::new();
    let lines: Vec<String> = src.lines().map(|l| l.to_string()).collect();
    b.set_lines(&lines);
    let syn = syntax::database()
        .iter()
        .find(|s| s.filetype == filetype)
        .expect("language");
    for r in 0..b.num_rows() {
        syntax::update_syntax(&mut b, r, Some(syn));
    }
    b
}

fn fmt(filetype: &str, src: &str) -> String {
    let b = buffer_for(filetype, src);
    let f = format_buffer(&b, filetype, 8).expect("formatted");
    f.rows
        .iter()
        .map(|r| r.text.clone())
        .collect::<Vec<_>>()
        .join("\n")
}

fn reindented(filetype: &str, src: &str) -> bool {
    let b = buffer_for(filetype, src);
    format_buffer(&b, filetype, 8).is_some_and(|f| f.reindented)
}

#[test]
fn rust_reindents_blocks() {
    let src = "fn main() {\nlet x = 1;\n    if x > 0 {\n  println!(\"a\");\n        }\n}\n";
    let out = fmt("rust", src);
    assert_eq!(
        out,
        "fn main() {\n    let x = 1;\n    if x > 0 {\n        println!(\"a\");\n    }\n}"
    );
}

#[test]
fn brackets_in_strings_and_comments_are_ignored() {
    let src = "fn f() {\nlet s = \"}\"; // {\n   g();\n}\n";
    let out = fmt("rust", src);
    assert_eq!(out, "fn f() {\n    let s = \"}\"; // {\n    g();\n}");
}

#[test]
fn closing_line_with_several_brackets_returns_to_opener_level() {
    let src = "foo(function() {\nbar();\n});\n";
    let out = fmt("javascript", src);
    assert_eq!(out, "foo(function() {\n  bar();\n});");
}

#[test]
fn else_stays_on_the_level_of_its_if() {
    let src = "function f() {\n  if (a) {\n  b();\n      } else {\n    c();\n  }\n}\n";
    let out = fmt("javascript", src);
    assert_eq!(
        out,
        "function f() {\n  if (a) {\n    b();\n  } else {\n    c();\n  }\n}"
    );
}

#[test]
fn switch_labels_and_bodies() {
    let src = "void f(int x) {\nswitch (x) {\ncase 1:\nfoo();\nbreak;\ndefault:\nbar();\n}\n}\n";
    let out = fmt("c", src);
    assert_eq!(
        out,
        "void f(int x) {\n    switch (x) {\n        case 1:\n            foo();\n            break;\n        default:\n            bar();\n    }\n}"
    );
}

#[test]
fn braceless_if_indents_one_statement() {
    let src = "void f() {\nif (a)\nfoo();\nbar();\n}\n";
    let out = fmt("c", src);
    assert_eq!(out, "void f() {\n    if (a)\n        foo();\n    bar();\n}");
}

#[test]
fn method_chain_lines_are_indented() {
    let src = "const x = a\n.b()\n.c();\nnext();\n";
    let out = fmt("javascript", src);
    assert_eq!(out, "const x = a\n  .b()\n  .c();\nnext();");
}

#[test]
fn continuation_after_operator() {
    let src = "const total =\na + b;\nnext();\n";
    let out = fmt("javascript", src);
    assert_eq!(out, "const total =\n  a + b;\nnext();");
}

#[test]
fn increment_is_not_a_continuation() {
    let src = "function f() {\ni++;\nj--;\nk();\n}\n";
    let out = fmt("javascript", src);
    assert_eq!(out, "function f() {\n  i++;\n  j--;\n  k();\n}");
}

#[test]
fn designated_initializers_are_not_chains() {
    let src = "struct S s = {\n.a = 1,\n.b = 2,\n};\n";
    let out = fmt("c", src);
    assert_eq!(out, "struct S s = {\n    .a = 1,\n    .b = 2,\n};");
}

#[test]
fn block_comments_follow_their_opener() {
    let src = "int f() {\n/*\n * text\n */\nreturn 0;\n}\n";
    let out = fmt("c", src);
    assert_eq!(
        out,
        "int f() {\n    /*\n     * text\n     */\n    return 0;\n}"
    );
}

#[test]
fn preprocessor_stays_in_column_zero() {
    let src = "int f() {\n    #ifdef X\n    return 1;\n    #endif\n}\n";
    let b = buffer_for("c", src);
    let f = format_buffer(&b, "c", 8).unwrap();
    assert!(f.rows[1].text.starts_with("#ifdef"));
}

#[test]
fn go_uses_tabs() {
    let src = "func main() {\n    if x {\n        run()\n    }\n}\n";
    let out = fmt("go", src);
    assert_eq!(out, "func main() {\n\tif x {\n\t\trun()\n\t}\n}");
}

#[test]
fn unbalanced_file_only_gets_hygiene() {
    let src = "fn f() {\n   let x = 1;   \n\n\n\n   foo();\n";
    let b = buffer_for("rust", src);
    let f = format_buffer(&b, "rust", 8).unwrap();
    assert!(!f.reindented);
    let texts: Vec<_> = f.rows.iter().map(|r| r.text.as_str()).collect();
    assert_eq!(texts, ["fn f() {", "   let x = 1;", "", "   foo();"]);
}

#[test]
fn blank_lines_are_collapsed_and_edges_removed() {
    let out = fmt("rust", "\n\nfn a() {}\n\n\n\nfn b() {}\n\n\n");
    assert_eq!(out, "fn a() {}\n\nfn b() {}");
}

#[test]
fn json_is_reindented() {
    let src = "{\n  \"a\": [\n1,\n2\n],\n  \"b\": {\n\"c\": 1\n}\n}\n";
    let out = fmt("json", src);
    assert_eq!(
        out,
        "{\n  \"a\": [\n    1,\n    2\n  ],\n  \"b\": {\n    \"c\": 1\n  }\n}"
    );
}

#[test]
fn css_is_reindented() {
    let src = "a {\ncolor: red;\n}\n@media (x) {\n.b {\ncolor: blue;\n}\n}\n";
    let out = fmt("css", src);
    assert_eq!(
        out,
        "a {\n  color: red;\n}\n@media (x) {\n  .b {\n    color: blue;\n  }\n}"
    );
}

#[test]
fn jsx_is_left_alone() {
    let src = "const a = (\n<div>\n<p/>\n</div>\n);\n";
    assert!(!reindented("javascript", src));
}

#[test]
fn ruby_blocks_and_middles() {
    let src = "class A\ndef f(x)\nif x\ny\nelsif z\nw\nelse\nv\nend\nend\nend\n";
    let out = fmt("ruby", src);
    assert_eq!(
        out,
        "class A\n  def f(x)\n    if x\n      y\n    elsif z\n      w\n    else\n      v\n    end\n  end\nend"
    );
}

#[test]
fn ruby_modifier_if_does_not_open_a_block() {
    let src = "def f\nreturn 1 if x\nfoo do |a|\nbar\nend\nend\n";
    let out = fmt("ruby", src);
    assert_eq!(
        out,
        "def f\n  return 1 if x\n  foo do |a|\n    bar\n  end\nend"
    );
}

#[test]
fn ruby_heredoc_body_is_untouched() {
    let src = "def f\ntext = <<~EOS\n   keep   me\nEOS\nend\n";
    let out = fmt("ruby", src);
    assert_eq!(out, "def f\n  text = <<~EOS\n   keep   me\nEOS\nend");
}

#[test]
fn lua_functions_and_loops() {
    let src = "local function f(a)\nfor i = 1, 3 do\nprint(i)\nend\nif a then\nreturn 1\nelseif b then\nreturn 2\nend\nend\n";
    let out = fmt("lua", src);
    assert_eq!(
        out,
        "local function f(a)\n    for i = 1, 3 do\n        print(i)\n    end\n    if a then\n        return 1\n    elseif b then\n        return 2\n    end\nend"
    );
}

#[test]
fn shell_blocks_and_case() {
    let src = "if [ -f x ]; then\necho a\nelse\necho b\nfi\ncase $1 in\na)\necho A\n;;\n*)\necho other\n;;\nesac\nfor i in 1 2; do\necho $i\ndone\n";
    let out = fmt("shell", src);
    assert_eq!(
        out,
        "if [ -f x ]; then\n    echo a\nelse\n    echo b\nfi\ncase $1 in\n    a)\n        echo A\n        ;;\n    *)\n        echo other\n        ;;\nesac\nfor i in 1 2; do\n    echo $i\ndone"
    );
}

#[test]
fn shell_words_that_look_like_keywords_are_ignored() {
    let src = "echo done\nif true; then\necho fi\nfi\n";
    let out = fmt("shell", src);
    assert_eq!(out, "echo done\nif true; then\n    echo fi\nfi");
}

#[test]
fn python_normalizes_to_four_spaces() {
    let src = "def f(x):\n  if x:\n    return 1\n  else:\n    return 2\n";
    let out = fmt("python", src);
    assert_eq!(
        out,
        "def f(x):\n    if x:\n        return 1\n    else:\n        return 2"
    );
}

#[test]
fn python_continuation_lines_keep_their_alignment() {
    let src = "def f(a,\n      b):\n  return foo(a,\n             b)\n";
    let out = fmt("python", src);
    assert_eq!(
        out,
        "def f(a,\n      b):\n    return foo(a,\n               b)"
    );
}

#[test]
fn python_keeps_two_blank_lines_and_docstrings() {
    let src = "def a():\n  \"\"\"doc\n     more\n  \"\"\"\n  pass\n\n\n\ndef b():\n  pass\n";
    let out = fmt("python", src);
    assert_eq!(
        out,
        "def a():\n    \"\"\"doc\n     more\n  \"\"\"\n    pass\n\n\ndef b():\n    pass"
    );
}

#[test]
fn python_comment_follows_next_statement() {
    let src = "if x:\n      # note\n      y = 1\n";
    let out = fmt("python", src);
    assert_eq!(out, "if x:\n    # note\n    y = 1");
}

#[test]
fn yaml_normalizes_to_two_spaces() {
    let src = "a:\n    b:\n        c: 1\n    d: 2\nlist:\n    - x: 1\n      y: 2\n    - z\n";
    let out = fmt("yaml", src);
    assert_eq!(
        out,
        "a:\n  b:\n    c: 1\n  d: 2\nlist:\n  - x: 1\n    y: 2\n  - z"
    );
}

#[test]
fn yaml_block_scalars_move_with_their_key() {
    let src = "a:\n    text: |\n        hello\n          world\n    next: 1\n";
    let out = fmt("yaml", src);
    assert_eq!(out, "a:\n  text: |\n      hello\n        world\n  next: 1");
}

#[test]
fn multiline_strings_are_untouched() {
    let src = "const a = `\n   keep   \n`;\nfoo();   \n";
    let out = fmt("javascript", src);
    assert_eq!(out, "const a = `\n   keep   \n`;\nfoo();");
}

#[test]
fn markdown_and_diff_are_not_formatted() {
    let b = buffer_for("markdown", "text  \n\n\n\nmore\n");
    assert!(format_buffer(&b, "markdown", 8).is_none());
}

#[test]
fn output_is_stable_when_formatted_twice() {
    let src = "fn main() {\nlet x = 1;\nif x > 0 {\nfoo(\n1,\n2\n);\n}\n}\n";
    let once = fmt("rust", src);
    let twice = fmt("rust", &once);
    assert_eq!(once, twice);
}

#[test]
fn html_nesting_and_void_elements() {
    let src = "<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"utf-8\">\n<title>T</title>\n</head>\n<body>\n<div>\n<br>\n<p>text <b>bold</b></p>\n</div>\n</body>\n</html>\n";
    let out = fmt("html", src);
    assert_eq!(
        out,
        "<!DOCTYPE html>\n<html>\n  <head>\n    <meta charset=\"utf-8\">\n    <title>T</title>\n  </head>\n  <body>\n    <div>\n      <br>\n      <p>text <b>bold</b></p>\n    </div>\n  </body>\n</html>"
    );
}

#[test]
fn html_multiline_tag_attributes() {
    let src = "<div\nclass=\"a\"\nid=\"b\"\n>\n<span>x</span>\n</div>\n";
    let out = fmt("html", src);
    assert_eq!(
        out,
        "<div\n  class=\"a\"\n  id=\"b\"\n>\n  <span>x</span>\n</div>"
    );
}

#[test]
fn html_pre_script_and_style_are_untouched() {
    let src = "<div>\n<pre>\n  keep   \n    this\n</pre>\n<script>\nif (a) {\n      b();\n}\n</script>\n</div>\n";
    let out = fmt("html", src);
    assert_eq!(
        out,
        "<div>\n  <pre>\n  keep   \n    this\n  </pre>\n  <script>\nif (a) {\n      b();\n}\n  </script>\n</div>"
    );
}

#[test]
fn html_comments_move_with_their_opener() {
    let src = "<div>\n<!--\n   note\n-->\n<p>x</p>\n</div>\n";
    let out = fmt("html", src);
    assert_eq!(out, "<div>\n  <!--\n     note\n  -->\n  <p>x</p>\n</div>");
}

#[test]
fn xml_self_closing_and_declaration() {
    let src = "<?xml version=\"1.0\"?>\n<root>\n<a>\n<b/>\n<c x=\"1\"/>\n</a>\n</root>\n";
    let out = fmt("xml", src);
    assert_eq!(
        out,
        "<?xml version=\"1.0\"?>\n<root>\n  <a>\n    <b/>\n    <c x=\"1\"/>\n  </a>\n</root>"
    );
}

#[test]
fn html_with_unclosed_optional_tags_is_left_alone() {
    let b = buffer_for("html", "<ul>\n<li>one\n<li>two\n</ul>\n");
    assert!(format_buffer(&b, "html", 8).is_none());
}

#[test]
fn c_aligned_arguments_keep_their_alignment() {
    let src = "void f() {\nprintf(\"%d %d\",\n       a, b);\nstream << x\n       << y;\n}\n";
    let out = fmt("c", src);
    assert_eq!(
        out,
        "void f() {\n    printf(\"%d %d\",\n           a, b);\n    stream << x\n           << y;\n}"
    );
}

#[test]
fn include_directive_is_not_an_open_string() {
    let src = "#include <stdio.h>\nint main() {\nreturn 0;\n}\n";
    let out = fmt("c", src);
    assert_eq!(out, "#include <stdio.h>\nint main() {\n    return 0;\n}");
}
