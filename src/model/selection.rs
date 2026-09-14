#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }
}

fn position_less(a: Position, b: Position) -> bool {
    if a.row != b.row {
        a.row < b.row
    } else {
        a.col < b.col
    }
}

#[derive(Clone, Copy, Default)]
pub struct Selection {
    pub anchor: Position,
    pub active: Position,
    pub is_active: bool,
}

impl Selection {
    pub fn start(&self) -> Position {
        if position_less(self.anchor, self.active) {
            self.anchor
        } else {
            self.active
        }
    }

    pub fn end(&self) -> Position {
        if position_less(self.anchor, self.active) {
            self.active
        } else {
            self.anchor
        }
    }

    pub fn is_empty(&self) -> bool {
        self.anchor == self.active
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_range() {
        let sel = Selection {
            anchor: Position::new(2, 5),
            active: Position::new(1, 3),
            is_active: true,
        };

        let start = sel.start();
        let end = sel.end();
        assert_eq!(start, Position::new(1, 3));
        assert_eq!(end, Position::new(2, 5));
        assert!(!sel.is_empty());
    }
}
