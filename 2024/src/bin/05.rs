advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let rules = sections[0];

    for rule in sections[0].lines(){
        let split: Vec<&str> = rule.split("|").collect();
        let left = split[0].parse::<u32>().unwrap();
        let right = split[1].parse::<u32>().unwrap();
    }
    
    let pages = sections[1];
    for s in sections.iter(){
        println!("{s}");
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
   None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
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


    #[test]
    fn test_part_two_input() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(31));
    }
}