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

    pub fn open_file(&mut self, filename: &str, syntax_db: &[Syntax]) -> usize {
        let reuse_first = self.buffers.len() == 1
            && self.buffers[0].buffer.filename().is_none()
            && !self.buffers[0].buffer.dirty();

        let idx = if reuse_first {
            0
        } else {
            self.buffers.push(OpenBuffer::new());
            self.buffers.len() - 1
        };

        file_io::open_file(&mut self.buffers[idx].buffer, filename);
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
        idx
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

        mgr.open_file(path1, &db);
        assert_eq!(mgr.count(), 1);
        assert_eq!(mgr.active().buffer.filename(), Some(path1));
        assert!(mgr.active().active_syntax_idx.is_some());

        mgr.open_file(path2, &db);
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
}
