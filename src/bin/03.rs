advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    let re = match Regex::new(r"mul\(\d{1,3},\d{1,3}\)") {
        Ok(v) => v,
        Err(e) => {
            panic!("Failed to parse regex: {e}");
        }
    };

    let matches: Vec<_> = re.find_iter(input).map(|m| m.as_str()).collect();
    for m in matches {
        let match_re = match Regex::new(r"\d+") {
            Ok(v) => v,
            Err(e) => {
                panic!("Failed to parse regex: {e}");
            }
        };

        result += match_re
            .find_iter(m)
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
            .iter()
            .copied()
            .reduce(|a, b| a * b)
            .unwrap();
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result: u32 = 0;

    let mut should_apply: bool = true;

    let matches = match Regex::new(r"mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\)") {
        Ok(v) => v,
        Err(e) => {
            panic!("Failed to parse regex: {e}");
        }
    }
    .find_iter(input)
    .map(|m| m.as_str())
    .collect::<Vec<&str>>();

    for m in matches {
        match m {
            "don't()" => should_apply = false,
            "do()" => should_apply = true,
            _ => {
                if should_apply {
                    let match_re = match Regex::new(r"\d+") {
                        Ok(v) => v,
                        Err(e) => {
                            panic!("Failed to parse regex: {e}");
                        }
                    };
                    result += match_re
                        .find_iter(m)
                        .map(|m| m.as_str().parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                        .iter()
                        .copied()
                        .reduce(|a, b| a * b)
                        .unwrap()
                }
            }
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }
    #[test]
    fn test_part_one_input() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(179834255));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }

    #[test]
    fn test_part_two_input() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(80570939));
    }
}
