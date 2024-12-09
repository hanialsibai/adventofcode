advent_of_code::solution!(4);
use std::collections::HashMap;


type Point = (i32, i32);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.trim().lines().collect::<Vec<&str>>();
    let mut start_points = Vec::<Point>::new();
    let mut grid =  HashMap::<Point, char>::new();

    for (y, line) in lines.iter().enumerate(){
        for (x, c) in line.chars().enumerate(){

            grid.insert((x as i32, y as i32), c);

            if c == 'X'{
                start_points.push((x as i32, y as i32));
            }
        }
    }
    let mut result = 0;
    let mut mas = "".to_string();
    for start in start_points.iter() {
        for direction in &get_directions_part1(&start){
            for point in direction {
                mas.push(grid.get(&point).unwrap_or(&'.').to_owned());
            }
            if mas == "MAS" {
                result += 1;
            }
            mas.clear();
        }
    }

    Some(result)
}


fn get_directions_part1(start: &Point) -> Vec<Vec<Point>> {
    vec![
        // UP
        vec![
            (start.0, start.1-1),
            (start.0, start.1-2),
            (start.0, start.1-3),
        ],
        // UP and LEFT
        vec![
            (start.0+1, start.1-1),
            (start.0+2, start.1-2),
            (start.0+3, start.1-3),
        ],
        //LEFT
        vec![
            (start.0+1, start.1),
            (start.0+2, start.1),
            (start.0+3, start.1),
        ],
        //LEFT and DOWN
        vec![
            (start.0+1, start.1+1),
            (start.0+2, start.1+2),
            (start.0+3, start.1+3),
        ],
        //DOWN
        vec![
            (start.0, start.1+1),
            (start.0, start.1+2),
            (start.0, start.1+3),
        ],
        //DOWN and RIGHT
        vec![
            (start.0-1, start.1+1),
            (start.0-2, start.1+2),
            (start.0-3, start.1+3),
        ],
        //RIGHT
        vec![
            (start.0-1, start.1),
            (start.0-2, start.1),
            (start.0-3, start.1),
        ],
        //UP AND RIGHT
        vec![
            (start.0-1, start.1-1),
            (start.0-2, start.1-2),
            (start.0-3, start.1-3),
        ]
    ]
}


fn get_directions_part2(start: &Point) -> Vec<Vec<Point>> {
    vec![
        vec![
            (start.0-1, start.1-1), // top left
            (start.0+1, start.1+1) // bottom right
        ],
        vec![
            (start.0+1, start.1-1), // top right
            (start.0-1, start.1+1) // bottom left
        ]
    ]
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.trim().lines().collect::<Vec<&str>>();
    let mut start_points = Vec::<Point>::new();
    let mut grid =  HashMap::<Point, char>::new();

    for (y, line) in lines.iter().enumerate(){
        for (x, c) in line.chars().enumerate(){

            grid.insert((x as i32, y as i32), c);

            if c == 'A'{
                start_points.push((x as i32, y as i32));
            }
        }
    }
    let mut result = 0;
    let mut mas = "".to_string();
    for start in start_points.iter() {
        let mut is_mas = false;
        for direction in &get_directions_part2(&start){
            for point in direction {
                mas.push(grid.get(&point).unwrap_or(&'.').to_owned());
            }

            if mas != "MS" && mas != "SM" {
                mas.clear();
                is_mas = false;
                break;
                
            }else{
                is_mas = true;
            }

            mas.clear();
        }

        if is_mas {
            result +=1;
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_one_input() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(2500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }

    #[test]
    fn test_part_two_input() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1933));
    }
}
