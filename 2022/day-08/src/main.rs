const INPUT_TXT: &str = include_str!("input.txt");

fn main() {
    let input = read_input(INPUT_TXT);
    println!("Answer to first parts is {}", solve_first(&input));
    println!("Answer to second parts is {}", solve_second(&input));
}

#[derive(Debug)]
struct ForestMap {
    forest: Vec<i8>,
    size: usize
}

impl ForestMap {
    fn new_visibility_map(forest_map: &ForestMap) -> ForestMap {
        let size = forest_map.size;
        let mut vm = ForestMap{forest: vec![0; size * size], size};
        let last_line = (size - 1) * size;
        for i in 0..size {
            vm.forest[i] = 1;
            vm.forest[last_line + i] = 1;
            vm.forest[i * size] = 1;
            vm.forest[(i + 1) * size - 1] = 1;
        }
        return vm;
    }

    #[allow(dead_code)]
    fn print(&self) {
        for i in 0..self.size {
            for j in 0..self.size {
                print!("{}", self.forest[i*self.size + j]);
            }
            println!("")
        }
    }
}

fn build_visibility_map(input: &ForestMap) -> ForestMap {
    let mut vm = ForestMap::new_visibility_map(input);
    let mut check_max = |max_val: &mut i8, idx: usize| {
        if input.forest[idx] > *max_val {
            *max_val = input.forest[idx];
            vm.forest[idx] = 1;
        }
    };
    for i in 1..(vm.size - 1) {
        let mut top_max: i8 = input.forest[i];
        let mut left_max: i8 = input.forest[i*input.size];
        let mut right_max: i8 = input.forest[(i + 1)*input.size - 1];
        let mut bottom_max: i8 = input.forest[(input.size-1)*input.size + i];
        for j in 1..(vm.size - 1) {
            check_max(&mut left_max, i*vm.size + j);
            check_max(&mut right_max, (i + 1)*vm.size - 1 - j);
            check_max(&mut top_max, j*vm.size + i);
            check_max(&mut bottom_max, (vm.size - j)*vm.size + i);
        }
    }
    // vm.print();
    return vm;
}

fn solve_first(input: &ForestMap) -> usize {
    let vm = build_visibility_map(input);
    return vm.forest.into_iter().filter(|&x| x != 0).count();
}

fn count_trees<R: Iterator>(input: &ForestMap, max: i8, range: R) -> i32 
where R: Iterator<Item = usize> {
    let mut counter = 0;
    for k in range {
        counter += 1;
        if input.forest[k] >= max {
            break;
        }
    }
    return counter;
}

fn solve_second(input: &ForestMap) -> usize {
    let vm = build_visibility_map(input);
    let mut max_score = 0; 
    for i in 1..(vm.size - 1) {
        let row_start = i * vm.size;
        let row_end = row_start + vm.size;
        for j in 1..(vm.size - 1) {
            let idx = row_start + j;
            if vm.forest[idx] == 0 {
                continue;
            }
            let col_start = j;
            let col_end = vm.size*(vm.size - 1) + j;
            let max = input.forest[idx];
            let mut score = count_trees(&input, max, (row_start..idx).rev());
            score *= count_trees(&input, max, (idx+1)..row_end);
            score *= count_trees(&input, max, (idx..=col_end).step_by(vm.size).skip(1));
            score *= count_trees(&input, max, (col_start..idx).step_by(vm.size).rev());
            max_score = max_score.max(score);
        }
    }
    return max_score as usize;
}

fn read_input(file_content: &str) -> ForestMap {
    let size = file_content.lines().count();
    assert_eq!(size, file_content.len() / size);
    ForestMap{
        forest: file_content.lines().into_iter().flat_map(
            |line| line.chars().map(|y| y as i8 - '0' as i8))
            .collect(),
        size,
    }
}


#[cfg(test)]
mod tests {
    use crate::*;
    const TEST_INPUT_TXT: &str = include_str!("input_test.txt");

    #[test]
    fn input() {
        let input = read_input(TEST_INPUT_TXT);
        let want: Vec<i8> = vec![3,0,3,7,3,2,5,5,1,2,6,5,3,3,2,3,3,5,4,9,3,5,3,9,0];
        assert_eq!(input.forest, want);
        assert_eq!(input.size, 5);
    }

    #[test]
    fn part1() {
        let input = read_input(TEST_INPUT_TXT);
        assert_eq!(solve_first(&input), 21);
    }

    #[test]
    fn part2() {
        let input = read_input(TEST_INPUT_TXT);
        assert_eq!(solve_second(&input), 8);
    }
}
