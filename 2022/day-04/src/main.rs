const INPUT_TXT: &str = include_str!("input.txt");

fn main() {
    let input = read_input(INPUT_TXT);
    println!("Answer to first parts is {}", solve_first(&input));
    println!("Answer to second parts is {}", solve_second(&input));
}

#[derive(Clone, Copy)]
struct Interval(i32, i32);

#[derive(Clone, Copy)]
struct IntervalPair(Interval, Interval);

fn is_one_fully_overlaps_other(first: &Interval, second: &Interval) -> bool {
    (first.0 <= second.0 && second.1 <= first.1)
        || (second.0 <= first.0 && first.1 <= second.1)
}

fn solve_first(input: &Vec<IntervalPair>) -> i32 {
    input.iter()
        .filter(|IntervalPair(first, second)| is_one_fully_overlaps_other(first, second))
        .count().try_into().unwrap()
}

fn is_one_partially_overlaps_other(first: &Interval, second: &Interval) -> bool {
    let contains = |x, interval: &Interval| interval.0 <= x && x <= interval.1;
    contains(first.0, second)
        || contains(first.1, second)
        || contains(second.0, first)
        || contains(second.1, first)
}

fn solve_second(input: &Vec<IntervalPair>) -> i32 {
    input.iter()
        .filter(|IntervalPair(first, second)| is_one_partially_overlaps_other(first, second))
        .count().try_into().unwrap()
}

// converts string formatted as "start-finish" into an interval pair
fn make_interval(s: &str) -> Interval {
    let ints: Vec<i32> = s.split('-').map(|x| x.parse::<i32>().unwrap()).collect();
    assert!(ints.len() == 2);
    assert!(ints[0] <= ints[1]);
    Interval(ints[0], ints[1])
}

fn read_input(file_content: &str) -> Vec<IntervalPair> {
    file_content.lines().into_iter().map(|line| {
        let parts: Vec<Interval> = line.split(',').map(|x| make_interval(x)).collect();
        assert!(parts.len() == 2);
        IntervalPair(parts[0], parts[1])
    }
    ).collect()
}


#[cfg(test)]
mod tests {
    use crate::*;
    const TEST_INPUT_TXT: &str = include_str!("input_test.txt");

    #[test]
    fn fully_contains_other() {
        // not intersecting
        let mut first = Interval(0,1);
        let mut second = Interval(2,3);
        assert!(!is_one_fully_overlaps_other(&first, &second));
        assert!(!is_one_fully_overlaps_other(&second, &first));
        // intersecting
        // one is unit interval
        first = Interval(1,1);
        second = Interval(1,2);
        assert!(is_one_fully_overlaps_other(&first, &second));
        assert!(is_one_fully_overlaps_other(&second, &first));
        // one is unit interval
        first = Interval(2,2);
        second = Interval(1,2);
        assert!(is_one_fully_overlaps_other(&first, &second));
        assert!(is_one_fully_overlaps_other(&second, &first));
        // fully embedded
        first = Interval(1,2);
        second = Interval(0,3);
        assert!(is_one_fully_overlaps_other(&first, &second));
        assert!(is_one_fully_overlaps_other(&second, &first));
        // touching
        first = Interval(1,3);
        second = Interval(3,6);
        assert!(is_one_partially_overlaps_other(&first, &second));
        assert!(is_one_partially_overlaps_other(&second, &first));
        // touching
        first = Interval(6,9);
        second = Interval(3,6);
        assert!(is_one_partially_overlaps_other(&first, &second));
        assert!(is_one_partially_overlaps_other(&second, &first));
    }

    #[test]
    fn part1() {
        let input = read_input(TEST_INPUT_TXT);
        assert_eq!(solve_first(&input), 2);
    }

    #[test]
    fn part1_wrong_submissions() {
        let input = read_input(INPUT_TXT);
        assert!(solve_first(&input) > 476);
    }

    #[test]
    fn partially_contains_other() {
        // not intersecting 
        let mut first = Interval(0,1);
        let mut second = Interval(2,3);
        assert!(!is_one_partially_overlaps_other(&first, &second));
        assert!(!is_one_partially_overlaps_other(&second, &first));
        // intersecting
        // one is unit interval
        first = Interval(1,1);
        second = Interval(1,2);
        assert!(is_one_partially_overlaps_other(&first, &second));
        assert!(is_one_partially_overlaps_other(&second, &first));
        // one is unit interval
        first = Interval(2,2);
        second = Interval(1,2);
        assert!(is_one_partially_overlaps_other(&first, &second));
        assert!(is_one_partially_overlaps_other(&second, &first));
        // fully embedded
        first = Interval(1,2);
        second = Interval(0,3);
        assert!(is_one_partially_overlaps_other(&first, &second));
        assert!(is_one_partially_overlaps_other(&second, &first));
        // touching
        first = Interval(1,3);
        second = Interval(3,6);
        assert!(is_one_partially_overlaps_other(&first, &second));
        assert!(is_one_partially_overlaps_other(&second, &first));
        // touching
        first = Interval(6,9);
        second = Interval(3,6);
        assert!(is_one_partially_overlaps_other(&first, &second));
        assert!(is_one_partially_overlaps_other(&second, &first));
    }

    #[test]
    fn part2() {
        let input = read_input(TEST_INPUT_TXT);
        assert_eq!(solve_second(&input), 4);
    }
}

