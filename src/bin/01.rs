advent_of_code::solution!(1);

const MIN: i32 = 0;
const MAX: i32 = 100;

pub fn part_one(input: &str) -> Option<u64> {
    let mut curr = 50;
    let mut sum = 0;

    for line in input.lines() {
        let dir = line.chars().nth(0).unwrap();
        let amt: i32 = line.chars().skip(1).collect::<String>().parse().unwrap();

        match dir {
            'L' => {
                curr -= amt % MAX;
                if curr < MIN {
                    curr += MAX;
                }
            },
            'R' => {
                curr += amt;
                if curr >= MAX {
                    curr %= MAX;
                }
            },
            _ => ()
        }

        if curr == 0 {
            sum += 1;
        }
    }

    return Some(sum);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut curr = 50;
    let mut sum = 0;

    for line in input.lines() {
        let dir = line.chars().nth(0).unwrap();
        let mut amt: i32 = line.chars().skip(1).collect::<String>().parse().unwrap();

        match dir {
            'L' => {
                sum += amt / MAX;
                amt %= MAX;

                curr -= amt;
                if curr < MIN {
                    if curr + amt != 0 {
                        sum += 1;
                    }
                    curr += MAX;
                }
            },
            'R' => {
                sum += amt / MAX;
                amt %= MAX;

                curr += amt;
                if curr > MAX {
                    sum += 1;
                }
                curr %= MAX;
            },

            _ => ()
        }

        if curr == 0 {
            sum += 1;
        }
    }

    return Some(sum as u64);
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
        assert_eq!(result, Some(8));
    }
}
