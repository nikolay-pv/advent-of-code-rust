use std::{borrow::BorrowMut, collections::VecDeque, i32::MAX};

const INPUT_TXT: &str = include_str!("input.txt");

fn main() {
    let input = read_input(INPUT_TXT);
    // println!("{:?}", &input);
    println!("Answer to first parts is {}", solve_first(&input));
    println!("Answer to second parts is {}", solve_second(&input));
}

fn solve_first(input: &Field) -> i32 {
    let rows = input.depths.len();
    let cols = input.depths.first().unwrap().len();

    let mut move_counts = vec![vec![MAX; cols]; rows];
    move_counts[input.start.0][input.start.1] = 0;
    let mut q = VecDeque::from([input.start]);

    while !q.is_empty() {
        let pos = q.pop_front().unwrap();
        if pos == input.end {
            break;
        }
        let current = move_counts[pos.0][pos.1] + 1;
        neighbors(pos, rows, cols).into_iter().for_each(|elem| {
            let n = move_counts[elem.0][elem.1].borrow_mut();
            if (input.depths[elem.0][elem.1] - input.depths[pos.0][pos.1]) <= 1 {
                if current < *n {
                    *n = current;
                    q.push_back(elem);
                }
            }
        });
    }
    move_counts[input.end.0][input.end.1]
}

fn solve_second(input: &Field) -> i32 {
    return input.depths.len() as i32;
}

fn neighbors(pos: (usize, usize), rows: usize, cols: usize) -> Vec<(usize, usize)> {
    const DIRECTIONS: [(i32, i32); 4] = [(-1,0), (1,0), (0,-1), (0,1)];
    let mut res = vec![];
    for (d_row, d_col) in DIRECTIONS {
        let new_row = pos.0 as i32 - d_row;
        let new_col = pos.1 as i32 - d_col;
        if 0 <= new_row && new_row < rows as i32 && 0 <= new_col && new_col < cols as i32 {
            res.push((new_row as usize, new_col as usize));
        }
    }
    res
}

#[derive(Debug, Clone)]
struct Field {
    depths: Vec<Vec<i32>>,
    start: (usize, usize),
    end: (usize, usize),
}

fn read_input(file_content: &str) -> Field {
    let mut start_pos = (0, 0);
    let mut end_pos = start_pos;
    let depths = file_content.lines().enumerate().map(|(row, line)|
        line.chars().enumerate().map(|(col, c)|
            if c == 'S' {
                start_pos = (row, col);
                0
            } else if c == 'E' {
                end_pos = (row, col);
                ('z' as i32) - ('a' as i32)
            } else {
                (c as i32) - ('a' as i32)
            }
        ).collect()
    ).collect();
    Field { depths, start: start_pos, end: end_pos }
}


#[cfg(test)]
mod tests {
    use crate::*;
    const TEST_INPUT_TXT: &str = include_str!("input_test.txt");

    #[test]
    fn part1() {
        let input = read_input(TEST_INPUT_TXT);
        assert_eq!(solve_first(&input), 31);
    }

    #[test]
    fn part2() {
        let input = read_input(TEST_INPUT_TXT);
        assert_eq!(solve_second(&input), 5);
    }
}

