use std::collections::HashSet;

// input is modified for the sake of parsing
// x1 y1 x2 y2, where x1,y1 is sensor and x2,y2 is beacon
const INPUT_TXT: &str = include_str!("input.txt");

fn main() {
    let input = read_input(INPUT_TXT);
    let y = 2000000;
    println!("Answer to first parts is {}", solve_first(&input, y));
    println!("Answer to second parts is {}", solve_second(&input, y));
}

fn solve_first(input: &Vec<SensorBeacon>, y: i32) -> i32 {
    let mut beacons = HashSet::<(i32, i32)>::new();
    let mut x_ranges = Vec::<(i32, i32)>::new();
    for p in input {
        if p.beacon.1 == y {
            beacons.insert(p.beacon);
        }
        if p.is_in_range(y) {
            x_ranges.push(p.get_empty_range(y));
        }
    }
    let merged = merge(x_ranges);
    assert_eq!(merged.len(), 1);
    let range = merged.first().unwrap();
    let beacons_in_range = beacons.into_iter()
        .filter(|n| &range.0 <= &n.0 && &n.0 <= &range.1)
        .count() as i32;
    return range.1 - range.0 + 1 - beacons_in_range;
}

fn solve_second(input: &Vec<SensorBeacon>, y: i32) -> i32 {
    return y * input.len() as i32;
}

fn merge(mut ranges: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    assert!(!ranges.is_empty());
    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut range: (i32, i32) = *ranges.first().unwrap();
    let mut result = Vec::<(i32, i32)>::new();
    for r in ranges.iter().skip(1) {
        // range.0 <= r.0 => check if one can include the other
        if r.0 <= range.1 {
            range = (range.0, range.1.max(r.1));
        } else {
            // start new merging
            result.push(range);
            range = *r;
        }
    }
    result.push(range);
    result
}

struct SensorBeacon {
    sensor: (i32, i32),
    beacon: (i32, i32),
    radius: i32
}

impl SensorBeacon {
    fn new(x_s: i32, y_s: i32, x_b: i32, y_b: i32) -> SensorBeacon {
        SensorBeacon{
            sensor: (x_s, y_s),
            beacon: (x_b, y_b),
            radius: (x_s - x_b).abs() + (y_s - y_b).abs(),
        }
    }

    fn from_str(line: &str) -> SensorBeacon {
        let coords: Vec<i32> = line.split_whitespace().map(|c| c.parse::<i32>().expect("Failed parsing int from input")).collect();
        SensorBeacon::new(coords[0], coords[1], coords[2], coords[3])
    }

    fn is_in_range(&self, y: i32) -> bool {
        (self.sensor.1 - self.radius) <= y && y <= (self.sensor.1 + self.radius)
    }

    fn get_empty_range(&self, y: i32) -> (i32, i32) {
        let distance = (self.sensor.1 - y).abs();
        assert!(distance <= self.radius);
        let center = self.sensor.0;
        let offset = self.radius - distance;
        ((center - offset), (center + offset))
    }
}

fn read_input(file_content: &str) -> Vec<SensorBeacon> {
    file_content.lines().into_iter()
        .map(|line| 
            SensorBeacon::from_str(line)
    ).collect()
}


#[cfg(test)]
mod tests {
    use crate::*;
    const TEST_INPUT_TXT: &str = include_str!("input_test.txt");

    #[test]
    fn conut_empty() {
        let input = SensorBeacon::new(8, 7, 2, 10);
        {
            let s = input.get_empty_range(10);
            assert_eq!(s.0, 2);
            assert_eq!(s.1, 14);
        }
        {
            let s = input.get_empty_range(7);
            assert_eq!(s.0, -1);
            assert_eq!(s.1, 17);
        }
        {
            let s = input.get_empty_range(16);
            assert_eq!(s.0, 8);
            assert_eq!(s.1, 8);
        }
    }

    #[test]
    fn part1() {
        let input = read_input(TEST_INPUT_TXT);
        assert_eq!(solve_first(&input, 10), 26);
    }

    #[test]
    fn part2() {
        let input = read_input(TEST_INPUT_TXT);
        assert_eq!(solve_second(&input, 10), 70);
    }
}

