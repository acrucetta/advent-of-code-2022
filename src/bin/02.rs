pub enum Shapes {
    Rock(i32),
    Paper(i32),
    Scissors(i32),
}

pub enum RoundScore {
    Win(i32),
    Lose(i32),
    Draw(i32),
}

impl Shapes {
    fn get_score(&self) -> i32 {
        match self {
            &Shapes::Rock(x) => x,
            &Shapes::Paper(x) => x,
            &Shapes::Scissors(x) => x,
        }
    }
}

pub fn calculate_round_result(player_move: &Shapes, opponent_move: &Shapes) -> RoundScore {
    match player_move {
        Shapes::Rock(_) => match opponent_move {
            Shapes::Rock(_) => RoundScore::Draw(3),
            Shapes::Paper(_) => RoundScore::Lose(0),
            Shapes::Scissors(_) => RoundScore::Win(6),
        },
        Shapes::Paper(_) => match opponent_move {
            Shapes::Rock(_) => RoundScore::Win(6),
            Shapes::Paper(_) => RoundScore::Draw(3),
            Shapes::Scissors(_) => RoundScore::Lose(0),
        },
        Shapes::Scissors(_) => match opponent_move {
            Shapes::Rock(_) => RoundScore::Lose(0),
            Shapes::Paper(_) => RoundScore::Win(6),
            Shapes::Scissors(_) => RoundScore::Draw(3),
        },
    }
}

pub fn get_winning_shape(shape: &Shapes) -> Shapes {
    match shape {
        Shapes::Rock(_) => Shapes::Paper(2),
        Shapes::Paper(_) => Shapes::Scissors(3),
        Shapes::Scissors(_) => Shapes::Rock(1),
    }
}

pub fn get_losing_shape(shape: &Shapes) -> Shapes {
    match shape {
        Shapes::Rock(_) => Shapes::Scissors(3),
        Shapes::Paper(_) => Shapes::Rock(1),
        Shapes::Scissors(_) => Shapes::Paper(2),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut score = 0;
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        
        let opponent_move = match parts.next() {
            Some("A") => Shapes::Rock(1),
            Some("B") => Shapes::Paper(2),
            Some("C") => Shapes::Scissors(3),
            _ => return None,
        };
        let player_move = match parts.next() {
            Some("X") => Shapes::Rock(1),
            Some("Y") => Shapes::Paper(2),
            Some("Z") => Shapes::Scissors(3),
            _ => return None,
        };
        // Add the points to the score as well as the points for the move
        // Make a copy of the player move so we can get the points
        score += match calculate_round_result(&player_move, &opponent_move) {
            RoundScore::Win(points) => points + player_move.get_score(),
            RoundScore::Lose(points) => points +player_move.get_score(),
            RoundScore::Draw(points) => points + player_move.get_score(),
        };
    }
    // Return the score
    Some(score.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut score = 0;
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        
        let opponent_move = match parts.next() {
            Some("A") => Shapes::Rock(1),
            Some("B") => Shapes::Paper(2),
            Some("C") => Shapes::Scissors(3),
            _ => return None,
        };
        let round_result = match parts.next() {
            Some("Y") => RoundScore::Draw(3),
            Some("Z") => RoundScore::Win(6),
            Some("X") => RoundScore::Lose(0),
            _ => return None,
        };
        // Get the move that would have the desired result
        // given the opponent's move
        let player_move = match round_result {
            RoundScore::Draw(_) => opponent_move,
            RoundScore::Win(_) => get_winning_shape(&opponent_move),
            RoundScore::Lose(_) => get_losing_shape(&opponent_move),
        };
        // Add the points to the score as well as the points for the move
        score += match round_result {
            RoundScore::Win(points) => points + player_move.get_score(),
            RoundScore::Lose(points) => points + player_move.get_score(),
            RoundScore::Draw(points) => points + player_move.get_score(),
        };
    }
    // Return the score
    Some(score.try_into().unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
