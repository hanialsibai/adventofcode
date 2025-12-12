use itertools::Itertools;
use std::ops::Index;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;

    'line: for line in input.lines() {
        let mut line_numbers: Vec<u32> = Vec::new();

        for n in line.split_whitespace() {
            if let Ok(number) = n.parse::<u32>() {
                if let Some(last_n) = line_numbers.last() {
                    if last_n.abs_diff(number) < 1 || last_n.abs_diff(number) > 3 {
                        continue 'line;
                    } else {
                        line_numbers.push(number);
                    }
                } else {
                    line_numbers.push(number);
                    continue;
                }
            }
        }

        if is_sorted(&line_numbers, true) || is_sorted(&line_numbers, false) {
            result += 1;
        }
    }

    Some(result)
}

fn is_sorted(input: &Vec<u32>, ascending: bool) -> bool {
    if input.len() < 2 {
        return true;
    }

    if ascending {
        for i in 0..input.len() - 1 {
            if input.index(i) > input.index(i + 1) {
                return false;
            }
        }
    } else {
        for (index, n) in input.iter().enumerate().skip(1) {
            if n > input.index(index - 1) {
                return false;
            }
        }
    }
    true
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect()
            })
            .collect::<Vec<Vec<i32>>>()
            .iter()
            .filter(|report| is_safe(report, true))
            .count() as u32,
    )
}

fn is_safe(report: &[i32], flag: bool) -> bool {
    let direction = (report[1] - report[0]).signum();
    for w in report.windows(2) {
        let diff = w[1] - w[0];
        if diff.signum() != direction || diff.abs() < 1 || diff.abs() > 3 {
            return if flag {
                report
                    .iter()
                    .combinations(report.len() - 1)
                    .any(|r| is_safe(&r.iter().map(|&&x| x).collect_vec(), false))
            } else {
                false
            };
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    #[ignore]
    fn test_input_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(639));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    #[ignore]
    fn test_input_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(674));
    }
}
