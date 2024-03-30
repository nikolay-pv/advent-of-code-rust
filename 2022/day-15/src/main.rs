use std::collections::HashSet;

// input is modified for the sake of parsing
// x1 y1 x2 y2, where x1,y1 is sensor and x2,y2 is beacon
const INPUT_TXT: &str = include_str!("input.txt");

fn main() {
    let input = read_input(INPUT_TXT);
    let y = 2000000;
    println!("Answer to first parts is {}", solve_first(&input, y));
    let limits = (0, 4000000);
    println!("Answer to second parts is {}", solve_second(&input, limits));
}

fn get_beacons_and_x_ranges(input: &Vec<SensorBeacon>, y: i64) -> (HashSet<(i64, i64)>, Vec<(i64,i64)>) {
    let mut beacons = HashSet::<(i64, i64)>::new();
    let mut x_ranges = Vec::<(i64, i64)>::new();
    for p in input {
        if p.beacon.1 == y {
            beacons.insert(p.beacon);
        }
        if p.is_in_range(y) {
            x_ranges.push(p.get_empty_range(y));
        }
    }
    let merged = merge(x_ranges);
    (beacons, merged)
}

fn solve_first(input: &Vec<SensorBeacon>, y: i64) -> i64 {
    let (beacons, x_ranges) = get_beacons_and_x_ranges(&input, y);
    assert_eq!(x_ranges.len(), 1);
    let range = x_ranges.first().unwrap();
    let beacons_in_range = beacons.into_iter()
        .filter(|n| &range.0 <= &n.0 && &n.0 <= &range.1)
        .count() as i64;
    return range.1 - range.0 + 1 - beacons_in_range;
}

fn solve_second(input: &Vec<SensorBeacon>, limits: (i64, i64)) -> i64 {
    for y in limits.0..=limits.1 {
        let (beacons, x_ranges) = get_beacons_and_x_ranges(&input, y);
        if x_ranges.len() == 1 {
            continue;
        }
        assert_eq!(x_ranges.len(), 2);
        let possible_range = (x_ranges.first().unwrap().1 + 1, x_ranges.last().unwrap().0 - 1);
        let beacons_x: HashSet<i64> = beacons.into_iter().map(|x| x.0).collect();
        let mut x = possible_range.0;
        while x <= possible_range.1 {
            if beacons_x.contains(&x) {
                x += 1;
                continue;
            }
            break;
        }
        let frequency = x * 4000000 + y;
        return frequency;
    }
    unreachable!();
}

fn merge(mut ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    assert!(!ranges.is_empty());
    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut range: (i64, i64) = *ranges.first().unwrap();
    let mut result = Vec::<(i64, i64)>::new();
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
    sensor: (i64, i64),
    beacon: (i64, i64),
    radius: i64
}

impl SensorBeacon {
    fn new(x_s: i64, y_s: i64, x_b: i64, y_b: i64) -> SensorBeacon {
        SensorBeacon{
            sensor: (x_s, y_s),
            beacon: (x_b, y_b),
            radius: (x_s - x_b).abs() + (y_s - y_b).abs(),
        }
    }

    fn from_str(line: &str) -> SensorBeacon {
        let coords: Vec<i64> = line.split_whitespace().map(|c| c.parse::<i64>().expect("Failed parsing int from input")).collect();
        SensorBeacon::new(coords[0], coords[1], coords[2], coords[3])
    }

    fn is_in_range(&self, y: i64) -> bool {
        (self.sensor.1 - self.radius) <= y && y <= (self.sensor.1 + self.radius)
    }

    fn get_empty_range(&self, y: i64) -> (i64, i64) {
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
        let limits = (0, 20);
        assert_eq!(solve_second(&input, limits), 56000011);
    }
}

