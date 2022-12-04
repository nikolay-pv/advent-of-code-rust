use std::collections::HashSet;

const INPUT_TXT: &str = include_str!("input.txt");

fn main() {
    let input = read_input(INPUT_TXT);
    println!("Answer to first parts is {}", solve_first(&input));
    println!("Answer to second parts is {}", solve_second(&input));
}

fn get_score(c: char) -> i32 {
    if 'a' <= c && c <= 'z' {
        return c as i32 - 'a' as i32 + 1;
    } else if 'A' <= c && c <= 'z' {
        return c as i32 - 'A' as i32 + 27;
    }
    panic!("unexpected char in get_score")
}

fn solve_first(input: &Vec<&str>) -> i32 {
    let mut sum = 0;
    for line in input {
        assert!(line.len() % 2 == 0);
        let m = HashSet::<char>::from_iter(line[..line.len()/2].chars().into_iter());
        for c in line[line.len()/2..].chars() {
            if m.contains(&c) {
                sum += get_score(c);
                break;
            }
        }
    }
    return sum;
}

fn get_intersection_score(seen: &[u64; 3]) -> i32 {
    let mut common = seen[0] & seen[1] & seen[2];
    let mut counter = 0;
    while common != 1 {
        common = common >> 1;
        counter += 1;
    }
    return counter;
}

fn solve_second(input: &Vec<&str>) -> i32 {
    let mut sum = 0;
    // 3 bitmaps to store the 'seen' chars, where 0b10 == 'a'
    let mut seen = [0u64; 3];
    for (i, line) in input.iter().enumerate() {
        let idx = i % 3;
        for c in line.chars() {
            // set bit corresponding to the found char
            seen[idx] |= 1 << get_score(c);
        }
        if idx == 2 {
            sum += get_intersection_score(&seen);
            seen = [0u64,0u64,0u64];
        }
    }
    return sum;
}

fn read_input(file_content: &str) -> Vec<&str> {
    file_content.lines().into_iter().collect()
}


#[cfg(test)]
mod tests {
    use crate::*;
    const TEST_INPUT_TXT: &str = include_str!("input_test.txt");

    #[test]
    fn part1() {
        let input = read_input(TEST_INPUT_TXT);
        assert_eq!(solve_first(&input), 157);
    }

    #[test]
    fn part2() {
        let input = read_input(TEST_INPUT_TXT);
        assert_eq!(solve_second(&input), 70);
    }
}

