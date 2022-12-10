use std::{cmp::max, collections::{HashMap, HashSet}, hash::Hash};

struct Direction {
    direction: String,
    distance: i32,
}

fn parse_directions(input: &str) -> Vec<Direction> {
    input
        .lines()
        .map(|line| {
            let parsed_line: Vec<&str> = line.split_whitespace().collect();
            Direction {
                direction: parsed_line[0].to_string(),
                distance: parsed_line[1].parse().unwrap(),
            }
        })
        .collect()
}



fn walk_through_grid(movements : Vec<Direction>) -> HashSet<(i32, i32)> {
    // This function will move the head and tail
    // We will also update the visited grid for every position the tail has been in
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);
    let mut tail_visits : HashSet<(i32, i32)> = HashSet::new();
    tail_visits.insert(tail);
    // We will move the head and tail by 1 step
    // We will also update the visited grid for every position the tail has been in
    // We will stop when we've finished all the movements
    for mut movement in movements {
        // Move Head
        while movement.distance > 0 {
            
            if movement.direction == "R" {
                head.1 += 1;
            } else if movement.direction == "L" {
                head.1 -= 1;
            } else if movement.direction == "U" {
                head.0 += 1;
            } else if movement.direction == "D" {
                head.0 -= 1;
            }

            // Reduce the distance by 1
            movement.distance -= 1;
            
            // Move Tail
            let (x_tail, y_tail) = tail;
            let (x_diff, y_diff) = (head.0 - x_tail, head.1 - y_tail);

            // If the tail is within 1 unit of the head, we skip
            if x_diff.abs() <= 1 && y_diff.abs() <= 1 {
                continue;
            }
            
            if x_diff == 0 {
                // We move the Y only (right or left)
                // (0,+1/-1)
                if y_diff > 0 {
                    // We move the tail right
                    tail.1 += 1;
                } else {
                    // We move the tail left
                    tail.1 -= 1;
                }
            } else if y_diff == 0 {
                // We move the X only (up or down)
                // (+1/-1,0)
                if x_diff > 0 {
                    // We move the tail up
                    tail.0 += 1;
                } else {
                    // We move the tail down
                    tail.0 -= 1;
                }
            } else {
                if x_diff > 0 {
                    // Up
                    tail.0 += 1;
                } else {
                    // Down
                    tail.0 -= 1;
                }
                if y_diff > 0 {
                    // Right
                    tail.1 += 1;
                } else {
                    // Left
                    tail.1 -= 1;
                }
            }
            // Update tail visits
            tail_visits.insert(tail);
        }
    }
    tail_visits
}

pub fn part_one(input: &str) -> Option<u32> {
    let movements = parse_directions(input);
    let tail_visits = walk_through_grid(movements);
    let total_tail_visits = tail_visits.len();
    Some(total_tail_visits as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let movements = parse_directions(input);
    Some(1)
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
