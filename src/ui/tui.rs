use crate::ui::grid::Grid;

pub struct TUI {
    grid: Grid,
    height: usize,
    width: usize,
    elapsed_ticks: usize,
}

impl TUI {
    pub fn new() -> Self {
        let (width, height) = match term_size::dimensions() {
            Some(dimensions) => dimensions,
            None => panic!("failed to get terminal dimensions"),
        };

        TUI {
            grid: Grid::new(height - 4, width / 3 - 2),
            height,
            width,
            elapsed_ticks: 0,
        }
    }

    pub fn init_draw(&self) -> () {
        self.grid.init_draw();
    }

    pub fn tick(&mut self) -> () {
        self.grid.tick();
        println!();
        self.elapsed_ticks += 1;
    }

    pub fn awake_many(&mut self, points: Vec<(usize, usize)>) {
        self.grid.awake_many(points);
    }
}
