use std::vec;

const INPUT_TXT: &str = include_str!("input.txt");

fn main() {
    let input = read_input(INPUT_TXT);
    println!("Answer to first parts is {}", solve_first(&input));
    println!("Answer to second parts is {}", solve_second(&input));
}

#[derive(Debug, Clone)]
enum FSNode<'str> {
// FileSystemNode
    Dir(&'str str, usize, Vec<FSNode<'str>>),
    File(&'str str, usize),
}

impl<'str> FSNode<'str> {
    fn size(&self) -> usize {
        match self {
            FSNode::Dir(.., size, _children) => *size,
            FSNode::File(.., size) => *size,
        }
    }

    fn add_size(&mut self, d: usize) {
        match self {
            FSNode::Dir(.., size, _children) => *size += d,
            FSNode::File(.., size) => *size += d,
        }
    }

    fn name(&self) -> &'str str {
        match self {
            FSNode::Dir(name, ..) => name,
            FSNode::File(name, ..) => name,
        }
    }

    fn fd(&mut self, dir_name: &'str str) -> Option<&mut FSNode<'str>> {
        match self {
            FSNode::File(..) => None,
            FSNode::Dir(.., children) => 
                children.into_iter()
                .filter_map(|x| match x {
                    FSNode::Dir(..) => Some(x),
                    FSNode::File(..) => None, })
                .find(|x| x.name() == dir_name),
        }
    }

    fn add_node(&mut self, node: FSNode<'str>) {
        match self {
            FSNode::File(..) => unreachable!(),
            FSNode::Dir(.., children) => {
                assert!(!children.into_iter().any(|x| x.name() == node.name()));
                children.push(node);
            },
        }
    }

    fn in_order_traverse<P>(&self, f: &mut P)
        where P: FnMut(&FSNode) -> () {
        f(self);
        match self {
            FSNode::Dir(.., children) => children.into_iter().for_each(|x| x.in_order_traverse(f)),
            _ => (),
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
                current.add_node(FSNode::Dir(*name, 0, vec![]));
                *idx += 1;
            },
            Token::file(size, name) => {
                current.add_node(FSNode::File(*name, *size));
                current.add_size(*size);
                *idx += 1;
            },
        }
    }
}

fn parse_all<'str>(input: &Vec<Token<'str>>) -> FSNode<'str> {
    let mut root = FSNode::Dir("/", 0, vec![]);
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
        &mut |n| {
            match n {
                FSNode::Dir(..) if n.size() <= threshold => sum += n.size(),
                _ => (),
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
        &mut |n| {
            match n {
                FSNode::Dir(..) if n.size() + total_available >= required_free_space => smallest = smallest.min(n.size()),
                _ => (),
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
        if line.starts_with("$ cd") {
            Token::cd(line.split_at(5).1)
        } else if line.starts_with("$ ls") {
            Token::ls
        } else if line.starts_with("dir ") {
            Token::dir(line.split_at(4).1)
        } else {
            let (sz, name) = line.split_once(' ').unwrap();
            Token::file(sz.parse::<usize>().unwrap(), name)
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
