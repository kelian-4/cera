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
}
