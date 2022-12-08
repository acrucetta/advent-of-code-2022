/*
Approach:
- Use a grid class to load the input
- Use a second grid with 0s and 1s for trees that are visible
- Load the coordinates of all the edges of the grid (top, bottom, left, right)
- For each edge, if the edge is horizontal, iterate over the y values
- For each edge, if the edge is vertical, iterate over the x values
- If the coordinates are > than the previous node, add a 1 to the visible grid


Methods
- load_grid from input
- create_visible_grid
- get_edges
- walk_edges
- get_visible_count from visible grid
*/
use advent_of_code::helpers::grid::{Grid, ALL_DIRECTIONS, CARDINAL_DIRECTIONS};

fn load_grid(input: &str) -> Grid {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    Grid {
        grid,
        width: input.lines().next().unwrap().len(),
        height: input.lines().count(),
    }
}

fn create_visible_grid(width: usize, height: usize) -> Grid {
    let mut grid = vec![vec!['0'; width]; height];
    // All the edges are visible so we will set them to 1
    for x in 0..width {
        grid[0][x] = '1';
        grid[height - 1][x] = '1';
    }
    for y in 0..height {
        grid[y][0] = '1';
        grid[y][width - 1] = '1';
    }
    Grid {
        grid,
        width,
        height,
    }
}

fn get_edges(grid: &Grid) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    for x in 0..grid.width {
        edges.push((x, 0));
        edges.push((x, grid.height - 1));
    }
    for y in 0..grid.height {
        edges.push((0, y));
        edges.push((grid.width - 1, y));
    }
    // We don't need to check the corners, because they will be checked by the edges
    // We will remove the corners from the edges
    let corners = vec![(0, 0), (0, grid.height - 1), (grid.width - 1, 0), (grid.width - 1, grid.height - 1)];
    edges.retain(|edge| !corners.contains(edge));
    edges
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn walk_edges(grid: &Grid, visible_grid: &mut Grid, edges: Vec<(usize, usize)>) {
    for (x, y) in edges {
        let mut previous_node = (x, y);

        // We will check the direction of the edge and iterate over the nodes in that direction
        // If the edge X is 1 to len(x)-1, then we will iterate over the y values
        // If the edge Y is 1 to len(y)-1, then we will iterate over the x values
        let direction;
        
        if (y > 0 && y < grid.height - 1) && (x == 0 || x == grid.width - 1) {
            let direction = "VERTICAL";
            
        } else if (x > 0 && x < grid.width - 1) && (y == 0 || y == grid.height - 1) {
            let direction = "HORIZONTAL";
        } else {
            panic!("Edge is not on the edge of the grid");
        }

        match direction {
            "HORIZONTAL" => {
                for dy in 1..grid.height - 1 {
                    let current_node_value = grid.get(x, dy);
                    let previous_node_value = grid.get(x, y);
                    if previous_node_value < current_node_value {
                        visible_grid.set(x, dy, '1');
                        previous_node = (x, dy);
                    }
                }
            }
            "VERTICAL" => {
                for x in 1..grid.width - 1 {
                    let current_node = (x, y);
                    if grid.get(current_node) == '#' {
                        visible_grid.set(current_node, '1');
                        previous_node = current_node;
                    }
                }
            }
            _ => {}
        }
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    let grid = load_grid(input);
    let mut visible_grid = create_visible_grid(grid.width, grid.height);
    let edges = get_edges(&grid);
    walk_edges(&grid, &mut visible_grid, edges);
    let visible_count = visible_grid.grid.iter().flatten().filter(|&x| *x == '1').count();
    Some(visible_count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
