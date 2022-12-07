const INPUT_TXT: &str = include_str!("input.txt");

fn main() {
    let input = read_input(INPUT_TXT);
    println!("Answer to first parts is {}", solve_first(&input, 4));
    println!("Answer to second parts is {}", solve_second(&input, 14));
}

fn solve_first(input: &str, threshold: usize) -> i32 {
    let chars = input.chars();

    let mut counts = [0; 26];
    let get_index = |c| c as usize - 'a' as usize;
    for ch in input.chars().take(threshold) {
        counts[get_index(ch)] += 1;
    }

    let zipper = chars.clone().into_iter().zip(chars.skip(threshold).into_iter());
    for (i, (first, second)) in zipper.enumerate() {
        if counts.iter().filter(|&&x| x > 0).count() == threshold {
            return (i + threshold) as i32;
        }
        counts[get_index(first)] -= 1;
        counts[get_index(second)] += 1;
    }
    if counts.iter().filter(|&&x| x > 0).count() == threshold {
        return input.len() as i32;
    }
    unreachable!();
}

fn solve_second(input: &str, threshold: usize) -> i32 {
    return solve_first(input, threshold);
}

fn read_input(file_content: &str) -> &str {
    file_content
}


#[cfg(test)]
mod tests {
    use crate::*;
    const TEST_INPUT_TXT: [&str;5] = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb", // 7
        "bvwbjplbgvbhsrlpgdmjqwftvncz", // 5
        "nppdvjthqldpwncqszvftbrmjlhg", // 6
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", // 10
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", // 11
        ];

    #[test]
    fn part1() {
        let want = [7, 5, 6, 10, 11];
        for (i, &line) in TEST_INPUT_TXT.iter().enumerate() {
            let input = read_input(line);
            assert_eq!(solve_first(&input, 4), want[i]);
        }
    }

    #[test]
    fn part2() {
        let want = [19, 23, 23, 29, 26];
        for (i, &line) in TEST_INPUT_TXT.iter().enumerate() {
            let input = read_input(line);
            assert_eq!(solve_second(&input, 14), want[i]);
        }
    }
}

