use std::collections::HashMap;

const STRESS_REDUCE_PT1: fn(u64) -> u64 = |x| x / 3;
const INPUT: fn() -> Vec<Monkey> = || vec![
    Monkey::new(Vec::from([71,86]), |x| x * 13, |x| if x % 19 == 0 { 6 } else { 7 }),
    Monkey::new(Vec::from([66, 50, 90, 53, 88, 85]), |x| x + 3, |x| if x % 2 == 0 { 5 } else { 4 }),
    Monkey::new(Vec::from([97, 54, 89, 62, 84, 80, 63]), |x| x + 6, |x| if x % 13 == 0 { 4 } else { 1 }),
    Monkey::new(Vec::from([82, 97, 56, 92]), |x| x + 2, |x| if x % 5 == 0 { 6 } else { 0 }),
    Monkey::new(Vec::from([50, 99, 67, 61, 86]), |x| x * x, |x| if x % 7 == 0 { 5 } else { 3 }),
    Monkey::new(Vec::from([61, 66, 72, 55, 64, 53, 72, 63]), |x| x + 4, |x| if x % 11 == 0 { 3 } else { 0 }),
    Monkey::new(Vec::from([59, 79, 63]), |x| x * 7, |x| if x % 17 == 0 { 2 } else { 7 }),
    Monkey::new(Vec::from([55]), |x| x + 7, |x| if x % 3 == 0 { 2 } else { 1 }),
];

fn main() {
    let input = INPUT();
    println!("Answer to first parts is {}", solve_first(input.clone()));
    println!("Answer to second parts is {}", solve_second(input));
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    op: fn(u64) -> u64,
    test: fn(u64) -> usize,
    exam_counter: usize,
}

impl Monkey {
    fn new(items: Vec<u64>, op: fn(u64)->u64, test: fn(u64)->usize) -> Self { Self { items, op, test, exam_counter: 0 } }

    fn take_turn(&mut self, stress_mitigator: fn(u64) -> u64) -> HashMap<usize, Vec<u64>> {
        let mut res = HashMap::new();
        for item in self.items.iter() {
            let new_level = stress_mitigator((self.op)(*item));
            let new_monkey = (self.test)(new_level);
            res.entry(new_monkey)
                .and_modify(|items: &mut Vec<u64>| items.push(new_level))
                .or_insert(Vec::from([new_level]));
        }
        let examined = self.items.len();
        self.exam_counter += examined;
        self.items.clear();
        return res;
    }
}

fn run_exchange(mut input: Vec<Monkey>, rounds: usize, stress_mitigator: fn(u64) -> u64) -> usize {
    for _round in 0..rounds {
        for i in 0..input.len() {
            let fallout = input[i].take_turn(stress_mitigator);
            for (k, mut v) in fallout {
                input[k].items.append(&mut v);
            }
        }
    }
    let mut counters = input.iter().map(|x| x.exam_counter).collect::<Vec<usize>>();
    counters.sort();
    counters.reverse();
    return counters[0] * counters[1];
}

fn solve_first(input: Vec<Monkey>) -> usize {
    return run_exchange(input, 20, STRESS_REDUCE_PT1);
}

fn stress_reducer(x: u64) -> u64 {
    const DIVISOR: u64 = 2*3*5*7*11*13*17*19;
    x % DIVISOR
}

fn solve_second(input: Vec<Monkey>) -> usize {
    return run_exchange(input, 10000, stress_reducer);
}


#[cfg(test)]
mod tests {
    use crate::*;
    static INPUT_TEST: fn() -> Vec<Monkey> = || vec![
        Monkey::new(Vec::from([79, 98]), |x| x * 19, |x| if x % 23 == 0 { 2 } else { 3 }),
        Monkey::new(Vec::from([54, 65, 75, 74]), |x| x + 6, |x| if x % 19 == 0 { 2 } else { 0 }),
        Monkey::new(Vec::from([79, 60, 97]), |x| x * x, |x| if x % 13 == 0 { 1 } else { 3 }),
        Monkey::new(Vec::from([74]), |x| x + 3, |x| if x % 17 == 0 { 0 } else { 1 }),
    ];

    #[test]
    fn part1() {
        assert_eq!(solve_first(INPUT_TEST()), 10605);
    }

    #[test]
    fn monkey_turn() {
        let mut monkey = Monkey::new(Vec::from([79, 98]), |x| x * 19, |x| if x % 23 == 0 { 2 } else { 3 });
        let res = monkey.take_turn(STRESS_REDUCE_PT1);
        assert!(monkey.items.is_empty());
        let mut want = HashMap::new();
        want.insert(3, Vec::from([500, 620]));
        assert_eq!(res, want);
        assert_eq!(monkey.exam_counter, 2);
    }

    fn test(x: u64) -> u64 {
        const DIVISOR: u64 = 13*17*19*23;
        x % DIVISOR
    }

    #[test]
    fn part2() {
        // custom reducer for test data
        assert_eq!(run_exchange(INPUT_TEST(), 10000, test), 2713310158);
    }
}

