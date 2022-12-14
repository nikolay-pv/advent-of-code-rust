use std::vec;

const INPUT_TXT: &str = include_str!("input.txt");

fn main() {
    let input = read_input(INPUT_TXT);
    println!("Answer to first parts is {}", solve_first(&input));
    println!("Answer to second parts is {}", solve_second(&input));
}

#[derive(Debug, Clone)]
struct FSNode<'node> {
// FileSystemNode
    name: &'node str,
    size: usize,
    children: Option<Vec<FSNode<'node>>>,
}

impl<'node> FSNode<'node> {
    fn new_file(name: &'node str, size: usize) -> Self {
        FSNode { name, size, children: None }
    }

    fn new_directory(name: &'node str, size: usize, children: Vec<FSNode<'node>>) -> Self {
        FSNode { name, size, children: Some(children) }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn add_size(&mut self, d: usize) {
        self.size += d;
    }

    fn name(&self) -> &'node str {
        self.name
    }

    fn is_directory(&self) -> bool {
        self.children.is_some()
    }

    fn fd(&mut self, dir_name: &'node str) -> Option<&mut FSNode<'node>> {
        self.children.as_mut().and_then(|children| {
                children.into_iter()
                    .filter(|node| node.children.is_some())
                    .find(|node| node.name() == dir_name)
        })
    }

    fn add_node(&mut self, node: FSNode<'node>) {
        match &mut self.children {
            None => unreachable!(),
            Some(children) => {
                assert!(!children.into_iter().any(|x| x.name() == node.name()));
                children.push(node);
            },
        }
    }

    fn iter(&self) -> FSIter {
        FSIter::new(self)
    }
}

enum FSIterState<'node> {
    Start,
    CurrentNode,
    Iter(Box<dyn Iterator<Item = &'node FSNode<'node>> + 'node>),
}

struct FSIter<'node> {
    node: &'node FSNode<'node>,
    state: FSIterState<'node>,
}

impl<'node> FSIter<'node> {
    fn new(node: &'node FSNode<'node>) -> Self {
        FSIter { node, state: FSIterState::Start }
    }
}

impl<'node> Iterator for FSIter<'node> {
    type Item = &'node FSNode<'node>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.state {
            FSIterState::Start => {
                self.state = FSIterState::CurrentNode;
                Some(self.node)
            }
            FSIterState::CurrentNode => {
                let Some(children) = &self.node.children else {
                    return None;
                };
                self.state = FSIterState::Iter(Box::new(children.into_iter().map(FSNode::iter).flatten()));
                self.next()
            }
            FSIterState::Iter(iter) => iter.next(),
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
    node.iter()
        .filter_map(|node|{
            if node.is_directory() && node.size() <= threshold {
                Some(node.size())
            } else { None }
        })
        .sum()
}

fn solve_second(input: &Vec<Token>) -> usize {
    let node = parse_all(&input);
    let total_size = 70000000;
    let total_available = total_size - node.size(); // root
    let required_free_space = 30000000;
    node.iter()
        .filter_map(|node|{
            if node.is_directory() && node.size() + total_available >= required_free_space {
                Some(node.size())
            } else { None }
        })
        .fold(required_free_space, |smallest, size| smallest.min(size))
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
