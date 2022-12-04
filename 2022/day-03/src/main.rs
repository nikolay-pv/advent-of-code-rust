const INPUT_TXT: &str = include_str!("input.txt");

fn main() {
    let input = read_input(INPUT_TXT);
    println!("Answer to first parts is {}", solve_first(&input));
    println!("Answer to second parts is {}", solve_second(&input));
}

// gets index for the given char, where 'a' has index of 1 and 'A' has index of 27, etc.
fn get_index(c: char) -> i32 {
    if 'a' <= c && c <= 'z' {
        return c as i32 - 'a' as i32 + 1;
    } else if 'A' <= c && c <= 'z' {
        return c as i32 - 'A' as i32 + 27;
    }
    panic!("unexpected char in get_index")
}

// gets index of the set bit, assuming only one bit is set, otherwise will return the highest set bit in mask
fn get_setbit_index(mut mask: u64) -> i32 {
    let mut counter = 0;
    while mask != 1 {
        mask >>= 1;
        counter += 1;
    }
    return counter;
}

fn solve_first(input: &Vec<&str>) -> i32 {
    let mut sum = 0;
    let mut seen = 0u64;
    for line in input {
        assert!(line.len() % 2 == 0);
        // set the bits for the first part
        line[..line.len()/2].chars().for_each(|c| set_bit(&mut seen, get_index(c)));
        // search for duplicate
        for c in line[line.len()/2..].chars() {
            let idx = get_index(c);
            if is_bit_set(seen, idx) {
                sum += idx;
                break;
            }
        }
        seen = 0;
    }
    return sum;
}

fn solve_second(input: &Vec<&str>) -> i32 {
    let mut sum = 0;
    // 3 bitmaps to store the 'seen' chars, where 0b10 == 'a'
    let mut seen = [0u64; 3];
    for (i, line) in input.iter().enumerate() {
        let idx = i % 3;
        // set bits for this line
        line.chars().for_each(|c| set_bit(&mut seen[idx], get_index(c)));
        if idx == 2 {
            // mask holds common item across three strings
            let mask = seen[0] & seen[1] & seen[2];
            sum += get_setbit_index(mask);
            seen = [0, 0, 0];
        }
    }
    return sum;
}

fn read_input(file_content: &str) -> Vec<&str> {
    file_content.lines().into_iter().collect()
}

#[inline(always)]
fn set_bit(mask: &mut u64, n: i32) {
    *mask |= 1 << n;
}

#[inline(always)]
fn is_bit_set(mask: u64, n: i32) -> bool {
    (mask & (1 << n)) != 0
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

