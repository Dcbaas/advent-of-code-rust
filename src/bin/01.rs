use std::{collections::BinaryHeap};

pub fn part_one(input: &str) -> Option<u32> {
    if input == "" {
        return None;
    }

    let mut max_calories = 0;

    let mut current_calories = 0;
    for line in input.lines() {
        if line == "" {
            max_calories = if current_calories > max_calories {
                current_calories
            } else {
                max_calories
            };
            current_calories = 0;
        } else {
            let line_calories = line.parse::<u32>().unwrap();
            current_calories += line_calories;
        }
    }
    Some(max_calories)
}



pub fn part_two(input: &str) -> Option<u32> {
    let mut calorie_counts = BinaryHeap::new();

    let mut current_calories = 0;
    for line in input.lines() {
        if line == "" {
            calorie_counts.push(current_calories);
            current_calories = 0;
        } else {
            let line_calories = line.parse::<u32>().unwrap();
            current_calories += line_calories;
        }
    }

    if current_calories != 0 {
        // Solution doesn't detect extra blank line on end of input.
        calorie_counts.push(current_calories);
    }

    let mut total_calories:u32 = 0;
    let mut elf_count = 0;
    while elf_count != 3 && !calorie_counts.is_empty() {
        total_calories = total_calories + *calorie_counts.pop().get_or_insert(0);
        elf_count += 1;
    }
    Some(total_calories)
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
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
