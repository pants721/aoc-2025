use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let input = input
        .lines()
        .map(|l| l.split_whitespace().collect_vec())
        .collect_vec();
    let (lines, signs) = input.split_at(input.len() - 1);
    let lines = lines.iter()
        .map(|l| l.iter()
            .map(|n| n.parse::<u64>().unwrap())
            .collect_vec())
        .collect_vec();
    let signs = &signs[0];

    let mut sum = 0;
    for i in 0..signs.len() {
        let sign = signs[i];
        let nums = lines.iter().map(|l| l[i]).collect_vec();

        match sign {
            "+" => sum += nums.iter().sum::<u64>(),
            "*" => sum += nums.iter().product::<u64>(),
            &_ => (),
        }
    }

    Some(sum)
}

fn transpose<T: Clone>(grid: &[Vec<T>]) -> Vec<Vec<T>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut out = vec![vec![grid[0][0].clone(); rows]; cols];

    for r in 0..rows {
        for c in 0..cols {
            out[c][r] = grid[r][c].clone();
        }
    }
    out
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input
        .lines()
        .collect_vec();
    let (lines, signs) = input.split_at(input.len() - 1);

    let lines = lines.iter()
        .map(|s| s.chars().collect_vec())
        .collect_vec();

    let nums = transpose(&lines) // rows -> cols
        .split(|v| v.iter().all(|&c| c.is_whitespace())) // seperate by the whitespace only columns
        .map(|c| c.into_iter()
            // parse to ints
            .map(|v| v.iter()
                .collect::<String>()
                .trim()
                .parse::<u64>().unwrap())
            .collect_vec())
        .collect_vec();
    let signs = &signs[0].split_whitespace().collect_vec();

    let mut sum = 0;
    for (sign, terms) in signs.into_iter().zip(nums) {
        match *sign {
            "+" => sum += terms.iter().sum::<u64>(),
            "*" => sum += terms.iter().product::<u64>(),
            &_ => ()
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
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
