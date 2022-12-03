
const INPUT_TXT: &str = include_str!("input.txt");

fn main() {
    let input = read_input(INPUT_TXT);
    println!("Answer to first parts is {}", solve_first(&input));
    println!("Answer to second parts is {}", solve_second(&input));
}

fn solve_first(input: &Vec<i32>) -> i32 {
    let mut max_calories = i32::MIN;
    let mut sub_sum = 0;
    for food in input {
        if *food == -1 {
            max_calories = max_calories.max(sub_sum);
            sub_sum = 0;
        } else {
            sub_sum += *food;
        }
    }
    return max_calories.max(sub_sum);
}

fn solve_second(input: &Vec<i32>) -> i32 {
    let mut v = vec![];
    let mut sub_sum = 0;
    for food in input {
        if *food == -1 {
            v.push(sub_sum);
            sub_sum = 0;
        } else {
            sub_sum += *food;
        }
    }
    if sub_sum != 0 {
        v.push(sub_sum);
    }
    // no partial sort in std
    // v.partial_sort(3);
    v.sort_by(|a,b| a.cmp(b).reverse());
    assert!(v.len() > 2);
    return v[0..3].iter().sum();
}

fn read_input(file_content: &str) -> Vec<i32> {
    file_content.lines().into_iter().map(|line| 
        if line.is_empty() {
            -1
        } else {
            line.parse::<i32>().unwrap()
        }
    ).collect()
}


#[cfg(test)]
mod tests {
    use crate::*;
    const TEST_INPUT_TXT: &str = include_str!("input_test.txt");

    #[test]
    fn part1() {
        let input = read_input(TEST_INPUT_TXT);
        assert_eq!(solve_first(&input), 24000);
    }

    #[test]
    fn part2() {
        let input = read_input(TEST_INPUT_TXT);
        assert_eq!(solve_second(&input), 45000);
    }
}
