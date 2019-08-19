pub trait Grid {
    fn new(width: usize, height: usize) -> Self;

    fn width(&self) -> usize;

    fn height(&self) -> usize;

    fn at(&self, x: isize, y: isize) -> Option<bool>;

    fn set(&mut self, x: isize, y: isize, val: bool);

    fn count_live_neighbors(&self, x: isize, y: isize) -> usize;
}

pub trait LifeBoard {
    type Grid: Grid;

    fn new(initial: Self::Grid) -> Self;

    fn tick(&mut self) -> &Self::Grid;
}
