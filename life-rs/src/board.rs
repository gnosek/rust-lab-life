use crate::traits::{Grid, LifeBoard};
use itertools::Itertools;

pub struct NativeGrid(Vec<Vec<bool>>);

impl NativeGrid {
    fn get_neighbors(&self, x: isize, y: isize) -> [(isize, isize); 8] {
        [
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
        ]
    }

    fn iter_coords(&self) -> impl Iterator<Item = (isize, isize)> {
        (0..self.width() as isize).cartesian_product(0..self.height() as isize)
    }
}

impl Grid for NativeGrid {
    fn new(width: usize, height: usize) -> NativeGrid {
        NativeGrid(vec![vec![false; width]; height])
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn at(&self, x: isize, y: isize) -> Option<bool> {
        self.0
            .get(y as usize)
            .and_then(|row| row.get(x as usize))
            .map(|val| *val)
    }

    fn set(&mut self, x: isize, y: isize, val: bool) {
        self.0
            .get_mut(y as usize)
            .and_then(|row| row.get_mut(x as usize))
            .map(|cell| *cell = val);
    }

    fn count_live_neighbors(&self, x: isize, y: isize) -> usize {
        self.get_neighbors(x, y)
            .iter()
            .map(|(x, y)| self.at(*x, *y).unwrap_or_default() as usize)
            .sum()
    }
}

pub struct NativeLifeBoard {
    current: NativeGrid,
    next: NativeGrid,
}

impl NativeLifeBoard {
    fn will_live(is_live: bool, num_live_neighbors: usize) -> bool {
        if is_live {
            num_live_neighbors == 2 || num_live_neighbors == 3
        } else {
            num_live_neighbors == 3
        }
    }

    fn update(&mut self) {
        for (x, y) in self.current.iter_coords() {
            let is_live = self.current.at(x, y).unwrap_or_default();
            let num_live_neighbors = self.current.count_live_neighbors(x, y);
            self.next
                .set(x, y, Self::will_live(is_live, num_live_neighbors));
        }
    }
}

impl LifeBoard for NativeLifeBoard {
    type Grid = NativeGrid;

    fn new(initial: NativeGrid) -> Self {
        let width = initial.width();
        let height = initial.height();

        NativeLifeBoard {
            current: initial,
            next: NativeGrid::new(width, height),
        }
    }

    fn tick(&mut self) -> &NativeGrid {
        println!("Rust board tick");
        self.update();
        std::mem::swap(&mut self.current, &mut self.next);
        &self.current
    }
}
