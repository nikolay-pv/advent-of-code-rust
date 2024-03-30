use std::{mem::swap, usize};

const INPUT_TXT: &str = include_str!("input.txt");

fn main() {
    let mut input = read_input(INPUT_TXT, false);
    // input.print();
    println!("Answer to first parts is {}", solve_first(&mut input));
    // input.print();
    input = read_input(INPUT_TXT, true);
    println!("Answer to second parts is {}", solve_second(&mut input));
}

fn solve_first(input: &mut Map) -> i32 {
    let mut count = 0;
    loop {
        let mut sand = input.source;
        loop {
            match input.check_moves(sand) {
                MoveResult::FreeSpot(dst) => sand = dst,
                MoveResult::FellOff => return count,
                MoveResult::Settled => {
                    input.fill(sand);
                    count += 1;
                    if sand == input.source {
                        return count;
                    }
                    break;
                },
            }
        }
    }
}

fn solve_second(input: &mut Map) -> i32 {
    solve_first(input)
}

#[derive(Debug, Clone)]
struct Map {
    tiles: Vec<char>,
    shape: (usize, usize),
    // offset: (usize, usize),
    source: (usize, usize),
}

enum Direction {
    Down, Left, Right
}

enum MoveResult {
    FreeSpot((usize, usize)), FellOff, Settled
}

impl Map {
    fn new(rocks: Vec<(usize, usize)>, x_lim: (usize, usize), y_lim: (usize, usize)) -> Map {
        let shape = ((x_lim.1 - x_lim.0 + 1) as usize, (y_lim.1 - y_lim.0 + 1) as usize);
        let mut tiles = vec!['.'; shape.0 * shape.1];
        // mark the rocks accounting for the shift into "view"
        for (x, y) in rocks {
            tiles[(y - y_lim.0) * shape.0 + (x - x_lim.0)] = '#';
        }
        let source = (500 - x_lim.0, 0 - y_lim.0);
        Map {
            tiles,
            shape,
            // offset: (x_lim.0 as usize, y_lim.0 as usize),
            source,
        }
    }

    fn at(&self, pos: (usize, usize)) -> char {
        self.tiles[pos.1 * self.shape.0 + pos.0]
    }

    fn fill(&mut self, pos: (usize, usize)) {
        self.tiles[pos.1 * self.shape.0 + pos.0] = 'o';
    }

    fn check_moves(&mut self, pos: (usize, usize)) -> MoveResult {
        for d in [Direction::Down, Direction::Left, Direction::Right] {
            let dst = match d {
                Direction::Down => (pos.0, pos.1 + 1),
                Direction::Left => {
                    if pos.0 == 0 {
                        return MoveResult::FellOff;
                    }
                    (pos.0 - 1, pos.1 + 1)
                },
                Direction::Right => (pos.0 + 1, pos.1 + 1),
            };
            if dst.0 < self.shape.0 && dst.1 < self.shape.1 {
                if self.at(dst) == '.' {
                    return MoveResult::FreeSpot(dst);
                } 
            } else {
                return MoveResult::FellOff;
            }
        }
        MoveResult::Settled
    }

    fn print(&self) {
        for r in 0..self.shape.1 {
            for c in 0..self.shape.0 {
                print!("{}", self.at((c, r)));
            }
            print!("\n");
        }
    }
}

fn read_input(file_content: &str, add_floor: bool) -> Map {
    let mut rocks = Vec::<(usize, usize)>::new();
    // take into account 500,0 for the sand source
    let mut x_lim = (500, 500);
    let mut y_lim = (0, 0);

    for l in file_content.lines() {
        let coords: Vec<(usize, usize)> = l.split(" -> ").map(|p| {
            let idx = p.find(',').unwrap();
            // parse the coordinates 
            let x = p[..idx].parse::<usize>().unwrap();
            let y = p[(idx+1)..].parse::<usize>().unwrap();
            // find boundaries of the map
            x_lim = (x_lim.0.min(x), x_lim.1.max(x));
            y_lim = (y_lim.0.min(y), y_lim.1.max(y));
            (x, y)
        }).collect();
        // generate all positions for the rocks
        for l in 1..coords.len() {
            let mut left = coords[l - 1];
            let mut right = coords[l];
            if left.0 > right.0 {
                swap(&mut left.0, &mut right.0);
            }
            if left.1 > right.1 {
                swap(&mut left.1, &mut right.1);
            }
            for i in left.0..=right.0 {
                for j in left.1..=right.1 {
                    rocks.push((i, j));
                }
            }
        }
    }
    if add_floor {
        // part2 add y_max+2 floor
        // the size of it will be at most 2 heights of the "triangle"
        let y_floor = y_lim.1 + 2;
        let height = y_floor;
        // from the source
        for i in 500-height..=500+height {
            rocks.push((i, y_floor));
        }
        x_lim = (x_lim.0.min(500-height), x_lim.1.max(500+height));
        y_lim = (y_lim.0, y_floor);
    }
    Map::new(rocks, x_lim, y_lim)
}


#[cfg(test)]
mod tests {
    use crate::*;
    const TEST_INPUT_TXT: &str = include_str!("input_test.txt");

    #[test]
    fn part1() {
        let mut input = read_input(TEST_INPUT_TXT, false);
        assert_eq!(solve_first(&mut input), 24);
    }

    #[test]
    fn part2() {
        let mut input = read_input(TEST_INPUT_TXT, true);
        assert_eq!(solve_second(&mut input), 93);
    }
}

