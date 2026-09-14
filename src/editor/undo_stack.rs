use crate::model::selection::Position;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EditKind {
    InsertChar,
    DeleteCharBackward,
    DeleteCharForward,
    InsertNewline,
    InsertBlock,
    DeleteBlock,
    ReplaceBlock,
    Other,
}

#[derive(Clone)]
pub struct EditAction {
    pub start: Position,
    pub removed_text: String,
    pub inserted_text: String,
    pub cursor_before: Position,
    pub cursor_after: Position,
    pub kind: EditKind,
}

impl Default for EditAction {
    fn default() -> Self {
        EditAction {
            start: Position::default(),
            removed_text: String::new(),
            inserted_text: String::new(),
            cursor_before: Position::default(),
            cursor_after: Position::default(),
            kind: EditKind::Other,
        }
    }
}

#[derive(Default)]
pub struct UndoStack {
    undo_list: Vec<EditAction>,
    redo_list: Vec<EditAction>,
    coalescing_broken: bool,
}

impl UndoStack {
    pub fn new() -> Self {
        UndoStack {
            undo_list: Vec::new(),
            redo_list: Vec::new(),
            coalescing_broken: true,
        }
    }

    fn can_coalesce(&self, next: &EditAction) -> bool {
        if self.coalescing_broken {
            return false;
        }
        let last = match self.undo_list.last() {
            Some(l) => l,
            None => return false,
        };

        let same_kind = last.kind == next.kind;
        let simple_insert = next.kind == EditKind::InsertChar
            && last.kind == EditKind::InsertChar
            && last.removed_text.is_empty()
            && next.removed_text.is_empty();
        let simple_backspace = next.kind == EditKind::DeleteCharBackward
            && last.kind == EditKind::DeleteCharBackward
            && last.inserted_text.is_empty()
            && next.inserted_text.is_empty();
        let simple_forward_delete = next.kind == EditKind::DeleteCharForward
            && last.kind == EditKind::DeleteCharForward
            && last.inserted_text.is_empty()
            && next.inserted_text.is_empty();

        if !same_kind || !(simple_insert || simple_backspace || simple_forward_delete) {
            return false;
        }

        last.cursor_after == next.start
            || (next.kind == EditKind::DeleteCharBackward && next.start == last.start)
    }

    pub fn push(&mut self, action: EditAction) {
        self.redo_list.clear();

        if self.can_coalesce(&action) {
            let last = self.undo_list.last_mut().unwrap();
            match action.kind {
                EditKind::InsertChar => {
                    last.inserted_text.push_str(&action.inserted_text);
                }
                EditKind::DeleteCharBackward => {
                    last.removed_text = format!("{}{}", action.removed_text, last.removed_text);
                    last.start = action.start;
                }
                EditKind::DeleteCharForward => {
                    last.removed_text.push_str(&action.removed_text);
                }
                _ => {}
            }
            last.cursor_after = action.cursor_after;
            return;
        }

        self.undo_list.push(action);
        self.coalescing_broken = false;
    }

    pub fn break_coalescing(&mut self) {
        self.coalescing_broken = true;
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_list.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_list.is_empty()
    }

    pub fn undo(&mut self) -> Option<EditAction> {
        let action = self.undo_list.pop()?;
        self.redo_list.push(action.clone());
        self.coalescing_broken = true;
        Some(action)
    }

    pub fn redo(&mut self) -> Option<EditAction> {
        let action = self.redo_list.pop()?;
        self.undo_list.push(action.clone());
        self.coalescing_broken = true;
        Some(action)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coalescing() {
        let mut stack = UndoStack::new();

        let a1 = EditAction {
            kind: EditKind::InsertChar,
            start: Position::new(0, 0),
            inserted_text: "a".to_string(),
            cursor_before: Position::new(0, 0),
            cursor_after: Position::new(0, 1),
            ..Default::default()
        };
        stack.push(a1);

        let a2 = EditAction {
            kind: EditKind::InsertChar,
            start: Position::new(0, 1),
            inserted_text: "b".to_string(),
            cursor_before: Position::new(0, 1),
            cursor_after: Position::new(0, 2),
            ..Default::default()
        };
        stack.push(a2);

        assert!(stack.can_undo());
        let undone = stack.undo().unwrap();
        assert_eq!(undone.inserted_text, "ab");

        assert!(!stack.can_undo());
        assert!(stack.can_redo());

        stack.break_coalescing();
        let a3 = EditAction {
            kind: EditKind::InsertChar,
            start: Position::new(0, 0),
            inserted_text: "c".to_string(),
            cursor_before: Position::new(0, 0),
            cursor_after: Position::new(0, 1),
            ..Default::default()
        };
        stack.push(a3);
        assert!(!stack.can_redo());
    }
}
