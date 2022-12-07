use regex::Regex;

const INPUT_TXT: &str = include_str!("input.txt");

fn main() {
    let input = read_input(INPUT_TXT);
    println!("Answer to first parts is {}", solve_first(input.clone()));
    println!("Answer to second parts is {}", solve_second(input));
}

// alias to handle array of stacks
type Stacks = Vec<Vec<char>>;

// stores instruction in form of (count, from, to)
#[derive(Clone, Copy)]
struct Instruction(i32, usize, usize);

impl Instruction {
    fn make(line: &str) -> Option<Instruction> {
        let re: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        let parameters: Vec<i32> = re.captures(line)?
            .iter().skip(1).map(|x| x.unwrap().as_str().parse::<i32>().unwrap())
            .collect();
        if parameters.len() != 3 {
            None
        } else {
            Some(Instruction(parameters[0], parameters[1] as usize, parameters[2] as usize))
        }
    }
}

// tuple alias
type Input = (Stacks, Vec<Instruction>);

fn move_reversed(s: &mut Stacks, action: Instruction) {
    let Instruction(mut count, from, to) = action;
    while count != 0 {
        let c = s[from].pop().unwrap();
        s[to].push(c);
        count -= 1;
    }
}

fn solve_first(input: Input) -> String {
    let (mut stacks, instructions) = input;
    for action in instructions {
        move_reversed(&mut stacks, action);
    }
    stacks.iter().skip(1).map(|x| x.last().unwrap()).collect()
}

fn move_ordered(s: &mut Stacks, action: Instruction) {
    let Instruction(mut count, from, to) = action;
    let pos = s[to].len();
    while count != 0 {
        let c = s[from].pop().unwrap();
        s[to].insert(pos, c);
        count -= 1;
    }
}

fn solve_second(input: Input) -> String {
    let (mut stacks, instructions) = input;
    for action in instructions {
        move_ordered(&mut stacks, action);
    }
    stacks.iter().skip(1).map(|x| x.last().unwrap()).collect()
}

fn read_input(file_content: &str) -> Input {
    let mut lines = file_content.lines();
    let dwg = lines.by_ref().take_while(|&x| !x.is_empty()).collect::<Vec<&str>>();
    let sz = dwg[0].len();
    let dwg_width = (sz + 1) / 4 + 1;
    let mut s: Stacks = vec![vec![]; dwg_width];
    // skip line with numbers
    for &line in dwg.iter().rev().skip(1) {
        for (j, c) in line.chars().enumerate().skip(1).step_by(4) {
            if c != ' ' {
                s[j / 4 + 1].push(c);
            }
        }
    }
    let instructions = lines.map(|x| { 
        Instruction::make(x).unwrap()}).collect();
    return (s, instructions)
}


#[cfg(test)]
mod tests {
    use crate::*;
    const TEST_INPUT_TXT: &str = include_str!("input_test.txt");

    #[test]
    fn part1() {
        let input = read_input(TEST_INPUT_TXT);
        assert_eq!(solve_first(input), "CMZ");
    }

    #[test]
    fn part2() {
        let input = read_input(TEST_INPUT_TXT);
        assert_eq!(solve_second(input), "MCD");
    }
}

