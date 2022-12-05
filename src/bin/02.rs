#[derive(Eq, PartialEq, Clone, Copy)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

// Panic because we can't recover from this situation
fn map_letter_to_action(letter: char) -> Action {
    if letter == 'A' || letter == 'X' {
        return Action::Rock;
    }
    else if letter == 'B' || letter == 'Y' {
        return Action::Paper;
    }
    else if letter == 'C' || letter == 'Z' {
        return Action::Scissors;
    }
    panic!("Unknown action letter {}", letter)
}

fn is_winning_action(action: &Action, opponent_action: &Action) -> bool {
    let losing_action = match action {
        Action::Paper => &Action::Rock,
        Action::Rock => &Action::Scissors,
        Action::Scissors => &Action::Paper
    };

    opponent_action == losing_action
}

fn get_action_points(action: &Action) -> u32 {
    return match action {
        Action::Rock => 1,
        Action::Paper => 2,
        Action::Scissors => 3,
    };
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut points: u32 = 0;
    for line in input.lines() {
        let opponent_move = map_letter_to_action(line.chars().nth(0)?);
        let my_move = map_letter_to_action(line.chars().nth(2)?);
        points += get_action_points(&my_move);

        if my_move == opponent_move {
            points += 3;
        }
        else if is_winning_action(&my_move, &opponent_move) {
            points += 6;
        }
    }
    Some(points)
}

fn get_losing_action(action: &Action) -> Action {
    return match action {
        Action::Paper => Action::Rock,
        Action::Rock => Action::Scissors,
        Action::Scissors => Action::Paper
    };
}

fn get_winning_action(action: &Action) -> Action {
    return match action {
        Action::Paper => Action::Scissors,
        Action::Rock => Action::Paper,
        Action::Scissors => Action::Rock,
    };
}

fn get_my_action(letter: char, opponent_action: &Action) -> Action {
    return match letter {
        'X' => get_losing_action(opponent_action),
        'Y' => *opponent_action,
        'Z' => get_winning_action(opponent_action),
        _ => panic!("Unknown letter {}", letter)
    };
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut points: u32 = 0;
    for line in input.lines() {
        let opponent_move = map_letter_to_action(line.chars().nth(0)?);
        let my_move = get_my_action(line.chars().nth(2)?, &opponent_move);
        points += get_action_points(&my_move);

        if my_move == opponent_move {
            points += 3;
        }
        else if is_winning_action(&my_move, &opponent_move) {
            points += 6;
        }
    }
    Some(points)
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
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
