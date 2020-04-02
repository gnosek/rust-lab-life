use crate::board::NativeGrid;
use crate::board::NativeLifeBoard;
use crate::traits::Grid;
use crate::traits::LifeBoard;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmLife(NativeLifeBoard);

#[wasm_bindgen]
impl WasmLife {
    pub fn new(width: usize, height: usize) -> Self {
        console_error_panic_hook::set_once();
        let mut grid = NativeGrid::new(width, height);
        grid.set(2, 1, true);
        grid.set(3, 2, true);
        grid.set(1, 3, true);
        grid.set(2, 3, true);
        grid.set(3, 3, true);

        Self(NativeLifeBoard::new(grid))
    }

    pub fn tick(&mut self) -> String {
        let grid = self.0.tick();
        let width = grid.width();
        let height = grid.height();
        let mut board_data = String::with_capacity((width + 1) * height);

        for y in 0..height {
            for x in 0..width {
                let cell = grid.at(x as isize, y as isize).unwrap_or_default();
                let cell_char = if cell { '#' } else { '.' };
                board_data.push(cell_char);
            }
            board_data.push('\n');
        }

        board_data
    }
}
