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

        if self.exploded {
            return write!(f, "💥 ");
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

        if self.exploded {
            return write!(f, "💥 ");
        }

        write!(f, "💣 ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_renders_an_empty_cell() {
        // arrange
        let mut cell = Cell::new();
        cell.show();

        // act
        let res = format!("{}", cell);

        // assert
        assert!(cell.is_shown());
        assert_eq!("⬜ ", res);
    }

    #[test]
    fn it_renders_a_flag() {
        // arrange
        let mut cell = Cell::new();
        cell.toggle_flag();

        // act
        let res = format!("{}", cell);

        // assert
        assert!(cell.is_flagged());
        assert_eq!("🏳 ", res);
    }

    #[test]
    fn it_renders_a_hidden_cell() {
        // arrange
        let cell = Cell::new();

        // act
        let res = format!("{}", cell);

        // assert
        assert!(!cell.is_shown());
        assert_eq!("🟧 ", res);
    }

    #[test]
    fn it_renders_a_number() {
        // arrange
        let mut cell = Cell::new();
        cell.inc_bombs_around();
        cell.inc_bombs_around();
        cell.inc_bombs_around();
        cell.show();

        // act
        let res = format!("{}", cell);

        // assert
        assert!(cell.is_shown());
        assert_eq!(3, cell.bombs_around());
        assert_eq!(" 3 ", res);
    }

    #[test]
    fn it_renders_a_bomb() {
        // arrange
        let mut cell = Cell::new();
        cell.plant_bomb();
        cell.show();

        // act
        let res = format!("{}", cell);

        // assert
        assert!(cell.is_shown());
        assert!(cell.is_mined());
        assert_eq!("💣 ", res);
    }

    #[test]
    fn it_renders_an_exploded_bomb() {
        // arrange
        let mut cell = Cell::new();
        cell.plant_bomb();
        cell.explode();
        cell.show();

        // act
        let res = format!("{}", cell);

        // assert
        assert!(cell.is_mined());
        assert!(cell.is_exploded());
        assert!(cell.is_shown());
        assert_eq!("💥 ", res);
    }

    #[test]
    fn it_creates_a_default_instance() {
        // act
        let cell = Cell::default();

        // assert
        assert!(!cell.is_exploded());
        assert!(!cell.is_shown());
        assert!(!cell.is_mined());
        assert!(!cell.is_flagged());
        assert_eq!(0, cell.bombs_around());
    }
}
