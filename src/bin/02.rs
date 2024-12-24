advent_of_code::solution!(2);

fn parse_reports(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|val: &str| {
            val.split_whitespace()
                .map(|n: &str| n.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(report: &[i32]) -> bool {
    let pred = if report[0] < report[1] {
        |a, b| a < b
    } else {
        |a, b| a > b
    };

    for i in 1..report.len() {
        if !pred(report[i - 1], report[i]) || (report[i - 1] - report[i]).abs() > 3 {
            return false;
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(parse_reports(input).iter().filter(|a| is_safe(a)).count() as u32)
}

fn with_each_index_removed(v: &[i32]) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    for i in 0..v.len() {
        let mut tmp: Vec<i32> = Vec::new();
        for j in 0..v.len() {
            if i != j {
                tmp.push(v[j])
            }
        }
        result.push(tmp);
    }

    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse_reports(input);
    let mut count: u32 = 0;
    for report in reports {
        let all_options = with_each_index_removed(report.as_slice());
        if all_options.iter().map(|a| a.as_slice()).any(is_safe) {
            count += 1;
        }
    }
    Some(count)
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
