use std::fmt::Display;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(u8)]
enum CellValue {
    Bomb,
    Empty,
    BombsAround(u8),
}

#[derive(Copy, Clone)]
pub struct Cell {
    shown: bool,
    exploded: bool,
    flagged: bool,
    value: CellValue,
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            shown: false,
            exploded: false,
            flagged: false,
            value: CellValue::Empty,
        }
    }

    pub fn is_shown(&self) -> bool {
        self.shown
    }

    pub fn is_exploded(&self) -> bool {
        self.exploded
    }

    pub fn is_flagged(&self) -> bool {
        self.flagged
    }

    pub fn is_mined(&self) -> bool {
        self.value == CellValue::Bomb
    }

    pub fn explode(&mut self) {
        self.exploded = true
    }

    pub fn bombs_around(&self) -> u8 {
        match self.value {
            CellValue::BombsAround(num) => num,
            _ => 0,
        }
    }

    pub fn show(&mut self) {
        self.shown = true;
    }

    pub fn toggle_flag(&mut self) {
        self.flagged = !self.flagged;
    }

    pub fn plant_bomb(&mut self) {
        self.value = CellValue::Bomb
    }

    pub fn inc_bombs_around(&mut self) {
        let bombs_around = self.bombs_around();
        self.value = CellValue::BombsAround(bombs_around + 1)
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.flagged {
            return write!(f, "🏳 ");
        }

        if !self.shown {
            return write!(f, "🟧 ");
        }

        if self.value == CellValue::Empty {
            return write!(f, "⬜ ");
        }

        if let CellValue::BombsAround(num) = self.value {
            return write!(f, " {} ", num);
        }

        write!(f, "💣 ")
    }
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.flagged {
            return write!(f, "🏳 ");
        }

        if self.value == CellValue::Empty {
            return write!(f, "⬜ ");
        }

        if let CellValue::BombsAround(num) = self.value {
            return write!(f, " {} ", num);
        }

        write!(f, "💣 ")
    }
}
