/*
Approach
- We need to find for each pair of numbers in the input, what are the numbers included in the range
- Then we will compare them with the inputs in the other pairs split by a comma
- We want to figure out which pairs fully include the other pairs
E.g. 2-4 is fully included by 1-5 or by 2-4.
We will count those pairs and return the count

Methods
- We need to use one method to split the input by commas into a vector of strings
- Then for each pair in that list, we need to create a range of numbers 
that includes the start and end of the range.
- We need to compare if the start and end is contained by the other ranges
If so we will count those.

E.g., 2-4, 1-5

In this example, we will store the ranges in a vector of tuples
[(2,4), (1,5)]

We will then compare the first range with the second range
If the start of the first range is greater than or equal to the start of the second range
we will check if the end of the first range is less than or equal to the end of the second range

If both conditions are true, we will count the pair
*/

use std::collections::HashSet;

struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn contains (&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlap (&self, other: &Range) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

fn string_into_range(input: &str) -> Range {
    let mut split = input.split('-');
    let start = split.next().unwrap().parse::<i32>().unwrap();
    let end = split.next().unwrap().parse::<i32>().unwrap();
    Range { start, end }
}

pub fn part_one(input: &str) -> Option<u32> {
    let ranges: Vec<Vec<Range>> = input
        .lines()
        .map(|line| line.split(',')) 
        .map(|split| split.map(|range| string_into_range(range)).collect())
        .collect();
    
   for vector in ranges.iter() {
       for range in vector.iter() {
        for other in vector.iter() {
            if range.contains(other) {
                println!("{} contains {}", range.start, other.start);
            }
        }
       }
   }
    None
}


pub fn part_two(input: &str) -> Option<u32> {
    let mut overlap_count = 0;
    for line in input.lines() {
        let assignment_ranges = line.split(",").collect::<Vec<_>>();
        
        let mut overlapping_shifts_indices = HashSet::new();

        for (i, range) in assignment_ranges.iter().enumerate() {
            let range_iter = string_into_range(range);
            
            for (j, range2) in assignment_ranges.iter().enumerate() {
                if i == j {
                    continue;
                }
                let range2_iter = string_into_range(range2);
                if range_iter.start <= range2_iter.end && range_iter.end >= range2_iter.start {
                    let mut indices = vec![i, j];
                    indices.sort();
                    if !overlapping_shifts_indices.contains(&indices) {
                        overlapping_shifts_indices.insert(indices);
                        overlap_count += 1;
                    }
                }
            }
        }
    }
    Some(overlap_count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
