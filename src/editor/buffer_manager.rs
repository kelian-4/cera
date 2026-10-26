use crate::editor::undo_stack::UndoStack;
use crate::io::file_io;
use crate::model::buffer::Buffer;
use crate::model::cursor::Cursor;
use crate::model::selection::Selection;
use crate::syntax::Syntax;

pub struct OpenBuffer {
    pub buffer: Buffer,
    pub cursor: Cursor,
    pub selection: Selection,
    pub rowoff: usize,
    pub coloff: usize,
    pub active_syntax_idx: Option<usize>,
    pub undo_stack: UndoStack,
}

impl OpenBuffer {
    fn new() -> Self {
        OpenBuffer {
            buffer: Buffer::new(),
            cursor: Cursor::default(),
            selection: Selection::default(),
            rowoff: 0,
            coloff: 0,
            active_syntax_idx: None,
            undo_stack: UndoStack::new(),
        }
    }
}

pub struct BufferManager {
    buffers: Vec<OpenBuffer>,
    active_index: usize,
}

impl BufferManager {
    pub fn new() -> Self {
        let mut mgr = BufferManager {
            buffers: Vec::new(),
            active_index: 0,
        };
        mgr.open_new_empty();
        mgr
    }

    pub fn count(&self) -> usize {
        self.buffers.len()
    }

    pub fn active_index(&self) -> usize {
        self.active_index
    }

    pub fn active(&self) -> &OpenBuffer {
        &self.buffers[self.active_index]
    }

    pub fn active_mut(&mut self) -> &mut OpenBuffer {
        &mut self.buffers[self.active_index]
    }

    pub fn at(&self, index: usize) -> &OpenBuffer {
        &self.buffers[index]
    }

    pub fn open_new_empty(&mut self) -> usize {
        self.buffers.push(OpenBuffer::new());
        self.active_index = self.buffers.len() - 1;
        self.active_index
    }

    fn find_open_buffer(&self, filename: &str) -> Option<usize> {
        let target_canonical = std::fs::canonicalize(filename).ok();
        self.buffers.iter().position(|b| {
            let Some(open_name) = b.buffer.filename() else {
                return false;
            };
            if open_name == filename {
                return true;
            }
            match (&target_canonical, std::fs::canonicalize(open_name).ok()) {
                (Some(t), Some(o)) => *t == o,
                _ => false,
            }
        })
    }

    pub fn open_file(&mut self, filename: &str, syntax_db: &[Syntax]) -> std::io::Result<usize> {
        if let Some(existing_idx) = self.find_open_buffer(filename) {
            self.active_index = existing_idx;
            return Ok(existing_idx);
        }

        let reuse_first = self.buffers.len() == 1
            && self.buffers[0].buffer.filename().is_none()
            && !self.buffers[0].buffer.dirty();

        let idx = if reuse_first {
            0
        } else {
            self.buffers.push(OpenBuffer::new());
            self.buffers.len() - 1
        };

        if let Err(error) = file_io::open_file(&mut self.buffers[idx].buffer, filename) {
            if !reuse_first {
                self.buffers.pop();
            }
            return Err(error);
        }
        let syntax_idx = syntax_db
            .iter()
            .position(|s| crate::syntax::matches(s, filename));
        self.buffers[idx].active_syntax_idx = syntax_idx;

        if let Some(sidx) = syntax_idx {
            let syn = &syntax_db[sidx];
            let num_rows = self.buffers[idx].buffer.num_rows();
            for r in 0..num_rows {
                crate::syntax::update_syntax(&mut self.buffers[idx].buffer, r, Some(syn));
            }
        }

        self.active_index = idx;
        Ok(idx)
    }

    pub fn close_active(&mut self) {
        if self.buffers.len() <= 1 {
            self.buffers[0] = OpenBuffer::new();
            self.active_index = 0;
            return;
        }
        self.buffers.remove(self.active_index);
        if self.active_index >= self.buffers.len() {
            self.active_index = self.buffers.len() - 1;
        }
    }

    pub fn next(&mut self) {
        if self.buffers.is_empty() {
            return;
        }
        self.active_index = (self.active_index + 1) % self.buffers.len();
    }

    pub fn previous(&mut self) {
        if self.buffers.is_empty() {
            return;
        }
        self.active_index = (self.active_index + self.buffers.len() - 1) % self.buffers.len();
    }

    pub fn switch_to(&mut self, index: usize) {
        if index < self.buffers.len() {
            self.active_index = index;
        }
    }

    pub fn any_dirty(&self) -> bool {
        self.buffers.iter().any(|b| b.buffer.dirty())
    }
}

impl Default for BufferManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_buffers() {
        let path1 = "/tmp/cera_rust_bm_test1.c";
        let path2 = "/tmp/cera_rust_bm_test2.py";
        std::fs::write(path1, "int main(){return 0;}\n").unwrap();
        std::fs::write(path2, "print('hi')\n").unwrap();

        let db = crate::syntax::build_database();
        let mut mgr = BufferManager::new();
        assert_eq!(mgr.count(), 1);

        mgr.open_file(path1, &db).unwrap();
        assert_eq!(mgr.count(), 1);
        assert_eq!(mgr.active().buffer.filename(), Some(path1));
        assert!(mgr.active().active_syntax_idx.is_some());

        mgr.open_file(path2, &db).unwrap();
        assert_eq!(mgr.count(), 2);
        assert_eq!(mgr.active_index(), 1);

        mgr.next();
        assert_eq!(mgr.active_index(), 0);
        mgr.next();
        assert_eq!(mgr.active_index(), 1);
        mgr.previous();
        assert_eq!(mgr.active_index(), 0);

        mgr.close_active();
        assert_eq!(mgr.count(), 1);
        assert_eq!(mgr.active().buffer.filename(), Some(path2));

        let _ = std::fs::remove_file(path1);
        let _ = std::fs::remove_file(path2);
    }

    #[test]
    fn test_open_unreadable_file_leaves_no_buffer_behind() {
        let path1 = "/tmp/cera_rust_bm_test_ok.c";
        let bad = "/tmp/cera_rust_bm_test_latin1.txt";
        std::fs::write(path1, "int a;\n").unwrap();
        std::fs::write(bad, b"caf\xe9\n").unwrap();

        let db = crate::syntax::build_database();
        let mut mgr = BufferManager::new();

        assert!(mgr.open_file(bad, &db).is_err());
        assert_eq!(mgr.count(), 1);
        assert_eq!(mgr.active().buffer.filename(), None);

        mgr.open_file(path1, &db).unwrap();
        assert!(mgr.open_file(bad, &db).is_err());
        assert_eq!(mgr.count(), 1);
        assert_eq!(mgr.active().buffer.filename(), Some(path1));

        let _ = std::fs::remove_file(path1);
        let _ = std::fs::remove_file(bad);
    }

    #[test]
    fn test_open_file_reuses_existing_buffer_instead_of_duplicating() {
        let path1 = "/tmp/cera_rust_bm_test_dup1.c";
        let path2 = "/tmp/cera_rust_bm_test_dup2.c";
        std::fs::write(path1, "int a;\n").unwrap();
        std::fs::write(path2, "int b;\n").unwrap();

        let db = crate::syntax::build_database();
        let mut mgr = BufferManager::new();

        mgr.open_file(path1, &db).unwrap();
        mgr.open_file(path2, &db).unwrap();
        assert_eq!(mgr.count(), 2);

        mgr.open_file(path1, &db).unwrap();
        assert_eq!(
            mgr.count(),
            2,
            "ouvrir un fichier deja ouvert ne doit pas creer un nouvel onglet"
        );
        assert_eq!(mgr.active_index(), 0);
        assert_eq!(mgr.active().buffer.filename(), Some(path1));

        mgr.open_file(path1, &db).unwrap();
        mgr.open_file(path1, &db).unwrap();
        assert_eq!(
            mgr.count(),
            2,
            "re-ouvrir plusieurs fois le meme fichier ne doit jamais dupliquer l'onglet"
        );

        let _ = std::fs::remove_file(path1);
        let _ = std::fs::remove_file(path2);
    }
}
