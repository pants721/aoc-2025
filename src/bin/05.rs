use std::ops::RangeInclusive;

use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    let mut ranges = ranges.trim().lines().map(|l| {
        let (lhs, rhs) = l.split_once('-').unwrap();
        lhs.parse::<u64>().unwrap()..=rhs.parse::<u64>().unwrap()
    }).collect_vec();
    ranges.sort_by_key(|r| *r.start());
    let ids = ids.trim().lines().map(|l| l.parse::<u64>().unwrap()).collect_vec();

    Some(ids.iter().filter(|id| ranges_contains(&ranges, **id)).count() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    let mut ranges = ranges.trim().lines().map(|l| {
        let (lhs, rhs) = l.split_once('-').unwrap();
        lhs.parse::<u64>().unwrap()..=rhs.parse::<u64>().unwrap()
    }).collect_vec();
    ranges.sort_by_key(|r| *r.start());
    let ids = ids.trim().lines().map(|l| l.parse::<u64>().unwrap()).collect_vec();

    None
}

fn ranges_contains(ranges: &[RangeInclusive<u64>], x: u64) -> bool {
    for range in ranges {
        if range.contains(&x) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
