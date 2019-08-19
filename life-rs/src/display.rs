use crate::traits::Grid;

pub fn show_grid<G: Grid>(grid: &G) {
//    print!("{0}[H{0}[J", 27 as char); // ANSI clear screen
    for _ in 0..grid.width() {
        print!("-");
    }
    println!();
    for row in 0..grid.height() {
        for col in 0..grid.width() {
            if grid.at(row as isize, col as isize).unwrap_or_default() {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }
}
