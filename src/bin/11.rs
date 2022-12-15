use regex::Regex;

/*
Approach:
- Parse the input into a struct Monkey
- Add a method for each monkey to process his items
- Add a structure for each turn
- Add a structure for each round
*/
struct Monkey {
    id: String,
    items: Vec<i32>,
    operation: Operation,
    test: Test,
}

struct Operation {
    operation: char,
    value: String,
}

struct Test {
    divisible_by: i32,
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
        let id = parse_id(monkey);
        let items = parse_items(monkey);
        let operation = parse_operation(monkey);
        let test = parse_test(monkey);
        let monkey = Monkey {
            id,
            items,
            operation,
            test,
        };
        structured_monkeys.push(monkey);
    }
    
    let structured_monkeys : Vec<Monkey> = structured_monkeys.into_iter().filter(|monkey| !monkey.id.is_empty()).collect();
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
            id = re.captures(line).unwrap().get(1).unwrap().as_str().to_string();
        }
    }
    id
}

fn parse_items(input: &str) -> Vec<i32> {
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
            test.divisible_by = re.captures(line).unwrap().get(1).unwrap().as_str().parse().unwrap();
        }
        if line.starts_with("If true") {
            // parse the input of the form "If true: throw to monkey 6"
            let re = Regex::new(r"^If true: throw to monkey ([\d]+)$").unwrap();
            test.true_monkey_id = re.captures(line).unwrap().get(1).unwrap().as_str().to_string();
        }
        if line.starts_with("If false") {
            // parse the input of the form "If false: throw to monkey 2"
            let re = Regex::new(r"^If false: throw to monkey ([\d]+)$").unwrap();
            test.false_monkey_id = re.captures(line).unwrap().get(1).unwrap().as_str().to_string();
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

fn update_worry_level(monkey: &mut Monkey, item: i32) -> i32 {
    // If the monkey operation value is old, we will use the same item.
    let item = if monkey.operation.value == "old" {
        item
    } else {
        monkey.operation.value.parse().unwrap()
    };
    let operation = monkey.operation.operation;
    let worry_level: i32 = match operation {
        '+' => item + item,
        '-' => item - item,
        '*' => item * item,
        '/' => item / item,
        _ => panic!("Invalid operation"),
    };
    worry_level/3 
}

fn monkey_round(monkeys: &mut Vec<Monkey>) {
    for monkey in monkeys {
        // Inspect the item
        let item = monkey.items.remove(0);

        // Update the worry level
        let worry_level = update_worry_level(monkey, item);

        // Test the worry level
        if worry_level % monkey.test.divisible_by == 0 {
            // Take the true monkey from the vector
            let true_monkey = monkeys.iter_mut()
                .find(|m| m.id == monkey.test.true_monkey_id)
                .unwrap()
                .take();

            // Modify the true monkey
            true_monkey.items.push(worry_level);

            // Put the true monkey back in the vector
            monkeys.push(true_monkey);

        } else {
            // Take the false monkey from the vector
            let false_monkey = monkeys.iter_mut()
                .find(|m| m.id == monkey.test.false_monkey_id)
                .unwrap()
                .take();

            // Modify the false monkey
            false_monkey.items.push(worry_level);

            // Put the false monkey back in the vector
            monkeys.push(false_monkey);
        }
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    let monkeys = parse_input(input);
    Some(1)
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
