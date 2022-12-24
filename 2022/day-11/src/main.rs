use std::collections::{LinkedList, HashMap};

const INPUT: fn() -> Vec<Monkey> = || vec![
    Monkey::new(LinkedList::from([71,86]), |x| x * 13, |x| if x % 19 == 0 { 6 } else { 7 }),
    Monkey::new(LinkedList::from([66, 50, 90, 53, 88, 85]), |x| x + 3, |x| if x % 2 == 0 { 5 } else { 4 }),
    Monkey::new(LinkedList::from([97, 54, 89, 62, 84, 80, 63]), |x| x + 6, |x| if x % 13 == 0 { 4 } else { 1 }),
    Monkey::new(LinkedList::from([82, 97, 56, 92]), |x| x + 2, |x| if x % 5 == 0 { 6 } else { 0 }),
    Monkey::new(LinkedList::from([50, 99, 67, 61, 86]), |x| x * x, |x| if x % 7 == 0 { 5 } else { 3 }),
    Monkey::new(LinkedList::from([61, 66, 72, 55, 64, 53, 72, 63]), |x| x + 4, |x| if x % 11 == 0 { 3 } else { 0 }),
    Monkey::new(LinkedList::from([59, 79, 63]), |x| x * 7, |x| if x % 17 == 0 { 2 } else { 7 }),
    Monkey::new(LinkedList::from([55]), |x| x + 7, |x| if x % 3 == 0 { 2 } else { 1 }),
];

fn main() {
    let input = INPUT();
    println!("Answer to first parts is {}", solve_first(input.clone()));
    println!("Answer to second parts is {}", solve_second(input));
}

#[derive(Debug, Clone)]
struct Monkey {
    items: LinkedList<usize>,
    op: fn(usize)->usize,
    test: fn(usize)->usize,
    exam_counter: usize,
}

impl Monkey {
    fn new(items: LinkedList<usize>, op: fn(usize)->usize, test: fn(usize)->usize) -> Self { Self { items, op, test, exam_counter: 0 } }

    fn compute_new_worry_level(&self, worry: usize) -> usize {
        (self.op)(worry) / 3
    }

    fn take_turn(&mut self) -> HashMap<usize, LinkedList<usize>> {
        let mut res = HashMap::new();
        for item in self.items.iter() {
            let new_level = self.compute_new_worry_level(*item);
            let new_monkey = (self.test)(new_level);
            res.entry(new_monkey)
                .and_modify(|items: &mut LinkedList<usize>| items.push_back(new_level))
                .or_insert(LinkedList::from([new_level]));
        }
        let examined = self.items.len();
        self.exam_counter += examined;
        self.items.clear();
        return res;
    }
}

fn solve_first(mut input: Vec<Monkey>) -> usize {
    for _round in 0..20 {
        for i in 0..input.len() {
            let monkey = input.get_mut(i).unwrap();
            let fallout = monkey.take_turn();
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

fn solve_second(mut input: Vec<Monkey>) -> usize {
    return input.len() as usize;
}


#[cfg(test)]
mod tests {
    use crate::*;
    static INPUT_TEST: fn() -> Vec<Monkey> = || vec![
        Monkey::new(LinkedList::from([79, 98]), |x| x * 19, |x| if x % 23 == 0 { 2 } else { 3 }),
        Monkey::new(LinkedList::from([54, 65, 75, 74]), |x| x + 6, |x| if x % 19 == 0 { 2 } else { 0 }),
        Monkey::new(LinkedList::from([79, 60, 97]), |x| x * x, |x| if x % 13 == 0 { 1 } else { 3 }),
        Monkey::new(LinkedList::from([74]), |x| x + 3, |x| if x % 17 == 0 { 0 } else { 1 }),
    ];

    #[test]
    fn part1() {
        assert_eq!(solve_first(INPUT_TEST()), 10605);
    }

    #[test]
    fn monkey_turn() {
        let mut monkey = Monkey::new(LinkedList::from([79, 98]), |x| x * 19, |x| if x % 23 == 0 { 2 } else { 3 });
        let res = monkey.take_turn();
        assert!(monkey.items.is_empty());
        let mut want = HashMap::new();
        want.insert(3, LinkedList::from([500, 620]));
        assert_eq!(res, want);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_second(INPUT_TEST()), 70);
    }
}

