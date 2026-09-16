pub fn codepoint_count(s: &str) -> usize {
    s.chars().count()
}

pub fn byte_offset_for_char_index(s: &str, char_index: usize) -> usize {
    s.char_indices()
        .nth(char_index)
        .map(|(offset, _)| offset)
        .unwrap_or(s.len())
}

pub fn char_index_for_byte_offset(s: &str, byte_offset: usize) -> usize {
    s.char_indices()
        .take_while(|(offset, _)| *offset < byte_offset)
        .count()
}

pub fn display_width(c: char) -> usize {
    if c == '\0' {
        return 0;
    }
    unicode_display_width(c)
}

fn unicode_display_width(c: char) -> usize {
    let cp = c as u32;

    if cp == 0 {
        return 0;
    }
    if cp < 0x20 || (0x7f..0xa0).contains(&cp) {
        return 1;
    }

    let wide_ranges: &[(u32, u32)] = &[
        (0x1100, 0x115F),
        (0x2E80, 0x303E),
        (0x3041, 0x33FF),
        (0x3400, 0x4DBF),
        (0x4E00, 0x9FFF),
        (0xA000, 0xA4CF),
        (0xAC00, 0xD7A3),
        (0xF900, 0xFAFF),
        (0xFF00, 0xFF60),
        (0xFFE0, 0xFFE6),
        (0x20000, 0x2FFFD),
        (0x30000, 0x3FFFD),
        (0x1F300, 0x1F64F),
        (0x1F900, 0x1F9FF),
    ];

    for &(start, end) in wide_ranges {
        if cp >= start && cp <= end {
            return 2;
        }
    }

    1
}

pub fn substr_by_char(s: &str, from_char_idx: usize, to_char_idx: usize) -> String {
    if from_char_idx >= to_char_idx {
        return String::new();
    }
    s.chars()
        .skip(from_char_idx)
        .take(to_char_idx - from_char_idx)
        .collect()
}

fn is_word_boundary_char(c: Option<char>) -> bool {
    match c {
        None => true,
        Some(ch) => ch.is_whitespace(),
    }
}

pub fn find_word_bounded(haystack: &str, needle: &str, from_byte: usize) -> Option<usize> {
    if needle.is_empty() || from_byte > haystack.len() {
        return None;
    }

    let mut search_from = from_byte;
    while let Some(rel_pos) = haystack.get(search_from..).and_then(|s| s.find(needle)) {
        let abs_pos = search_from + rel_pos;
        let before = haystack[..abs_pos].chars().next_back();
        let after_offset = abs_pos + needle.len();
        let after = haystack.get(after_offset..).and_then(|s| s.chars().next());

        if is_word_boundary_char(before) && is_word_boundary_char(after) {
            return Some(abs_pos);
        }

        search_from = abs_pos + needle.chars().next().map(|c| c.len_utf8()).unwrap_or(1);
    }

    None
}

pub fn rfind_word_bounded(haystack: &str, needle: &str, before_byte: usize) -> Option<usize> {
    if needle.is_empty() {
        return None;
    }

    let limit = before_byte.min(haystack.len());
    let mut last_match: Option<usize> = None;
    let mut search_from = 0usize;

    while let Some(rel_pos) = haystack
        .get(search_from..limit)
        .and_then(|s| s.find(needle))
    {
        let abs_pos = search_from + rel_pos;
        if abs_pos >= limit {
            break;
        }
        let before = haystack[..abs_pos].chars().next_back();
        let after_offset = abs_pos + needle.len();
        let after = haystack.get(after_offset..).and_then(|s| s.chars().next());

        if is_word_boundary_char(before) && is_word_boundary_char(after) {
            last_match = Some(abs_pos);
        }

        search_from = abs_pos + needle.chars().next().map(|c| c.len_utf8()).unwrap_or(1);
    }

    last_match
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codepoint_count() {
        assert_eq!(codepoint_count("a\u{e9}z"), 3);
    }

    #[test]
    fn test_byte_offset_for_char_index() {
        let s = "a\u{e9}z";
        assert_eq!(byte_offset_for_char_index(s, 0), 0);
        assert_eq!(byte_offset_for_char_index(s, 1), 1);
        assert_eq!(byte_offset_for_char_index(s, 2), 3);
        assert_eq!(byte_offset_for_char_index(s, 3), 4);
    }

    #[test]
    fn test_char_index_for_byte_offset() {
        let s = "a\u{e9}z";
        assert_eq!(char_index_for_byte_offset(s, 0), 0);
        assert_eq!(char_index_for_byte_offset(s, 1), 1);
        assert_eq!(char_index_for_byte_offset(s, 3), 2);
        assert_eq!(char_index_for_byte_offset(s, 4), 3);
    }

    #[test]
    fn test_display_width_cjk() {
        assert_eq!(display_width('\u{4e2d}'), 2);
        assert_eq!(display_width('a'), 1);
    }

    #[test]
    fn test_substr_by_char() {
        let s = "hello world";
        assert_eq!(substr_by_char(s, 0, 5), "hello");
        assert_eq!(substr_by_char(s, 6, 11), "world");
    }

    #[test]
    fn test_find_word_bounded_excludes_substring() {
        let s = "monon non";
        assert_eq!(find_word_bounded(s, "non", 0), Some(6));
    }

    #[test]
    fn test_find_word_bounded_matches_at_start_and_end() {
        assert_eq!(find_word_bounded("non", "non", 0), Some(0));
        assert_eq!(find_word_bounded("hello non", "non", 0), Some(6));
        assert_eq!(find_word_bounded("non hello", "non", 0), Some(0));
    }

    #[test]
    fn test_find_word_bounded_no_match_in_larger_word() {
        assert_eq!(find_word_bounded("monon", "non", 0), None);
        assert_eq!(find_word_bounded("nonmono", "non", 0), None);
    }

    #[test]
    fn test_rfind_word_bounded() {
        let s = "non monon non";
        assert_eq!(rfind_word_bounded(s, "non", 13), Some(10));
        assert_eq!(rfind_word_bounded(s, "non", 5), Some(0));
    }
}
