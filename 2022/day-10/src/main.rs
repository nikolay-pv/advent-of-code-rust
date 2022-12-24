const INPUT_TXT: &str = include_str!("input.txt");

fn main() {
    let input = read_input(INPUT_TXT);
    println!("Answer to first parts is {}", solve_first(&input));
    println!("Answer to second parts is \n{}", solve_second(&input));
}

fn solve_first(input: &Vec<Option<i32>>) -> i32 {
    let increment = 40;
    let max_mark = 260;
    let mut x = 1i32;
    let mut mark = 20;
    let mut signal_strengths_sum = 0i32;
    for cycle in 1..(input.len() + 1) {
        if cycle == mark {
            assert_ne!(mark, max_mark);
            signal_strengths_sum += x * (cycle as i32);
            mark += increment;
        }
        if let Some(val) = input[cycle - 1] {
            x += val;
        }
    }
    return signal_strengths_sum;
}

fn solve_second(input: &Vec<Option<i32>>) -> String {
    let mut output = vec!['.'; 40*6];
    let mut x = 1i32;
    for cycle in 1..(input.len() + 1) {
        let pos = ((cycle - 1) % 40) as i32; // screen is 0-based
        if x - 1 <= pos && pos <= x + 1 {
            output[cycle - 1] = '#';
        }
        if let Some(val) = input[cycle - 1] {
            x += val;
        }
    }
    return output.as_slice().chunks(40).into_iter()
        .map(|x| x.into_iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
}

fn read_input(file_content: &str) -> Vec<Option<i32>> {
    file_content.lines().into_iter().flat_map(|line| 
        if line == "noop" {
            vec![None]
        } else {
            // addx is noop + addition (2 cycles)
            vec![None, Some(line.split(' ').last().unwrap().parse::<i32>().unwrap())]
        }
    ).collect()
}


#[cfg(test)]
mod tests {

    use crate::*;
    const TEST_INPUT_TXT: &str = include_str!("input_test.txt");
    const TEST_OUTPUT_TXT: &str = include_str!("output_part2.txt");

    #[test]
    fn part1() {
        let input = read_input(TEST_INPUT_TXT);
        assert_eq!(solve_first(&input), 13140);
    }

    #[test]
    fn part2() {
        let input = read_input(TEST_INPUT_TXT);
        let output = TEST_OUTPUT_TXT.to_string();
        assert_eq!(solve_second(&input), output);
    }
}

