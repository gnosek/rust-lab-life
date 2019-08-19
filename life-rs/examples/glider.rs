use life_rs::display::show_grid;
use life_rs::traits::{Grid, LifeBoard};
use std::thread::sleep;

#[cfg(not(feature = "ffi_import"))]
mod board {
    pub use life_rs::board::{NativeGrid as Grid, NativeLifeBoard as LifeBoard};
}

#[cfg(feature = "ffi_import")]
mod board {
    pub use life_rs::ffi_import::{FFIGrid as Grid, FFILifeBoard as LifeBoard};
}

fn glider(width: usize, height: usize) -> board::Grid {
    let mut grid = board::Grid::new(width, height);

    grid.set(2, 1, true);
    grid.set(3, 2, true);
    grid.set(1, 3, true);
    grid.set(2, 3, true);
    grid.set(3, 3, true);

    grid
}

fn main() {
    let grid = glider(20, 20);
    show_grid(&grid);
    let mut board = board::LifeBoard::new(grid);
    loop {
        sleep(std::time::Duration::from_millis(500));
        let grid = board.tick();
        show_grid(grid);
    }
}
