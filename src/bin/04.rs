use itertools::Itertools;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let map = input.trim().lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let mut sum = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '@' {
                sum += can_access(&map, (x, y)) as u64;
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = input.trim().lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();
    Some(replace(&mut map))
}

fn replace(map: &mut Vec<Vec<char>>) -> u64 {
    let mut sum = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] != '@' {
                continue;
            }
            if can_access(&map, (x, y)) {
                map[y][x] = 'x';
                sum += 1;
            }
        }
    }

    if sum == 0 {
        return 0;
    }

    sum + replace(map)
}

fn can_access(map: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    let mut sum = 0;
    for y in pos.1.saturating_sub(1)..=(pos.1 + 1).min(map.len() - 1) {
        for x in pos.0.saturating_sub(1)..=(pos.0 + 1).min(map[y].len() - 1) {
            if (x, y) == pos {
                continue;
            }
            if map[y][x] == '@' {
                sum += 1;
            }
            if sum >= 4 {
                return false;
            }
        }
    }

    sum < 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
