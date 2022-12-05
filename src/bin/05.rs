/*
Problem:
- We have a series of crates, each one is placed on a stack with a number tag
- We have a series of instructions to move crates from one place ot the other
- We move one crate at a time, grabbing crates from the top of the stack and moving them to another stack

Approach
- We can use a struct to represen the each crate stack (CrateStack) it will have
    - a name (String)
    - a list of crates (Vec<i32>) (we can use a Vec because we will be adding and removing crates from the top of the stack)
- We need to parse the input into a list of instructions (Vec<Instruction>)
    - We can use a struct to represent each instruction (Instruction)
        - a source stack (String)
        - a destination stack (String)
        - a number of crates to move (i32)
- We can use a HashMap to represent the stacks (HashMap<String, CrateStack>)
- We can use a loop to iterate through the instructions
    - We can use a function to move the crates (move_crates)
        - We can use a loop to iterate through the crates
            - We can use a function to move a single crate (move_crate)
                - We can use a function to get the crate from the source stack (get_crate)
                - We can use a function to add the crate to the destination stack (add_crate)
*/

use std::collections::HashMap;

struct Instruction {
    source: i32,
    destination: i32,
    number_of_crates: i32,
}

struct Crates<'a> {
    stacks: HashMap<usize, Vec<&'a str>>,
}

impl Crates {
    fn move_crates(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.number_of_crates {
            let crate_to_move = self.get_crate(&instruction.source);
            self.add_crate(&instruction.destination, crate_to_move);
        }
    }

    fn move_crate(&mut self, source: &str, destination: &str) {
        let crate_to_move = self.get_crate(source);
        self.add_crate(destination, crate_to_move);
    }

    fn get_crate(&mut self, source: &str) -> i32 {
        let source_stack = self.stacks.get_mut(source).unwrap();
        source_stack.crates.pop().unwrap()
    }

    fn add_crate(&mut self, destination: &str, crate_to_move: i32) {
        let destination_stack = self.stacks.get_mut(destination).unwrap();
        destination_stack.crates.push(crate_to_move);
    }
}

fn parse_instruction(line: &str) -> Instruction {
    let mut parts = line.split_whitespace();

    // Parse each part and convert to i32 (without using unwrap)
    let number_of_crates : i32 = match parts.nth(1).expect("Number of Crates not Found").parse() {
        Ok(num) => num,
        Err(_) => panic!("Could not parse number of crates"),
    };
    let source : i32 = match parts.nth(3).expect("Source not Found").parse() {
        Ok(num) => num,
        Err(_) => panic!("Could not parse source"),
    };
    let destination : i32 = match parts.nth(5).expect("Destination not Found").parse() {
        Ok(num) => num,
        Err(_) => panic!("Could not parse destination"),
    };
    Instruction { source: source, destination: destination, number_of_crates: number_of_crates }
}

fn parse_stack_input(input: &str) -> Crates {
    let mut stacks: HashMap<usize, Vec<&str>> = HashMap::new();
    let mut stack_line_indices : HashMap<usize,usize> = HashMap::new();

    for (y, line) in input.lines().rev().enumerate() {

        for (x, char) in line.chars().enumerate() {
            if char.is_numeric() {
                let index = char.to_digit(10).unwrap() as usize;
                stacks.insert(index, Vec::new());
                stack_line_indices.insert(index, y as usize);
            }
            if char.is_alphabetic() {
                let index: usize = stack_line_indices.get(&x).unwrap().to_owned();
                let stack = match stacks.get_mut(&index) {
                    Some(stack) => stack,
                    None => panic!("Stack not found"),
                };
                stack.push(&char.to_string());
            }
        }
    }
    Crates { stacks}
}

fn parse_input(input: &str) -> ( Crates, Vec<Instruction>) {
    // We know there's a blank space between the instructions
    // and the stacks, so we can split the input into two parts
    let parts : Vec<&str> = input.split("\n").collect();
    let stacks_input = parse_stack_input(input);
    let instructions = parts[1].lines().map(|line| parse_instruction(line)).collect();

    (stacks_input, instructions)
}

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
