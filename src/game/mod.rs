mod cell;

pub use cell::Cell;
use std::collections::HashMap;

pub type Pos = (u8, u8);

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Status {
    InProgress(i8),
    Lost,
    Won,
}

pub struct Game {
    rows: u8,
    cols: u8,
    bombs: u8,
    cells: HashMap<Pos, Cell>,
    has_lost: bool,
}

impl Game {
    pub fn new(rows: u8, cols: u8, bombs: u8) -> Self {
        let mut cells = HashMap::new();

        for x in 0..rows {
            for y in 0..cols {
                cells.insert((x, y), Cell::new());
            }
        }

        Game {
            rows,
            cols,
            bombs,
            cells,
            has_lost: false,
        }
    }

    pub fn rows(&self) -> u8 {
        self.rows
    }

    pub fn cols(&self) -> u8 {
        self.cols
    }

    pub fn bombs(&self) -> u8 {
        self.bombs
    }

    pub fn cells(&self) -> HashMap<Pos, Cell> {
        self.cells.clone()
    }

    pub fn plant_bomb(&mut self, pos: Pos) {
        self.cell_mut(pos).plant_bomb();

        for pos in self.iter_neighbors(pos) {
            let cell = self.cell_mut(pos);
            if !cell.is_mined() {
                cell.inc_bombs_around();
            }
        }
    }

    pub fn toggle_flag(&mut self, pos: Pos) -> Status {
        let cell = self.cell_mut(pos);
        if cell.is_shown() {
            return self.status();
        }

        cell.toggle_flag();

        self.status()
    }

    pub fn open(&mut self, pos: Pos) -> Status {
        let cell = self.cell_mut(pos);
        if cell.is_shown() || cell.is_flagged() {
            return self.status();
        }

        if cell.is_mined() {
            cell.explode();
            cell.show();

            self.has_lost = true;
        }

        self.sweep_mine((pos.0 as i8, pos.1 as i8));

        self.status()
    }

    fn sweep_mine(&mut self, (x, y): (i8, i8)) {
        let rows = self.rows as i8;
        let cols = self.cols as i8;

        // check board boundaries
        if x < 0 || y < 0 || rows - 1 < x || cols - 1 < y {
            return;
        }

        let cell = self.cell_mut((x as u8, y as u8));
        if cell.is_shown() || cell.is_mined() || cell.is_flagged() {
            return;
        }

        cell.show();

        if cell.bombs_around() < 1 {
            self.show_empty_neighbours(x, y);
        }
    }

    fn show_empty_neighbours(&mut self, x: i8, y: i8) {
        let neighbours = [0, 1, -1];

        for i in neighbours {
            for j in neighbours {
                self.sweep_mine((x + i, y + j));
            }
        }
    }

    pub fn cell(&self, pos: Pos) -> &Cell {
        self.cells
            .get(&pos)
            .unwrap_or_else(|| panic!("cell at ({}, {}) does not exist", pos.0, pos.1))
    }

    fn cell_mut(&mut self, pos: Pos) -> &mut Cell {
        self.cells
            .get_mut(&pos)
            .unwrap_or_else(|| panic!("cell at ({}, {}) does not exist", pos.0, pos.1))
    }

    fn status(&self) -> Status {
        if self.has_lost {
            return Status::Lost;
        }

        if self.is_won() {
            return Status::Won;
        }

        Status::InProgress(self.flags_left())
    }

    fn flags_left(&self) -> i8 {
        let flagged_cells = self
            .cells
            .iter()
            .filter(|(_, cell)| cell.is_flagged())
            .count() as i8;

        self.bombs as i8 - flagged_cells
    }

    fn is_won(&self) -> bool {
        let cleared_cells = self
            .cells
            .iter()
            .filter(|(_, cell)| cell.is_shown())
            .count() as u8;

        if self.rows * self.cols - cleared_cells == self.bombs {
            return true;
        }

        false
    }

    fn iter_neighbors(&self, (x, y): Pos) -> impl Iterator<Item = Pos> {
        let rows = self.rows;
        let cols = self.cols;

        (x.max(1) - 1..=(x + 1).min(rows - 1))
            .flat_map(move |i| {
                (y.max(1) - 1..=(y + 1).min(cols - 1)).map(move |j| (i as u8, j as u8))
            })
            .filter(move |&pos| pos != (x, y))
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cells = self.cells();

        for x in 0..self.rows() {
            for y in 0..self.cols() {
                let pos = (x, y);
                let cell = cells.get(&pos).unwrap();

                write!(f, "{} ", cell)?
            }

            writeln!(f)?
        }

        Ok(())
    }
}

impl std::fmt::Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cells = self.cells();

        for x in 0..self.rows() {
            for y in 0..self.cols() {
                let pos = (x, y);
                let cell = cells.get(&pos).unwrap();

                write!(f, "{:?} ", cell)?
            }

            writeln!(f)?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_bombs_num_is_correct() {
        // arrange
        let bombs = 3;
        let sut = Game::new(3, 3, bombs);

        // act
        let got = sut.bombs();

        // assert
        assert_eq!(bombs, got);
    }

    #[test]
    fn rows_num_is_correct() {
        // arrange
        let rows = 3;
        let sut = Game::new(rows, 3, 3);

        // act
        let got = sut.rows();

        // assert
        assert_eq!(rows, got);
    }

    #[test]
    fn cols_num_is_correct() {
        // arrange
        let cols = 3;
        let sut = Game::new(3, cols, 3);

        // act
        let got = sut.cols();

        // assert
        assert_eq!(cols, got);
    }

    #[test]
    fn cell_is_flagged() {
        // arrange
        let mut sut = Game::new(3, 3, 3);

        // act
        let status = sut.toggle_flag((0, 0));

        // assert
        assert_eq!(Status::InProgress(2), status);
        assert!(sut.cell((0, 0)).is_flagged());
    }

    #[test]
    fn nothing_happens_if_an_open_cell_is_being_flagged() {
        // arrange
        let mut sut = Game::new(3, 3, 3);
        sut.open((0, 0));

        // act
        let status = sut.toggle_flag((0, 0));

        // assert
        assert_eq!(Status::InProgress(3), status);
        assert!(!sut.cell((0, 0)).is_flagged());
    }

    #[test]
    fn bomb_is_planted() {
        // arrange
        let mut sut = Game::new(3, 3, 1);
        let pos = (0, 0);

        // act
        sut.plant_bomb(pos);

        // assert
        assert!(sut.cell(pos).is_mined());
    }

    #[test]
    fn cell_is_revealed_once_it_is_open() {
        // arrange
        let mut sut = Game::new(3, 3, 3);
        let pos = (0, 0);

        // act
        let status = sut.open(pos);

        // assert
        assert_eq!(Status::InProgress(3), status);
        assert!(sut.cell(pos).is_shown());
    }

    #[test]
    fn game_is_lost_if_a_cell_with_a_bomb_is_opened() {
        // arrange
        let mut sut = Game::new(3, 3, 1);
        let pos = (0, 0);
        sut.plant_bomb(pos);

        // act
        let status = sut.open(pos);

        // assert
        assert_eq!(Status::Lost, status);
        assert!(sut.cell(pos).is_shown());
        assert!(sut.cell(pos).is_mined());
        assert!(sut.cell(pos).is_exploded());
    }

    #[test]
    fn game_is_won_if_all_bombs_are_flagged() {
        // arrange
        let mut sut = Game::new(3, 3, 2);
        sut.plant_bomb((0, 0));
        sut.plant_bomb((2, 2));

        // act
        sut.open((0, 1));
        sut.open((0, 2));
        sut.open((1, 0));
        sut.open((1, 1));
        sut.open((1, 2));
        sut.open((2, 0));
        sut.open((2, 1));

        sut.toggle_flag((0, 0));
        let status = sut.toggle_flag((2, 2));

        // assert
        assert_eq!(Status::Won, status)
    }

    #[test]
    fn game_is_won_if_all_non_bomb_cells_are_open() {
        // arrange
        let mut sut = Game::new(3, 3, 2);
        sut.plant_bomb((0, 0));
        sut.plant_bomb((2, 2));

        // act
        sut.open((0, 1));
        sut.open((0, 2));
        sut.open((1, 0));
        sut.open((1, 1));
        sut.open((1, 2));
        sut.open((2, 0));

        let status = sut.open((2, 1));

        // assert
        assert_eq!(Status::Won, status)
    }

    #[test]
    fn nothing_happens_if_a_flagged_cell_with_a_bomb_is_being_opened() {
        // arrange
        let mut sut = Game::new(3, 3, 1);
        let pos = (0, 0);

        sut.plant_bomb(pos);
        sut.toggle_flag(pos);

        // act
        let status = sut.open(pos);

        // assert
        assert_eq!(Status::InProgress(0), status);
        assert_eq!(false, sut.cell(pos).is_shown());
        assert!(sut.cell(pos).is_mined());
        assert!(sut.cell(pos).is_flagged());
    }

    #[test]
    fn there_are_no_bombs_around() {
        // arrange
        let mut sut = Game::new(3, 3, 0);

        // act
        sut.open((0, 0));

        // assert
        let bombs_around = sut.cell((0, 0)).bombs_around();
        assert_eq!(0, bombs_around);
    }

    #[test]
    fn there_is_one_bomb_around() {
        // arrange
        let mut sut = Game::new(3, 3, 1);
        sut.plant_bomb((0, 0));

        // act
        sut.open((1, 1));

        // assert
        let bombs_around = sut.cell((1, 1)).bombs_around();
        assert_eq!(1, bombs_around);
    }

    #[test]
    fn there_are_two_bombs_around() {
        // arrange
        let mut sut = Game::new(3, 3, 2);
        sut.plant_bomb((0, 0)); // top left
        sut.plant_bomb((2, 2)); // bottom right

        // act
        sut.open((1, 1));

        // assert
        let bombs_around = sut.cell((1, 1)).bombs_around();
        assert_eq!(2, bombs_around);
    }

    #[test]
    fn there_are_thee_bombs_around() {
        // arrange
        let mut sut = Game::new(3, 3, 3);
        sut.plant_bomb((0, 0)); // top left
        sut.plant_bomb((0, 1)); // top center
        sut.plant_bomb((2, 2)); // bottom right

        // act
        sut.open((1, 1));

        // assert
        let bombs_around = sut.cell((1, 1)).bombs_around();
        assert_eq!(3, bombs_around);
    }

    #[test]
    fn there_are_four_bombs_around() {
        // arrange
        let mut sut = Game::new(3, 3, 4);
        sut.plant_bomb((0, 0)); // top left
        sut.plant_bomb((0, 1)); // top center
        sut.plant_bomb((0, 2)); // top right
        sut.plant_bomb((2, 2)); // bottom right

        // act
        sut.open((1, 1));

        // assert
        let bombs_around = sut.cell((1, 1)).bombs_around();
        assert_eq!(4, bombs_around);
    }

    #[test]
    fn there_are_five_bombs_around() {
        // arrange
        let mut sut = Game::new(3, 3, 5);
        sut.plant_bomb((0, 0)); // top left
        sut.plant_bomb((0, 1)); // top center
        sut.plant_bomb((0, 2)); // top right
        sut.plant_bomb((1, 2)); // center right
        sut.plant_bomb((2, 2)); // bottom right

        // act
        sut.open((1, 1));

        // assert
        let bombs_around = sut.cell((1, 1)).bombs_around();
        assert_eq!(5, bombs_around);
    }

    #[test]
    fn there_are_six_bombs_around() {
        // arrange
        let mut sut = Game::new(3, 3, 6);
        sut.plant_bomb((0, 0)); // top left
        sut.plant_bomb((0, 1)); // top center
        sut.plant_bomb((0, 2)); // top right
        sut.plant_bomb((1, 2)); // center right
        sut.plant_bomb((2, 1)); // center bottom
        sut.plant_bomb((2, 2)); // bottom right

        // act
        sut.open((1, 1));

        // assert
        let bombs_around = sut.cell((1, 1)).bombs_around();
        assert_eq!(6, bombs_around);
    }

    #[test]
    fn there_are_seven_bombs_around() {
        // arrange
        let mut sut = Game::new(3, 3, 7);

        sut.plant_bomb((0, 0)); // top left
        sut.plant_bomb((0, 1)); // top center
        sut.plant_bomb((0, 2)); // top right
        sut.plant_bomb((1, 2)); // center right
        sut.plant_bomb((2, 0)); // bottom left
        sut.plant_bomb((2, 1)); // center bottom
        sut.plant_bomb((2, 2)); // bottom right

        // act
        sut.open((1, 1));

        // assert
        let bombs_around = sut.cell((1, 1)).bombs_around();
        assert_eq!(7, bombs_around);
    }

    #[test]
    fn there_are_eight_bombs_around() {
        // arrange
        let mut sut = Game::new(3, 3, 8);
        sut.plant_bomb((0, 0)); // top left
        sut.plant_bomb((0, 1)); // top center
        sut.plant_bomb((0, 2)); // top right
        sut.plant_bomb((1, 0)); // center left
        sut.plant_bomb((1, 2)); // center right
        sut.plant_bomb((2, 0)); // bottom left
        sut.plant_bomb((2, 1)); // center bottom
        sut.plant_bomb((2, 2)); // bottom right

        // act
        sut.open((1, 1));

        // assert
        let bombs_around = sut.cell((1, 1)).bombs_around();
        assert_eq!(8, bombs_around);
    }

    #[test]
    fn empty_neighbours_are_shown_when_an_empty_cell_is_open() {
        // arrange
        let rows = 4;
        let cols = 4;

        let mut sut = Game::new(rows, cols, 0);

        // act
        sut.plant_bomb((3, 3)); // bottom right

        sut.open((0, 0)); // top left

        // assert
        // all the cells, but the bottom right must be shown
        for x in 0..rows {
            for y in 0..cols {
                if x != 3 && y != 3 {
                    assert!(sut.cell((x, y)).is_shown());
                }
            }
        }
    }
}
