use crate::traits::{Grid, LifeBoard};
use std::os::raw::{c_int, c_uint};

#[allow(dead_code)]
mod ffi {
    use std::os::raw::{c_int, c_uint};

    #[repr(C)]
    pub enum CellValue {
        Dead = 0,
        Alive = 1,
        OutOfBounds = 2,
    }

    #[repr(C)]
    pub struct Grid {
        _private: [u8; 0],
    }

    #[repr(C)]
    pub struct LifeBoard {
        _private: [u8; 0],
    }

    extern "C" {
        pub fn grid_new(width: c_uint, height: c_uint) -> *mut Grid;
        pub fn grid_delete(grid: *mut Grid);
        pub fn grid_width(grid: *const Grid) -> c_uint;
        pub fn grid_height(grid: *const Grid) -> c_uint;
        pub fn grid_at(grid: *const Grid, x: c_int, y: c_int) -> CellValue;
        pub fn grid_set(grid: *mut Grid, x: c_int, y: c_int, val: bool);
        pub fn grid_count_live_neighbors(grid: *const Grid, x: c_int, y: c_int) -> c_uint;
        pub fn board_new(initial_grid: *mut Grid) -> *mut LifeBoard;
        pub fn board_delete(board: *mut LifeBoard);
        pub fn board_tick(board: *mut LifeBoard) -> *const Grid;
    }
}

pub struct FFIGrid(*const ffi::Grid);
pub struct FFILifeBoard(*mut ffi::LifeBoard, Option<FFIGrid>);

impl Grid for FFIGrid {
    fn new(width: usize, height: usize) -> FFIGrid {
        let grid = unsafe { ffi::grid_new(width as c_uint, height as c_uint) };
        FFIGrid(grid)
    }

    fn width(&self) -> usize {
        unsafe { ffi::grid_width(self.0) as usize }
    }

    fn height(&self) -> usize {
        unsafe { ffi::grid_height(self.0) as usize }
    }

    fn at(&self, x: isize, y: isize) -> Option<bool> {
        let val = unsafe { ffi::grid_at(self.0, x as c_int, y as c_int) };
        match val {
            ffi::CellValue::Alive => Some(true),
            ffi::CellValue::Dead => Some(false),
            ffi::CellValue::OutOfBounds => None,
        }
    }

    fn set(&mut self, x: isize, y: isize, val: bool) {
        unsafe { ffi::grid_set(self.0 as *mut _, x as c_int, y as c_int, val) }
    }

    fn count_live_neighbors(&self, x: isize, y: isize) -> usize {
        unsafe { ffi::grid_count_live_neighbors(self.0, x as c_int, y as c_int) as usize }
    }
}

impl LifeBoard for FFILifeBoard {
    type Grid = FFIGrid;

    fn new(initial: FFIGrid) -> FFILifeBoard {
        let board = unsafe { ffi::board_new(initial.0 as *mut _) };
        FFILifeBoard(board, None)
    }

    fn tick(&mut self) -> &FFIGrid {
        let new_grid = unsafe { ffi::board_tick(self.0) };
        self.1 = Some(FFIGrid(new_grid));
        self.1.as_ref().unwrap()
    }
}
