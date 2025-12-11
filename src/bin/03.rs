use itertools::Itertools;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .trim()
            .lines()
            .map(|l| joltage(
                &l
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect_vec(),
                2
            )
            )
            .sum()
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .trim()
            .lines()
            .map(|l| joltage(
                &l
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect_vec(),
                12
            )
            )
            .sum()
    )
}

fn joltage(digits: &[u8], n: usize) -> u64 {
    let mut out = 0;
    let mut after = 0;
    for batch in 1..=n {
        let size = digits.len() - (n - batch) - after;
        for i in (0..=9).rev() {
            if let Some(idx) = digits.iter().skip(after).take(size).position(|n| *n == i) {
                after += idx + 1;
                out = out * 10 + i as u64;
                break;
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
