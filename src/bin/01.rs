advent_of_code::solution!(1);

use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        if let (Some(first), Some(second)) = (parts.next(), parts.next()) {
            col1.push(first.parse().unwrap());
            col2.push(second.parse().unwrap());
        }
    }

    col1.sort();
    col2.sort();

    let mut result = 0;
    for i in 0..col1.len() {
        result += (col1[i] - col2[i]).abs()
    }

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut col1: Vec<i32> = Vec::new();
    let mut col2: HashMap<i32, i32> = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        if let (Some(first), Some(second)) = (parts.next(), parts.next()) {
            let v1 = first.parse().unwrap();
            let v2 = second.parse().unwrap();
            col1.push(v1);

            if !col2.contains_key(&v2) {
                col2.insert(v2, 0);
            }
            col2.entry(v2).and_modify(|value| *value += 1);
        }
    }

    let mut result = 0;
    for i in col1 {
        if col2.contains_key(&(i as i32)) {
            result += i * col2[&(i as i32)];
        }
    }

    Some(result as u32)
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
