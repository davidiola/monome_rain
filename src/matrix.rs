use rand::prelude::*;
use rand::distributions::{WeightedIndex, Distribution};

pub struct Matrix {
    grid: [bool; 64],
    positions: [usize; 8],
    lengths: [usize; 8],
    periods: [usize; 8],
    length_dist: WeightedIndex<u32>,
    rng: ThreadRng,
}

impl Matrix {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        // make mid-size raindrops the most likely
        let length_dist = WeightedIndex::new(&[1, 3, 4, 3, 2, 1]).unwrap();

        let mut positions = [0; 8];
        let mut lengths = [1; 8];
        // random update freq / raindrop progression speed
        let mut periods = [1; 8];

        for i in 0..8 {
            positions[i] = rng.gen_range(0..8);
            lengths[i] = length_dist.sample(&mut rng) + 1;
            periods[i] = rng.gen_range(1..4);
        }

        Matrix {
            grid: [false; 64],
            positions,
            lengths,
            periods,
            length_dist,
            rng,
        }
    }

    pub fn tick(&mut self, tick_count: usize) {
        for col in 0..8 {
            if tick_count % self.periods[col] == 0 {
                for row in 0..8 {
                    self.grid[row * 8 + col] = false;
                }

                self.positions[col] = (self.positions[col] + 1) % 8;

                if self.positions[col] == 0 {
                    self.lengths[col] = self.length_dist.sample(&mut self.rng) + 1;
                    self.periods[col] = self.rng.gen_range(1..4);
                }

                let length = self.lengths[col];
                for i in 0..length {
                    let row = (self.positions[col] + i) % 8;
                    self.grid[row * 8 + col] = true;
                }
            }
        }
    }

    pub fn start_raindrop(&mut self, col: usize, row: usize) {
        self.positions[col] = row;
        self.lengths[col] = self.length_dist.sample(&mut self.rng) + 1;
        self.periods[col] = self.rng.gen_range(1..4);

        for r in 0..8 {
            self.grid[r * 8 + col] = false;
        }

        let length = self.lengths[col];
        for i in 0..length {
            let current_row = (row + i) % 8;
            self.grid[current_row * 8 + col] = true;
        }
    }

    pub fn get_grid(&self) -> &[bool; 64] {
        &self.grid
    }
}