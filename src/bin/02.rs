use itertools::Itertools;
use fancy_regex::Regex;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let pairs = input
        .split(',')
        .map(|s| {
            s
                .split('-')
                .filter_map(|s2| s2.parse::<u64>().ok())
                .collect_tuple::<(u64, u64)>()
        })
        .flatten()
        .collect_vec();

    let mut sum = 0;
    for pair in pairs {
        for num in pair.0..=pair.1 {
            // odd numbers are always valid
            let num_str = num.to_string();
            if num_str.len() % 2 != 0 {
                continue;
            }

            let halves = num_str.split_at(num_str.len() / 2);
            if halves.0 == halves.1 {
                sum += num;
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let pairs = input
        .split(',')
        .map(|s| {
            s
                .trim()
                .split('-')
                .filter_map(|s2| s2.parse::<u64>().ok())
                .collect_tuple::<(u64, u64)>()
        })
        .flatten()
        .collect_vec();

    let re = Regex::new(r"^(.+)\1+$").unwrap();
    let mut sum = 0;
    for pair in pairs {
        for num in pair.0..=pair.1 {
            let num_str = num.to_string();

            if re.is_match(&num_str).unwrap() {
                sum += num;
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
