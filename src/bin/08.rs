use std::collections::HashMap;

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

#[derive(Debug)]
enum Direction {
    UP(i32, i32),
    DOWN(i32, i32),
    LEFT(i32, i32),
    RIGHT(i32, i32),
}

fn walk_edges(grid: &Grid, visible_grid: &mut Grid, edges: Vec<(usize, usize)>) -> HashMap<(usize, usize), HashMap<String, u32>> {

    // We will have a hash map with each node as a key, and a counter for each direction
    let mut direction_counters : HashMap<(usize, usize), HashMap<String, u32>> = HashMap::new();

    for (x, y) in edges {
        let mut previous_node = (x, y);

        // We will check the direction of the edge and iterate over the nodes in that direction
        // If the edge X is 1 to len(x)-1, then we will iterate over the y values
        // If the edge Y is 1 to len(y)-1, then we will iterate over the x values
        let max_x = grid.width - 1;
        let max_y = grid.height - 1;
        let direction : Direction;
       
        if x == 0 {
            direction = Direction::DOWN(1, 0);
        } else if x == max_x {
            direction = Direction::UP(-1, 0);
        } else if y == 0 {
            direction = Direction::RIGHT(0, 1);
        } else if y == max_y {
            direction = Direction::LEFT(0, -1);
        } else {
            panic!("Invalid edge");
        }
        let mut current_node = (x, y);
        let mut traversed_trees_height : Vec<u32> = Vec::new();
        
        while is_valid_node(current_node, grid) {
            match direction {
                Direction::UP(x, y) => current_node.0 = (current_node.0 as i32 + x) as usize,
                Direction::DOWN(x, y) => current_node.0 = (current_node.0 as i32 + x) as usize,
                Direction::LEFT(x, y) => current_node.1 = (current_node.1 as i32 + y) as usize,
                Direction::RIGHT(x, y) => current_node.1 = (current_node.1 as i32 + y) as usize,
            }
            if !is_valid_node(current_node, grid) {
                break;
            }

            let curr_tree_height = grid.grid[current_node.0][current_node.1].to_digit(10).unwrap();
            let prev_tree_height = grid.grid[previous_node.0][previous_node.1].to_digit(10).unwrap();
            
            traversed_trees_height.push(prev_tree_height as u32);

            // Ensure that the current node is higher than all the previous nodes
            // If it is, then it is visible
            if traversed_trees_height.iter().all(|&x| x < curr_tree_height as u32) {
                visible_grid.grid[current_node.0][current_node.1] = '1';
            } 

            if !direction_counters.contains_key(&current_node) {
                direction_counters.insert(current_node, HashMap::new());
            }

            // We are going to iterate over all the previous trees in 
            // reverse direction until we find a tree that is higher than the current tree
            // If we find a tree that is higher, then we will break out of the loop
            // if we don't we increase the direction counter by 1
            for tree_height in traversed_trees_height.iter().rev() {
                if tree_height >= &curr_tree_height {
                    increase_direction_counters(&direction, &mut direction_counters, current_node);
                    break;
                } else {
                    // We will increase the counter in the direction of the edge
                    increase_direction_counters(&direction, &mut direction_counters, current_node);
                }
            }
            previous_node = current_node;
        }
    }
    direction_counters
}

fn increase_direction_counters(direction: &Direction, direction_counters: &mut HashMap<(usize, usize), HashMap<String, u32>>, current_node: (usize, usize)) {
    match *direction {
        Direction::UP(x, y) => {
            let counter = direction_counters.get_mut(&current_node).unwrap();
            let counter = counter.entry("DOWN".to_string()).or_insert(0);
            *counter += 1;
        },
        Direction::DOWN(x, y) => {
            let counter = direction_counters.get_mut(&current_node).unwrap();
            let counter = counter.entry("UP".to_string()).or_insert(0);
            *counter += 1;
        },
        Direction::LEFT(x, y) => {
            let counter = direction_counters.get_mut(&current_node).unwrap();
            let counter = counter.entry("RIGHT".to_string()).or_insert(0);
            *counter += 1;
        },
        Direction::RIGHT(x, y) => {
            let counter = direction_counters.get_mut(&current_node).unwrap();
            let counter = counter.entry("LEFT".to_string()).or_insert(0);
            *counter += 1;
        },
    }
}

fn is_valid_node(current_node: (usize, usize), grid: &Grid) -> bool {
    let (x, y) = current_node;
    x < grid.width && y < grid.height
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
    let grid = load_grid(input);
    let mut visible_grid = create_visible_grid(grid.width, grid.height);
    let edges = get_edges(&grid);
    let direction_counters = walk_edges(&grid, &mut visible_grid, edges);
    
    // We will multiply all the direction counters for each node against each other
    // to get the total scenic score
    let aggregated_scores_by_hashmap = direction_counters.iter().map(|(_, x)| x.iter().map(|(_, y)| y).product::<u32>());
    let max_hashmap_score = aggregated_scores_by_hashmap.max().unwrap();
    Some(max_hashmap_score)
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
