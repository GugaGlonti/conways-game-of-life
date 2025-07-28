use rand::Rng;

use crate::ui::cell::Cell;

#[derive(Debug)]
pub struct Grid {
    matrix: Vec<Vec<Cell>>,
    interim_matrix: Vec<Vec<Cell>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut matrix = vec![];
        for _ in 0..rows {
            let mut row: Vec<Cell> = vec![];
            for _ in 0..cols {
                row.push(Cell::Dead);
            }
            matrix.push(row);
        }

        let mut interim_matrix = vec![];
        for _ in 0..rows {
            let mut row: Vec<Cell> = vec![];
            for _ in 0..cols {
                row.push(Cell::Dead);
            }
            interim_matrix.push(row);
        }

        Grid {
            matrix,
            rows,
            cols,
            interim_matrix,
        }
    }

    pub fn awake_many(&mut self, points: Vec<(usize, usize)>) -> () {
        points
            .iter()
            .for_each(|(x, y)| self.matrix[*y][*x] = Cell::Alive);
    }

    pub fn init_draw(&self) {
        let width = self.cols * 3 + 2;
        let mut out: String = String::from('┌');
        out.extend(std::iter::repeat('-').take(width));
        out.push_str("┐\n");
        for _ in 0..self.rows {
            out.push_str("│ ");
            out.extend(std::iter::repeat("░░░").take(width / 3));
            out.push_str(" │\n");
        }
        out.push('└');
        out.extend(std::iter::repeat('-').take(width));
        println!("{out}┘\n");
    }

    fn alive_neighbours(&self, x_anchor: usize, y_anchor: usize) -> usize {
        let mut count = 0;
        for y in (y_anchor - 1)..(y_anchor + 2) {
            for x in (x_anchor - 1)..(x_anchor + 2) {
                if y == y_anchor && x == x_anchor {
                    continue;
                }
                match self.matrix[y][x] {
                    Cell::Alive => count += 1,
                    _ => (),
                }
            }
        }
        count
    }

    pub fn tick(&mut self) {
        for y in 1..self.rows - 4 {
            for x in 1..self.cols - 1 {
                match self.matrix[y][x] {
                    Cell::Alive => match self.alive_neighbours(x, y) {
                        0 | 1 => {
                            self.interim_matrix[y][x] = Cell::Dead;
                            let print_x = x * 3 + 2;
                            let print_y = y + 6;
                            print!("\x1B[{print_y};{print_x}H░░░");
                        }
                        2 | 3 => {
                            self.interim_matrix[y][x] = Cell::Alive;
                            let print_x = x * 3 + 2;
                            let print_y = y + 6;
                            print!("\x1B[{print_y};{print_x}H███");
                        }
                        _ => {
                            self.interim_matrix[y][x] = Cell::Dead;
                            let print_x = x * 3 + 2;
                            let print_y = y + 6;
                            print!("\x1B[{print_y};{print_x}H░░░");
                        }
                    },
                    Cell::Dead => match self.alive_neighbours(x, y) {
                        3 => {
                            self.interim_matrix[y][x] = Cell::Alive;
                            let print_x = x * 3 + 2;
                            let print_y = y + 6;
                            print!("\x1B[{print_y};{print_x}H███");
                        }
                        _ => (),
                    },
                }
            }
        }

        for y in 0..self.rows {
            for x in 0..self.cols {
                self.matrix[y][x] = self.interim_matrix[y][x];
            }
        }
    }
}
