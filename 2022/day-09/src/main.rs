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
    nodes: Vec<Point>,
}

impl Grid {
    fn new(size: usize) -> Grid {
        Grid{
            trace: HashSet::from([Point::zero()]),
            nodes: vec![Point::zero(); size],
        }
    }

    fn head(&mut self) -> &mut Point {
        self.nodes.first_mut().unwrap()
    }

    fn tail(&mut self) -> &mut Point {
        self.nodes.last_mut().unwrap()
    }

    fn move_head(&mut self, mut mv: Point) {
        while mv != Point::zero() {
            let d = Point(mv.0.signum(), mv.1.signum());
            self.head().0 += d.0;
            self.head().1 += d.1;
            self.move_tail();
            mv.0 -= d.0;
            mv.1 -= d.1;
        }
    }

    fn move_tail(&mut self) {
        for i in 1..self.nodes.len() {
            let dx = self.nodes[i-1].0 - self.nodes[i].0;
            let dy = self.nodes[i-1].1 - self.nodes[i].1;
            if dx.abs() <= 1 && dy.abs() <= 1 {
                continue;
            }
            self.nodes[i].0 += dx.signum();
            self.nodes[i].1 += dy.signum();
        }
        let tail = *self.tail();
        self.trace.insert(tail);
    }
}

fn solve_first(input: &Vec<Point>) -> usize {
    let mut g = Grid::new(2);
    for mv in input.iter() {
        g.move_head(*mv);
    }
    return g.trace.len();
}

fn solve_second(input: &Vec<Point>) -> usize {
    let mut g = Grid::new(10);
    for mv in input.iter() {
        g.move_head(*mv);
    }
    return g.trace.len();
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
    const INPUT_TXT: &str = include_str!("input.txt");
    const TEST_INPUT_TXT: &str = include_str!("input_test.txt");
    const TEST_INPUT_PART2_TXT: &str = include_str!("input_test_part2.txt");

    #[test]
    fn movement() {
        let mut g = Grid::new(2);
        let mv = Point(4, 0);
        g.move_head(mv);
        assert_eq!(*g.head(), Point(4,0));
        assert_eq!(*g.tail(), Point(3,0));
    }

    #[test]
    fn part1() {
        let input = read_input(TEST_INPUT_TXT);
        assert_eq!(solve_first(&input), 13);
        let input = read_input(INPUT_TXT);
        assert_eq!(solve_first(&input), 6011);
    }

    #[test]
    fn part2() {
        let input = read_input(TEST_INPUT_PART2_TXT);
        assert_eq!(solve_second(&input), 36);
    }
}

