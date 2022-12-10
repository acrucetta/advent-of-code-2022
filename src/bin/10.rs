/*

Approach:
- Create buffers in a list for every addx
- Create indices in the list for every operation
- Create a list of operations (parse the input)
- Track the value of the register X
*/

use std::vec;

use itertools::Itertools;

pub enum Operation {
    addx,
    noop,
    buffer,
}

pub struct Instruction {
    operation: Operation,
    value: i32,
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace().collect_vec();

        let operation = match parts[0] {
            "addx" => Operation::addx,
            "noop" => Operation::noop,
            _ => panic!("Unknown operation"),
        };

        let value = match operation {
            Operation::addx => parts[1].parse::<i32>().unwrap(),
            Operation::noop => 0,
            _ => panic!("Unknown operation"),
        };

        instructions.push(Instruction { operation, value });

        if parts[0] == "addx" {
            instructions.push(Instruction {
                operation: Operation::buffer,
                value: 0,
            })
        }
    }
    instructions
}

pub fn part_one(input: &str) -> Option<u32> {
    let instructions = parse_input(input);
    let cycles_to_check : Vec<u32> = vec![20,60,100,140,180,220];
    let mut register_addx_log : Vec<i32> = vec![0;300];

    for (i, instruction) in instructions.iter().enumerate() {
        match instruction.operation {
            Operation::addx => {
                register_addx_log[i+2] = instruction.value;
            }
            Operation::noop => {}
            Operation::buffer => {}
        }
    }
    let mut signal_strengths : i32 = 0;
    // Make a drawing of the register, it will be 6 rows of 40 chars
    let crt_drawing : Vec<Vec<char>> = vec![vec![' ';40];6];
    
    for cycle in cycles_to_check {
        let mut register = 1;
        for i in 0..cycle {
            register += register_addx_log[i as usize];
        }
        signal_strengths += (register * cycle as i32);
    }
    Some(signal_strengths as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let instructions = parse_input(input);
    let mut register_addx_log : Vec<i32> = vec![0;242];

    for (i, instruction) in instructions.iter().enumerate() {
        match instruction.operation {
            Operation::addx => {
                register_addx_log[i+2] = instruction.value;
            }
            Operation::noop => {}
            Operation::buffer => {}
        }
    }

    // Make a drawing of the register, it will be 6 rows of 40 chars
    let mut crt_drawing : Vec<Vec<char>> = vec![vec![]];
    let mut curr_row = 0;
    
    for i in 0..240 {
        let mut register = 1;
        
        if i % 40 == 0 {
            // Add a new row to the drawing
            crt_drawing.push(vec!['.';1]);
        }
        
        for j in 0..i {
            register += register_addx_log[j as usize];
        }

        // Check if the pixel being drawn (current i) overlaps within 1
        // unit of the register, if it does, draw a #, otherwise a .
        // -1, 0, 1 
        let register_range = (register-1)..(register+1);
        
        if register_range.contains(&(i as i32)) {
            crt_drawing[curr_row].push('#');
        } else {
            crt_drawing[curr_row].push('.');
        }

        curr_row += 1;
    }
    // Print each row of the drawing
    for row in crt_drawing {
        for pixel in row {
            print!("{}", pixel);
        }
        println!("");
    }
    Some(0)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_one_tiny() {
        let input = "noop
        addx 3
        addx -5".to_string();
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
