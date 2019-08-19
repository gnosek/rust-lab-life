use crate::board::{NativeGrid, NativeLifeBoard};
use crate::traits::{Grid, LifeBoard};
use std::os::raw::{c_int, c_uint};

#[no_mangle]
pub extern "C" fn grid_new(width: c_uint, height: c_uint) -> *mut NativeGrid {
    let grid = Box::new(NativeGrid::new(width as usize, height as usize));

    Box::into_raw(grid)
}

#[no_mangle]
pub extern "C" fn grid_delete(grid: *mut NativeGrid) {
    unsafe {
        Box::from_raw(grid);
    }
}

#[no_mangle]
pub extern "C" fn grid_width(grid: *const NativeGrid) -> c_uint {
    let grid = unsafe { &*grid };
    grid.width() as c_uint
}

#[no_mangle]
pub extern "C" fn grid_height(grid: *const NativeGrid) -> c_uint {
    let grid = unsafe { &*grid };
    grid.height() as c_uint
}

#[repr(C)]
pub enum CellValue {
    Dead = 0,
    Alive = 1,
    OutOfBounds = 2,
}

#[no_mangle]
pub extern "C" fn grid_at(grid: *const NativeGrid, x: c_int, y: c_int) -> CellValue {
    let grid = unsafe { &*grid };
    match grid.at(x as isize, y as isize) {
        Some(true) => CellValue::Alive,
        Some(false) => CellValue::Dead,
        None => CellValue::OutOfBounds,
    }
}

#[no_mangle]
pub extern "C" fn grid_set(grid: *mut NativeGrid, x: c_int, y: c_int, val: bool) {
    let grid = unsafe { &mut *grid };
    grid.set(x as isize, y as isize, val)
}

#[no_mangle]
pub extern "C" fn grid_count_live_neighbors(grid: *const NativeGrid, x: c_int, y: c_int) -> c_uint {
    let grid = unsafe { &*grid };
    grid.count_live_neighbors(x as isize, y as isize) as c_uint
}

#[no_mangle]
pub extern "C" fn board_new(initial_grid: *mut NativeGrid) -> *mut NativeLifeBoard {
    let grid = unsafe { Box::from_raw(initial_grid) };
    let board = Box::new(NativeLifeBoard::new(*grid));

    Box::into_raw(board)
}

#[no_mangle]
pub extern "C" fn board_delete(board: *mut NativeLifeBoard) {
    unsafe {
        Box::from_raw(board);
    }
}

#[no_mangle]
pub extern "C" fn board_tick(board: *mut NativeLifeBoard) -> *const NativeGrid {
    let board = unsafe { &mut *board };
    let grid = board.tick();

    grid as *const _
}
