pub fn get_calories_vector(input: &str) -> Vec<u32> {
    // This function needs to load each line of the input file into a vector of strings.
    // The input file is a list of numbers, one per line.
    // The function needs to create an array of numbers from the vector of strings.
    // Each array will be a sum of the different numbers in the array, they
    // will be split by a space in the input file.
    // The function will find the max value in that number array and return it.
    // 
    // Args:
    //     input: A string containing the input file.
    // Output:
    //    The max value in the array of numbers.
    
    // Create empty vector to which we will push the numbers from the input file.
    let mut calories: Vec<u32> = Vec::new();

    // Iterate over each line in the input file.
    let mut curr_calories: u32 = 0;
    for line in input.lines() {
        // If the line is empty, we have a new elf, we reset the calories to 0
        // and append the current calories to the vector.
        if line.is_empty() {
            calories.push(curr_calories);
            curr_calories = 0;
        } else {
            // If the line is not empty, we add the calories to the current calories.
            curr_calories += line.parse::<u32>().unwrap();
        }
    }
    calories
}

pub fn part_one(input: &str) -> Option<u32> {
    
    // Get the vector of calories.
    let calories = get_calories_vector(input);

    // Now we get the max value in the vector.
    let max_calories = calories.iter().max().unwrap();
    
    // And return it.
    Some(*max_calories)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Now we use the same code from part_one
    // but we need to find the max 3 values in the vector.
    let mut calories = get_calories_vector(input);

    // Sort the vector in descending order.
    calories.sort();

    // Get the last 3 values.
    let max_calories: Vec<u32> = calories[calories.len()-3..].to_vec();

    // And return the sum of the 3 values.
    Some(max_calories.iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
