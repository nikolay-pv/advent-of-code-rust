const INPUT_TXT: &str = include_str!("input.txt");

fn main() {
    let input = read_input(INPUT_TXT);
    println!("Answer to first parts is {}", solve_first(&input));
    println!("Answer to second parts is {}", solve_second(&input));
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Hand {
    Rock = 1, Paper = 2, Scissors = 3
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum GameResult {
    Lose = 0, Draw = 3, Win = 6
}

impl Hand {
    // includes the conditions for the first part of the task
    fn new(c: char) -> Hand {
        match c {
            'A'| 'X' => Hand::Rock,
            'B'| 'Y' => Hand::Paper,
            'C'| 'Z' => Hand::Scissors,
            _ => panic!("can't create game"),
        }
    }
    /*
    returns if result of the game for self
    */
    fn play_against(self: &Hand, other: Hand) -> GameResult {
        use Hand::*;
        use GameResult::*;
        if *self == other {
            return Draw;
        }
        match self {
            &Rock => if other == Scissors { Win } else { Lose },
            &Paper => if other == Rock { Win } else { Lose },
            &Scissors => if other == Paper { Win } else { Lose },
        }
    }
}

fn solve_first(input: &Vec<&str>) -> i32 {
    // hands
    let mut score = 0;
    for line in input.iter() {
        let opponent = Hand::new(line.chars().nth(0).unwrap());
        let player = Hand::new(line.chars().nth(2).unwrap());
        let res = player.play_against(opponent);
        score += (res as i32) + (player as i32);
    }
    return score;
}

impl GameResult {
    // includes the conditions for the first part of the task
    fn new(c: char) -> GameResult {
        match c {
            'X' => GameResult::Lose,
            'Y' => GameResult::Draw,
            'Z' => GameResult::Win,
            _ => panic!("can't create GameReuslt"),
        }
    }
}

impl Hand {
    // returns hand self would win
    fn wins(self: &Hand) -> Hand {
        use Hand::*;
        match self {
            &Rock => Scissors,
            &Paper => Rock,
            &Scissors => Paper,
        }
    }

    // returns hand self would lose to
    fn loses(self: &Hand) -> Hand {
        use Hand::*;
        match self {
            &Rock => Paper,
            &Paper => Scissors,
            &Scissors => Rock,
        }
    }
}

fn pick_hand(opponent: Hand, result: GameResult) -> Hand {
    match result {
        GameResult::Lose => opponent.wins(),
        GameResult::Draw => opponent,
        GameResult::Win => opponent.loses(),
    }
}


fn solve_second(input: &Vec<&str>) -> i32 {
    let mut score = 0;
    for line in input.iter() {
        let opponent = Hand::new(line.chars().nth(0).unwrap());
        let res = GameResult::new(line.chars().nth(2).unwrap());
        let player = pick_hand(opponent, res);
        score += (res as i32) + (player as i32);
    }
    return score;
}

fn read_input(file_content: &str) -> Vec<&str> {
    file_content.lines().into_iter().collect()
}


#[cfg(test)]
mod tests {
    use crate::*;
    const TEST_INPUT_TXT: &str = include_str!("input_test.txt");

    #[test]
    fn part1() {
        let input = read_input(TEST_INPUT_TXT);
        assert_eq!(solve_first(&input), 15);
    }

    #[test]
    fn part1_wrong_submissions() {
        let input = read_input(INPUT_TXT);
        assert!(solve_first(&input) < 12958);
    }

    #[test]
    fn part2() {
        let input = read_input(TEST_INPUT_TXT);
        assert_eq!(solve_second(&input), 12);
    }
}

