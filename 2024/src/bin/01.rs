advent_of_code::solution!(1);
use itertools::Itertools;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    input.lines().for_each(|line| {
        if let Some((first, second)) = line.split_whitespace().collect::<Vec<_>>().split_first() {
            if let Ok(left_id) = first.parse::<u32>() {
                left.push(left_id);
            }
            if let Some(second) = second.first() {
                if let Ok(right_id) = second.parse::<u32>() {
                    right.push(right_id);
                }
            }
        }
    });

    Some(
        left.iter()
            .sorted()
            .zip(right.iter().sorted())
            .map(|(l, r)| l.abs_diff(*r))
            .collect::<Vec<u32>>()
            .iter()
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left: Vec<u32> = Vec::new();
    let mut right: HashMap<u32, u32> = HashMap::new();

    input.lines().for_each(|line| {
        if let Some((first, second)) = line.split_whitespace().collect::<Vec<_>>().split_first() {
            if let Ok(left_id) = first.parse::<u32>() {
                left.push(left_id);
            }
            if let Some(second) = second.first() {
                if let Ok(right_id) = second.parse::<u32>() {
                    right.entry(right_id).and_modify(|f| *f += 1).or_insert(1);
                }
            }
        }
    });

    Some(
        left.into_iter()
            .map(|f: u32| f * *(right.entry(f).or_default()))
            .collect::<Vec<u32>>()
            .iter()
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    #[ignore]
    fn test_part_one_input() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1941353));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
