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
use advent_of_code::helpers::grid::{Grid, ALL_DIRECTIONS, CARDINAL_DIRECTIONS, self};

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

struct TreeScenic {
    up: u32,
    down: u32,
    left: u32,
    right: u32,
}

fn walk_edges(grid: &Grid, visible_grid: &mut Grid, edges: Vec<(usize, usize)>) -> HashMap<(usize, usize), TreeScenic> {
    // We are going to keep track of each node
    // and how many trees are visible from that node 
    // We will use a hashmap to keep track of the nodes
    // and a vec to grab the value of the visible trees
    // from that node
    let mut scenic_trees : HashMap<(usize, usize), TreeScenic> = HashMap::new();

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

            // Adding to the hashmap
            if !scenic_trees.contains_key(&current_node) {
                scenic_trees.insert(current_node, TreeScenic {up: 0, down: 0, left: 0, right: 0});
            }
            
            traversed_trees_height.push(prev_tree_height);

            // Ensure that the current node is higher than all the previous nodes
            // If it is, then it is visible
            if traversed_trees_height.iter().all(|&x| x < curr_tree_height) {
                visible_grid.grid[current_node.0][current_node.1] = '1';
            } 
            
            // If the current node is higher or equal to the previous node
            // then we will add the previous node to the hashmap
            if curr_tree_height >= prev_tree_height {
                let mut tree_scenic = scenic_trees.get_mut(&current_node).unwrap();
                match direction {
                    Direction::UP(x, y) => tree_scenic.up += 1,
                    Direction::DOWN(x, y) => tree_scenic.down += 1,
                    Direction::LEFT(x, y) => tree_scenic.left += 1,
                    Direction::RIGHT(x, y) => tree_scenic.right += 1,
                }
            }
            previous_node = current_node;
        }
    }
    scenic_trees
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
    let visible_nodes = walk_edges(&grid, &mut visible_grid, edges);
    
    // Multiply each vector in the hashmap, and find the max
    let tree_counts : Vec<u32> = visible_nodes.values().map(|x| x.up * x.down * x.left * x.right).collect();
    let max = tree_counts.iter().max().unwrap();
    Some(*max)
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
