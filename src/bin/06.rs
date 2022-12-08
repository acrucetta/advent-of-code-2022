/*
Approach:
- The input comes in the shape of a list of characters
- We want to keep traversing the list as long as we can't find
4 last characters that are all different
- We will do this by taking a set of the last 4 characters and
checking if the set has 4 elements
- If it does, we will return the index of the first character
- If it doesn't, we will keep going
*/

use std::collections::{HashSet, HashMap};


pub fn part_one(input: &str) -> Option<u32> {
    let mut index = 0;

    loop {
        let last_four = input.chars().skip(index).take(4).collect::<Vec<char>>();
        let set : HashSet<char> = last_four.iter().cloned().collect();

        if set.len() == 4 {
            // We return the index of the last character
            return Some(index as u32 + 4);
        }

        index += 1;
    }
}

pub fn part_two(message: &str) -> Option<usize> {
    let marker_size = 14;
    for i in 0..(message.len() - marker_size) {
        let cmd: String = message[i..i+marker_size].chars().collect();
        if cmd.len() == marker_size {
            return Some(i + marker_size);
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
