use std::{collections::HashSet, ops::RangeInclusive};

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
    let (ranges, _) = input.split_once("\n\n").unwrap();
    let mut ranges = ranges.trim().lines().map(|l| {
        let (lhs, rhs) = l.split_once('-').unwrap();
        lhs.parse::<u64>().unwrap()..=rhs.parse::<u64>().unwrap()
    }).collect_vec();
    ranges.sort_by_key(|r| *r.start());

    if ranges.is_empty() {
        return None;
    }

    let mut total = 0;
    let mut curr_start = *ranges[0].start();
    let mut curr_end = *ranges[0].end();

    for r in ranges.into_iter().skip(1) {
        let s = *r.start();
        let e = *r.end();

        if s <= curr_end + 1 {
            curr_end = curr_end.max(e);
        } else {
            total += curr_end - curr_start + 1;
            curr_start = s;
            curr_end = e;
        }
    }

    total += curr_end - curr_start + 1;


    Some(total)
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
        assert_eq!(result, Some(14));
    }
}
