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
use regex::Regex;

struct Instruction {
    source: usize,
    destination: usize,
    number_of_crates: usize,
}

struct Crates {
    stacks: HashMap<usize, Vec<String>>,
}


impl Crates {
    fn move_crates(&mut self, instruction: &Instruction) {
        let num_crates_to_move = instruction.number_of_crates;
        for _ in 0..num_crates_to_move {
            self.move_crate(&instruction.source, &instruction.destination);
        }
        }

    fn move_crate(&mut self, source: &usize, destination: &usize) {
        let crate_to_move = self.stacks.get_mut(&source).unwrap().pop().unwrap();
        self.stacks.get_mut(&destination).unwrap().push(crate_to_move);
    }

    fn move_crates_in_order(&mut self, instruction: Instruction) {
        // Instead of moving crates popping and pushing them, we can move them in order
        // we grab the last N crates from the source stack and add them to the destination stack
        let num_crates_to_move = instruction.number_of_crates;
        let source = instruction.source;
        let destination = instruction.destination;
        let stack = self.stacks.get_mut(&source).unwrap();
        // We can use the drain method to get the last N crates from the source stack
        // We can use the extend method to add the crates to the destination stack
        let crates_to_move : Vec<String> = stack.drain(stack.len() - num_crates_to_move..).collect();
        self.stacks.get_mut(&destination).unwrap().extend(crates_to_move);

    }
}


fn parse_instructions(line: &str) -> Vec<Instruction> {

    // Parse lines with the following format:
    // "move 1 from 1 to 2"
    // using regex
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let mut instructions = Vec::new();
    
    for line in line.lines() {
        if line.is_empty() {
            continue;
        }
        let captures = re.captures(line).unwrap();
        let number_of_crates = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let source = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let destination = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();

        instructions.push(Instruction { source: source, destination: destination, number_of_crates: number_of_crates });
    }
    instructions
}


fn parse_stack_input(input: &str) -> Crates {
    let mut stacks: HashMap<usize, Vec<String>> = HashMap::new();
    let mut stack_line_indices : HashMap<usize,usize> = HashMap::new();

    for (y, line) in input.lines().rev().enumerate() {

        for (x, char) in line.chars().enumerate() {

            if char.is_numeric() {
                let index = char.to_digit(10).unwrap() as usize;
                stacks.insert(index, Vec::new());
                stack_line_indices.insert(x, index);
            }

            if char.is_alphabetic() {
                let index: usize = *stack_line_indices.get(&x).unwrap();
                let stack = stacks.get_mut(&index).unwrap();
                stack.push(String::from(&char.to_string()));
                
            }
        }
    }
    Crates { stacks: stacks }
}


fn parse_input(input: &str) -> (Crates, Vec<Instruction>) {
    // Split the input into two parts, there's a blank line between the stacks and the instructions
    // We will recognize the empty whitespace as the delimiter using regex \s+
    let mut stack_input = "".to_string();
    let mut instructions_input = "".to_string();
    let mut found_stack_input = false;

    for line in input.lines() {
        // We will add the line to the stack_input until we find the empty line
        // Then we will add the rest of the lines to the instructions_input
        if line.is_empty() {
            found_stack_input = true;
        }
        if !found_stack_input {
            stack_input.push_str(line);
            stack_input.push_str("\n");
        } else {
            instructions_input.push_str(line);
            instructions_input.push_str("\n");
        }
    }
    let stacks_input = parse_stack_input(&stack_input);
    let instructions = parse_instructions(&instructions_input);

    (stacks_input, instructions)
}


pub fn part_one(input: &str) -> Option<String> {
    let mut top_stack_chars : Vec<String> = Vec::new();
    let (mut stacks, instructions) = parse_input(input);

    for instruction in instructions {
        stacks.move_crates(&instruction);
    }
    for key in 1..stacks.stacks.len()+1 {
        let stack = stacks.stacks.get(&key).unwrap();
        let top_char = stack.last().unwrap();
        top_stack_chars.push(top_char.to_string());
    }
    
    Some(top_stack_chars.join(""))
}

pub fn part_two(input: &str) -> Option<String> {
    let mut top_stack_chars : Vec<String> = Vec::new();
    let (mut stacks, instructions) = parse_input(input);

    for instruction in instructions {
        stacks.move_crates_in_order(instruction);
    }

    for key in 1..stacks.stacks.len()+1 {
        let stack = stacks.stacks.get(&key).unwrap();
        let top_char = stack.last().unwrap();
        top_stack_chars.push(top_char.to_string());
    }
    
    Some(top_stack_chars.join(""))
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
