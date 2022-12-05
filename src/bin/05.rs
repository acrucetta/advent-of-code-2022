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

struct CrateStack {
    crates: Vec<i32>,
}

struct Instruction {
    source: i32,
    destination: i32,
    number_of_crates: i32,
}

struct Crates {
    stacks: HashMap<i32, CrateStack>,
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

fn parse_input(input: &str) -> (Crates, Vec<Instruction>) {
    let mut stacks = HashMap::new();
    let mut instructions = Vec::new();

    for line in input.lines() {
        if line.contains("move") {
            let instruction = parse_instruction(line);
            instructions.push(instruction);
        } else {
            let stack = parse_stack(line);
            stacks.insert(stack.name.clone(), stack);
        }
    }

    (Crates { stacks }, instructions)
}

fn parse_instruction(line: &str) -> Instruction {
    let mut parts = line.split_whitespace();
    let number_of_crates : i32 = parts.nth(1)
    let source : i32 = parts.nth(3);
    let destination: i32 = parts.nth(5);
    Instruction { source: source, destination: destination, number_of_crates: number_of_crates }
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
