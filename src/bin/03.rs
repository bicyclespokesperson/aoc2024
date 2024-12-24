advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    if input.len() == 0 {
        panic!("empty input!");
    }
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut result: u32 = 0;
    for m in re.captures_iter(input) {
        //println!("{:?}", m.get(0));
        //println!("{:?}", m.get(1));
        //println!("{:?}", m.get(2));
        let v1 = m.get(1).unwrap().as_str();
        let v2 = m.get(2).unwrap().as_str();
        result += v1.parse::<u32>().unwrap() * v2.parse::<u32>().unwrap();
    }

    return Some(result);
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
