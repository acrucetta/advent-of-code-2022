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
    let mut unstructured_monkeys = Vec::new();
    // First we split the input into a vector of strings, each string being a monkey
    // we will be separating the input by a blank line between monkeys
    for monkey in input.split("\n\n") {
        unstructured_monkeys.push(monkey);
    
    }
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
    structured_monkeys
}

fn parse_id(input: &str) -> String {
    let mut id = String::new();
    for line in input.lines() {
        let line = line.trim();
        if line.starts_with("Monkey") {
            id = line.split_whitespace().nth(1).unwrap().to_string();
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
            let re = Regex::new(r"^Operation: new = old ([\*\+\-\$/$]) ([\d]+)$").unwrap();

            // Try to extract the operation and operand from the input string.
            let captures = re.captures(input).unwrap();
            operation.operation = captures.get(1).unwrap().as_str().chars().next().unwrap();
            operation.value = captures.get(2).unwrap().as_str().to_string();
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
            test.divisible_by = re.captures(input).unwrap().get(1).unwrap().as_str().parse().unwrap();
        }
        if line.starts_with("If true") {
            // parse the input of the form "If true: throw to monkey 6"
            let re = Regex::new(r"^If true: throw to monkey ([\d]+)$").unwrap();
            test.true_monkey_id = re.captures(input).unwrap().get(1).unwrap().as_str().to_string();
        }
        if line.starts_with("If false") {
            // parse the input of the form "If false: throw to monkey 2"
            let re = Regex::new(r"^If false: throw to monkey ([\d]+)$").unwrap();
            test.false_monkey_id = re.captures(input).unwrap().get(1).unwrap().as_str().to_string();
        }
    }
    test
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
