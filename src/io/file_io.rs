use std::fs;
use std::io;

use crate::model::buffer::Buffer;

pub fn open_file(buffer: &mut Buffer, filename: &str) {
    buffer.set_filename(filename);

    let content = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            buffer.set_lines(&[]);
            return;
        }
    };

    let lines: Vec<String> = content
        .split('\n')
        .map(|l| l.trim_end_matches('\r').to_string())
        .collect();

    let lines = if let Some(last) = lines.last() {
        if last.is_empty() && lines.len() > 1 {
            lines[..lines.len() - 1].to_vec()
        } else {
            lines
        }
    } else {
        lines
    };

    buffer.set_lines(&lines);
}

pub fn save_file(buffer: &mut Buffer) -> io::Result<()> {
    let filename = buffer
        .filename()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "aucun nom de fichier"))?
        .to_string();

    let content = buffer.to_string_content();
    fs::write(filename, content)?;
    buffer.clear_dirty();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_open_existing_file() {
        let path = "/tmp/cera_rust_test_open.txt";
        let mut f = fs::File::create(path).unwrap();
        writeln!(f, "line1").unwrap();
        writeln!(f, "line2").unwrap();

        let mut buf = Buffer::new();
        open_file(&mut buf, path);

        assert_eq!(buf.num_rows(), 2);
        assert_eq!(buf.row(0).chars(), "line1");
        assert_eq!(buf.row(1).chars(), "line2");
        assert!(!buf.dirty());

        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_open_nonexistent_file_creates_empty_buffer() {
        let mut buf = Buffer::new();
        open_file(&mut buf, "/tmp/cera_rust_definitely_does_not_exist.txt");

        assert_eq!(buf.num_rows(), 0);
        assert!(!buf.dirty());
        assert_eq!(
            buf.filename(),
            Some("/tmp/cera_rust_definitely_does_not_exist.txt")
        );
    }

    #[test]
    fn test_save_writes_to_disk() {
        let path = "/tmp/cera_rust_test_save.txt";
        let _ = fs::remove_file(path);

        let mut buf = Buffer::new();
        buf.set_filename(path);
        buf.insert_row(0, "hello");
        buf.insert_row(1, "world");

        save_file(&mut buf).unwrap();

        let content = fs::read_to_string(path).unwrap();
        assert_eq!(content, "hello\nworld\n");
        assert!(!buf.dirty());

        let _ = fs::remove_file(path);
    }
}
