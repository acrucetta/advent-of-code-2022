use std::collections::HashMap;

use regex::Regex;

/*
Approach:
- Parse the input into a struct Monkey
- Add a method for each monkey to process his items
- Add a structure for each turn
- Add a structure for each round
*/

#[derive(Clone)]
struct Monkey {
    id: String,
    items: Vec<u64>,
    operation: Operation,
    test: Test,
    inspections: u64,
}

#[derive(Clone)]
struct Operation {
    operation: char,
    value: String,
}

#[derive(Clone)]

struct Test {
    divisible_by: u64,
    true_monkey_id: String,
    false_monkey_id: String,
}

fn parse_input(input: &str) -> Vec<Monkey> {
    /*
    We will be parsing an input like this:
    Monkey 0:
        Starting items: 59, 74, 65, 86
        Operation: new = old * 19
        Test: divisible by 7
            If true: throw to monkey 6
            If false: throw to monkey 2

    From this we will create a Monkey struct with the following fields:
    id: String
    items: Vec<i32>
    operation: Operation
    test: Test
    */
    // First we split the input into a vector of strings, each string being a monkey
    // we will be separating the input by a blank line between monkeys
    let unstructured_monkeys = input.split("Monkey").collect::<Vec<&str>>();

    let mut structured_monkeys = Vec::new();
    for monkey in unstructured_monkeys {
        if monkey.is_empty() {
            continue;
        }
        let id = parse_id(monkey);
        let items = parse_items(monkey);
        let operation = parse_operation(monkey);
        let test = parse_test(monkey);
        let monkey = Monkey {
            id,
            items,
            operation,
            test,
            inspections: 0,
        };
        structured_monkeys.push(monkey);
    }

    let structured_monkeys: Vec<Monkey> = structured_monkeys
        .into_iter()
        .filter(|monkey| !monkey.id.is_empty())
        .collect();
    structured_monkeys
}

fn parse_id(input: &str) -> String {
    let mut id = String::new();
    for line in input.lines() {
        let line = line.trim();
        // We want to parse the input of the form "0:"
        // We will use regex to parse the id, we will get 0
        // as the id.
        // Check if the line starts with a number and ends with a colon
        if line.starts_with(char::is_numeric) && line.ends_with(':') {
            // Create a regular expression to match the pattern we are looking for.
            let re = Regex::new(r"^([\d]+):$").unwrap();

            // Try to extract the id from the input string.
            id = re
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .to_string();
        }
    }
    id
}

fn parse_items(input: &str) -> Vec<u64> {
    let mut items = Vec::new();
    for line in input.lines() {
        // Strip line of whitespace
        let line = line.trim();
        if line.starts_with("Starting items") {
            // Line is of the form "Starting items: 59, 74, 65, 86"
            // We want to split the line by the colon, then by the comma
            let items_str = line.split(':').nth(1).unwrap().trim();
            for item in items_str.split(',') {
                items.push(item.trim().parse().unwrap());
            }
        }
    }
    items
}

fn parse_operation(input: &str) -> Operation {
    let mut operation = Operation {
        operation: ' ',
        value: String::new(),
    };
    for line in input.lines() {
        let line = line.trim();
        if line.starts_with("Operation") {
            // Parse the input of the form "Operation: new = old * 19"
            // We will use regex to parse the operation, we will get *
            // as the operation and 19 as the value.
            // Create a regular expression to match the pattern we are looking for.
            match Regex::new(r"^Operation: new = old ([\*\+\-\$/$]) (\w+)$") {
                Ok(re) => {
                    // Try to extract the operation and operand from the input string.
                    let captures = re.captures(line).unwrap();
                    operation.operation = captures.get(1).unwrap().as_str().chars().next().unwrap();
                    operation.value = captures.get(2).unwrap().as_str().to_string();
                }
                Err(e) => {
                    panic!("Error: {}", e);
                }
            }
        }
    }
    operation
}

fn parse_test(input: &str) -> Test {
    let mut test = Test {
        divisible_by: 0,
        true_monkey_id: String::new(),
        false_monkey_id: String::new(),
    };
    for line in input.lines() {
        let line = line.trim();
        if line.starts_with("Test") {
            // parse the input of the form "Test: divisible by 7"
            // We will use regex to parse the operation, we will get 7
            // as the value.
            let re = Regex::new(r"^Test: divisible by ([\d]+)$").unwrap();
            test.divisible_by = re
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
        }
        if line.starts_with("If true") {
            // parse the input of the form "If true: throw to monkey 6"
            let re = Regex::new(r"^If true: throw to monkey ([\d]+)$").unwrap();
            test.true_monkey_id = re
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .to_string();
        }
        if line.starts_with("If false") {
            // parse the input of the form "If false: throw to monkey 2"
            let re = Regex::new(r"^If false: throw to monkey ([\d]+)$").unwrap();
            test.false_monkey_id = re
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .to_string();
        }
    }
    test
}

/*
Monkey Round:
- A monkey round starts with a list of monkeys and their starting items.
- Each monkey has an id, a list of items, an operation, and a test.
- The monkey will inspect the item, update the worry level, and then throw the item to another monkey (end of queue).
- After the monkey inspects an item, we will divide the worry level of the item by 3.
- We will then test the worry level of the item and throw the item to respective monkey.
*/

fn update_worry_level(monkey: &mut Monkey, item: u64) -> u64 {
    // If the monkey operation value is old, we will use the same item.
    let operand: u64 = item;
    let operand = if monkey.operation.value == "old" {
        operand
    } else {
        monkey.operation.value.parse().unwrap()
    };
    let operation = monkey.operation.operation;
    let worry_level: u64 = match operation {
        '+' => item + operand,
        '-' => item - operand,
        '*' => item * operand,
        '/' => item / operand,
        _ => panic!("value operation"),
    };
    worry_level % item
}

fn monkey_round(monkeys: &mut Vec<Monkey>) {
    // Items to move to each monkey
    let mut items_to_move: HashMap<String, Vec<u64>> = monkeys
        .iter()
        .map(|monkey| (monkey.id.clone(), monkey.items.clone()))
        .collect();

    for monkey in monkeys.iter_mut() {
        // While we have items to inspect, inspect them
        while items_to_move.get(&monkey.id).unwrap().len() > 0 {
            
            // Inspect the item
            let item = items_to_move.get_mut(&monkey.id).unwrap().remove(0);
            monkey.inspections += 1;

            // Update the worry level
            let worry_level:u64 = update_worry_level(monkey, item);

            // Test the worry level
            if worry_level % monkey.test.divisible_by == 0 {
                // Add it to the items to move list with the monkey id
                items_to_move
                    .entry(monkey.test.true_monkey_id.clone())
                    .or_insert(vec![])
                    .push(worry_level);
            } else {
                // Add it to the items to move list with the monkey id
                items_to_move
                    .entry(monkey.test.false_monkey_id.clone())
                    .or_insert(vec![])
                    .push(worry_level);
            }
        }
    }
    monkeys.iter_mut().for_each(|monkey| {
        monkey.items = items_to_move.get(&monkey.id).unwrap().clone();
    });
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut monkeys = parse_input(input);
    // Print initial state
    println!("Initial state:");
    for monkey in monkeys.iter() {
        println!("Monkey {} items: {:?}", monkey.id, monkey.items);
    }

    for round in 1..=10000 {
        monkey_round(&mut monkeys);

        println!("Round: {}", round);
        // Print the items of each monkey
        for monkey in monkeys.iter() {
            println!("Monkey {} items: {:?}", monkey.id, monkey.items);
        }
    }
    // Print the inspect count of each monkey
    for monkey in monkeys.iter() {
        println!("Monkey {} inspect count: {}", monkey.id, monkey.inspections);
    }
    // Return the multiplication of the max two inspect counts
    let mut inspect_counts: Vec<u64> = monkeys.iter().map(|monkey| monkey.inspections).collect();
    inspect_counts.sort();
    Some((inspect_counts[inspect_counts.len() - 1] * inspect_counts[inspect_counts.len() - 2]) as u64)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
    }
}
