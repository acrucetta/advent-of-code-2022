/*
Approach
- Parse Instructions into Direction and Distance
- Create grid with the max distance in each direction
- Create a grid of booleans to track visited cells for TAILS
- Create a method to move the tail and head
- Create a method to track the head with respect to the tail
    - If the tail is not within a single cell of the head, move the tail
 */

use std::{collections::HashMap, hash::Hash, cmp::max};

struct Direction {
    direction: String,
    distance: u32,
}

fn parse_directions(input: &str) -> Vec<Direction> {
    input.lines().map(|line| {
        let parsed_line: Vec<&str> = line.split_whitespace().collect();
        Direction {
            direction: parsed_line[0].to_string(),
            distance: parsed_line[1].parse().unwrap(),
        }
    }).collect()
}

fn create_grid(max_directions : HashMap<String, u32>) -> Vec<Vec<bool>> {
    let max_width = max(max_directions.get("R").unwrap(), max_directions.get("L").unwrap());
    let max_height = max(max_directions.get("U").unwrap(), max_directions.get("D").unwrap());
    let mut grid = vec![vec![false; *max_width as usize]; *max_height as usize];
    grid
}

fn get_max_directions(directions : Vec<Direction>) -> HashMap<String, u32> {
    let mut max_directions = HashMap::new();
    max_directions.insert("R".to_string(), 0);
    max_directions.insert("L".to_string(), 0);
    max_directions.insert("U".to_string(), 0);
    max_directions.insert("D".to_string(), 0);
    for direction in directions {
        let current_value = max_directions.get(&direction.direction).unwrap();
        max_directions.insert(direction.direction, current_value + direction.distance);
    }
    max_directions
}

/*
Move Head (Grid, Curr Location, Direction)
- Move step by step
- Every time we move the head we will move the tail
    - If the X coordinate is the same, we move the Y only (right or left)
        (0,+1/-1)
    - If the Y coordinate is the same, we move the X only (up or down)
        (+1/-1,0)
    - If the X and Y is different, we move diagonally
        - E.g., (3,1) tails, (2,3) heads
        - Difference is (-1,2)
        - We need to close the difference to 1. We increase Y by 1 and X by 1
        - (+1/-1,+1/-1)

Update Visited Grid
- For every position the tail has been in, we have a visited grid
*/

pub fn part_one(input: &str) -> Option<u32> {
    let movements = parse_directions(input);
    let grid = create_grid(get_max_directions(movements));
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
