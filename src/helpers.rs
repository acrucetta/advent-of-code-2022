/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

pub mod grid {
    use std::fmt;
    pub const CARDINAL_DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    pub const ALL_DIRECTIONS: [(i32, i32); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    pub struct Grid {
        // A class used to read a grid and manipulate its positions
        // (x,y) where the top-left position is (0,0)
        pub grid: Vec<Vec<char>>,
        pub width: usize,
        pub height: usize,
    }

    impl fmt::Display for Grid {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let rows = self
                .grid
                .iter()
                .map(|row| row.iter().collect::<String>())
                .collect::<Vec<String>>();
            write!(f, "{}", rows.join("\n"))
        }
    }

    impl Grid {
        pub fn get(&self, x: usize, y: usize) -> Option<char> {
            if x >= self.width || y >= self.height {
                return None;
            }
            Some(self.grid[y][x])
        }

        pub fn set(&mut self, x: usize, y: usize, value: char) {
            self.grid[y][x] = value;
        }

        pub fn get_adjacent_positions(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
            let mut positions = Vec::new();
            for (dx, dy) in CARDINAL_DIRECTIONS.iter() {
                let new_x = (x as i32 + dx) as usize;
                let new_y = (y as i32 + dy) as usize;
                if let Some(_) = self.get(new_x, new_y) {
                    positions.push((new_x, new_y));
                }
            }
            positions
        }

        pub fn get_all_positions(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
            let mut positions = Vec::new();
            for (dx, dy) in CARDINAL_DIRECTIONS.iter() {
                let new_x = (x as i32 + dx) as usize;
                let new_y = (y as i32 + dy) as usize;
                if let Some(_) = self.get(new_x, new_y) {
                    positions.push((new_x, new_y));
                }
            }
            positions
        }

        pub fn iterate_through_grid(&self) -> Vec<(usize, usize)> {
            let mut positions = Vec::new();
            for y in 0..self.height {
                for x in 0..self.width {
                    positions.push((x, y));
                }
            }
            positions
        }
    }
}
