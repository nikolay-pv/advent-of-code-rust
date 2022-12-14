use std::vec;

const INPUT_TXT: &str = include_str!("input.txt");

fn main() {
    let input = read_input(INPUT_TXT);
    println!("Answer to first parts is {}", solve_first(&input));
    println!("Answer to second parts is {}", solve_second(&input));
}

#[derive(Debug, Clone)]
struct FSNode<'str> {
// FileSystemNode
    name: &'str str,
    size: usize,
    children: Option<Vec<FSNode<'str>>>,
}

impl<'str> FSNode<'str> {
    fn new_file(name: &'str str, size: usize) -> Self {
        FSNode { name, size, children: None }
    }

    fn new_directory(name: &'str str, size: usize, children: Vec<FSNode<'str>>) -> Self {
        FSNode { name, size, children: Some(children) }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn add_size(&mut self, d: usize) {
        self.size += d;
    }

    fn name(&self) -> &'str str {
        self.name
    }

    fn is_directory(&self) -> bool {
        self.children.is_some()
    }

    fn fd(&mut self, dir_name: &'str str) -> Option<&mut FSNode<'str>> {
        self.children.as_mut().and_then(|children| {
                children.into_iter()
                    .filter(|node| node.children.is_some())
                    .find(|node| node.name() == dir_name)
        })
    }

    fn add_node(&mut self, node: FSNode<'str>) {
        match &mut self.children {
            None => unreachable!(),
            Some(children) => {
                assert!(!children.into_iter().any(|x| x.name() == node.name()));
                children.push(node);
            },
        }
    }

    fn in_order_traverse<P>(&self, f: &mut P)
        where P: FnMut(&FSNode) -> () {
        f(self);
        if let Some(children) = &self.children {
            children.into_iter().for_each(|x| x.in_order_traverse(f));
        }
    }
}

fn ls<'str>(current: &mut FSNode<'str>, input: &Vec<Token<'str>>, idx: &mut usize) {
    if *idx == input.len() {
        return;
    }
    while *idx != input.len() {
        match &input[*idx] {
            Token::cd(..) | Token::ls => break,
            Token::dir(name) => {
                current.add_node(FSNode::new_directory(*name, 0, vec![]));
                *idx += 1;
            },
            Token::file(size, name) => {
                current.add_node(FSNode::new_file(*name, *size));
                current.add_size(*size);
                *idx += 1;
            },
        }
    }
}

fn parse_all<'str>(input: &Vec<Token<'str>>) -> FSNode<'str> {
    let mut root = FSNode::new_directory("/", 0, vec![]);
    let mut i = 1;
    parse(&mut root, input, &mut i);
    // println!("{:?}", &root);
    return root; 
}

fn parse<'str>(current: &mut FSNode<'str>, input: &Vec<Token<'str>>, i: &mut usize) {
    while *i != input.len() {
        match &input[*i] {
            Token::ls => { *i += 1; ls(current, input, i); },
            Token::cd(name) => {
                if *name == ".." {
                    *i += 1;
                    break;
                }
                if let Some(new_node) = current.fd(name) {
                    *i += 1;
                    parse(new_node, input, i);
                    let d = new_node.size();
                    current.add_size(d);
                }
            },
            Token::dir(..) | Token::file(..) => unreachable!(), // should be consumed in 'match ls'
        }
    }
}

fn solve_first(input: &Vec<Token>) -> usize {
    let node = parse_all(&input);
    let threshold = 100000;
    let mut sum: usize = 0;
    node.in_order_traverse(
        &mut |node| {
            if node.is_directory() && node.size() <= threshold {
                sum += node.size();
            }
        }
    );
    sum
}

fn solve_second(input: &Vec<Token>) -> usize {
    let node = parse_all(&input);
    let total_size = 70000000;
    let total_available = total_size - node.size(); // root
    let required_free_space = 30000000;
    let mut smallest: usize = required_free_space;
    node.in_order_traverse(
        &mut |node| {
            if node.is_directory() && node.size() + total_available >= required_free_space {
                smallest = smallest.min(node.size());
            }
        }
    );
    smallest
}

#[derive(Debug)]
enum Token<'str> {
    #[allow(non_camel_case_types)]
    ls,
    #[allow(non_camel_case_types)]
    cd(&'str str),
    #[allow(non_camel_case_types)]
    dir(&'str str),
    #[allow(non_camel_case_types)]
    file(usize, &'str str),
}

impl<'str> Token<'str> {
    fn new(line: &'str str) -> Token {
        match line {
            line if line.starts_with("$ cd") => Token::cd(line.split_at(5).1),
            line if line.starts_with("$ ls") => Token::ls,
            line if line.starts_with("dir ") => Token::dir(line.split_at(4).1),
            _ => {
                let (sz, name) = line.split_once(' ').unwrap();
                Token::file(sz.parse::<usize>().unwrap(), name)
            }
        }
    }
}

fn read_input(file_content: &str) -> Vec<Token> {
    file_content.lines().map(|x| Token::new(x)).collect()
}

#[cfg(test)]
mod tests {
    use crate::*;
    const TEST_INPUT_TXT: &str = include_str!("input_test.txt");

    #[test]
    fn part1() {
        let input = read_input(TEST_INPUT_TXT);
        assert_eq!(solve_first(&input), 95437);
    }

    #[test]
    fn part2() {
        let input = read_input(TEST_INPUT_TXT);
        assert_eq!(solve_second(&input), 24933642);
    }
}
