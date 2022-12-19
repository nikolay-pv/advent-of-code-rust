use std::collections::HashSet;

const INPUT_TXT: &str = include_str!("input.txt");

fn main() {
    let input = read_input(INPUT_TXT);
    println!("Answer to first parts is {}", solve_first(&input));
    println!("Answer to second parts is {}", solve_second(&input));
}

// (x, y) == (right, up)
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point(i32, i32);

impl Point {
    fn zero() -> Point {
        Point(0,0)
    }
}

#[derive(Debug)]
struct Grid {
    trace: HashSet<Point>,
    head: Point,
    tail: Point,
}

impl Grid {
    fn new() -> Grid {
        Grid{
            trace: HashSet::from([Point::zero()]),
            head: Point::zero(),
            tail: Point::zero(),
        }
    }

    fn move_head(&mut self, mv: &Point) {
        self.head.0 += mv.0;
        self.head.1 += mv.1;
    }

    fn move_tail(&mut self) {
        let dx = self.head.0 - self.tail.0;
        let dy = self.head.1 - self.tail.1;
        if dx.abs() <= 1 && dy.abs() <= 1 {
            return;
        }
        self.tail.0 += dx.signum();
        self.tail.1 += dy.signum();
        self.trace.insert(self.tail);
    }
}

fn solve_first(input: &Vec<Point>) -> usize {
    println!("");
    let mut g = Grid::new();
    for mut mv in input.iter().copied() {
        while mv != Point::zero() {
            let d = Point(mv.0.signum(), mv.1.signum());
            g.move_head(&d);
            g.move_tail();
            mv.0 -= d.0;
            mv.1 -= d.1;
        }
    }
    return g.trace.len();
}

fn solve_second(input: &Vec<Point>) -> usize {
    return input.len();
}

fn to_coord_increment(line: &str) -> Point {
    let (left, right) = line.split_once(' ').unwrap();
    let count = right.parse::<i32>().unwrap();
    match left {
        "R" => Point(count, 0),
        "L" => Point(-count, 0),
        "U" => Point(0, count),
        "D" => Point(0, -count),
        _ => unreachable!(),
    }
}

fn read_input(file_content: &str) -> Vec<Point> {
    file_content.lines().into_iter().map(|line| 
        to_coord_increment(line)
    ).collect()
}


#[cfg(test)]
mod tests {
    use crate::*;
    const TEST_INPUT_TXT: &str = include_str!("input.txt");

    #[test]
    fn part1() {
        let input = read_input(TEST_INPUT_TXT);
        assert_eq!(solve_first(&input), 13);
    }

    #[test]
    fn part2() {
        let input = read_input(TEST_INPUT_TXT);
        assert_eq!(solve_second(&input), 70);
    }
}

