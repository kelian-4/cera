use std::fs;
use std::io;

use crate::model::buffer::Buffer;

/// Ouvre `filename` dans `buffer`.
///
/// Un fichier qui n'existe pas encore donne un buffer vide portant ce nom (nouveau fichier).
/// Toute autre erreur (fichier non UTF-8, droits insuffisants, dossier...) est renvoyee et le
/// buffer n'est pas modifie : il ne porte pas le nom du fichier, donc une sauvegarde ne peut pas
/// ecraser le fichier d'origine avec un contenu vide.
pub fn open_file(buffer: &mut Buffer, filename: &str) -> io::Result<()> {
    let content = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            buffer.set_filename(filename);
            buffer.set_lines(&[]);
            return Ok(());
        }
        Err(e) => return Err(e),
    };

    buffer.set_filename(filename);

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
    Ok(())
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
        open_file(&mut buf, path).unwrap();

        assert_eq!(buf.num_rows(), 2);
        assert_eq!(buf.row(0).chars(), "line1");
        assert_eq!(buf.row(1).chars(), "line2");
        assert!(!buf.dirty());

        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_open_nonexistent_file_creates_empty_buffer() {
        let mut buf = Buffer::new();
        open_file(&mut buf, "/tmp/cera_rust_definitely_does_not_exist.txt").unwrap();

        assert_eq!(buf.num_rows(), 1);
        assert_eq!(buf.row(0).chars(), "");
        assert!(!buf.dirty());
        assert_eq!(
            buf.filename(),
            Some("/tmp/cera_rust_definitely_does_not_exist.txt")
        );
    }

    #[test]
    fn test_open_non_utf8_file_is_refused_and_not_overwritten() {
        let path = "/tmp/cera_rust_test_open_latin1.txt";
        fs::write(path, b"caf\xe9 important\n").unwrap();

        let mut buf = Buffer::new();
        let error = open_file(&mut buf, path).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert_eq!(buf.filename(), None);
        assert!(!buf.dirty());

        assert!(save_file(&mut buf).is_err());
        assert_eq!(fs::read(path).unwrap(), b"caf\xe9 important\n");

        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_open_directory_is_refused() {
        let mut buf = Buffer::new();
        assert!(open_file(&mut buf, "/tmp").is_err());
        assert_eq!(buf.filename(), None);
    }

    #[test]
    fn test_save_writes_to_disk() {
        let path = "/tmp/cera_rust_test_save.txt";
        let _ = fs::remove_file(path);

        let mut buf = Buffer::new();
        buf.set_filename(path);
        buf.set_lines(&["hello".to_string(), "world".to_string()]);

        save_file(&mut buf).unwrap();

        let content = fs::read_to_string(path).unwrap();
        assert_eq!(content, "hello\nworld\n");
        assert!(!buf.dirty());

        let _ = fs::remove_file(path);
    }
}
