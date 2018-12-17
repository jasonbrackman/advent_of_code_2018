use std::collections::VecDeque;
use std::fmt;

#[derive(Default)]
pub struct Node {
    children: i32,
    metadata: Vec<i32>,
    value: i32,
    childs: Vec<Node>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Children: {} -> Metadata: {:?} -> Value: {}",
            self.children, self.metadata, self.value
        )
    }
}

impl Node {
    pub fn add_metadata(&self) -> i32 {
        let sum: i32 = self.childs.iter().map(|node| node.add_metadata()).sum();
        self.metadata.iter().sum::<i32>() + sum
    }

    pub fn add_metadata2(&self) -> i32 {

        if self.childs.is_empty() {
            // If a node has no child nodes, its value is the sum of its metadata entries
            self.add_metadata()
        } else {
            // if a node does have child nodes, the metadata entries become indexes which
            // refer to those child nodes.

            self.metadata
                .iter()
                // Must be greater than zero, but is 1 indexed so can reach the end.
                .filter(|&&m| m > 0 && (m as usize) <= self.childs.len())
                .map(|&x| self.childs[(x-1) as usize].add_metadata2())
                .sum()  // 48785 too high
        }
    }
}

pub struct Tree {
    data: VecDeque<i32>,
}

impl Tree {
    pub fn new(input: &str) -> Tree {
        let data = Tree::parse_data(input);
        Tree {
            data,
        }
    }

    fn parse_data(input: &str) -> VecDeque<i32> {
        input
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<VecDeque<i32>>()
    }

    pub fn get_next_node(&mut self) -> Node {

        // First two numbers indicate the children & metacount
        let children = self.data.pop_front().expect("Child Count Missing?");
        let metacount = self.data.pop_front().expect("Metacount Missing?");

        // create a node -- which can be filled in as we go as it may contain children
        let mut node = Node::default();


        for _ in 0..children {
            // need to collect all the children nodes and what is left over will be metadata
            let result = self.get_next_node();
            node.childs.push(result);
        }

        for _ in 0..metacount {
            if let Some(n) = self.data.pop_front() { node.metadata.push(n) };
        }

        node.children = children;
        node
    }
}

#[test]
fn test_parse_data() {
    let data = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2".to_string();
    let mut tree = Tree::new(&data);
    let root = tree.get_next_node();
    let test_a = root.add_metadata();
    let test_b = root.add_metadata2();
    assert_eq!(test_a, 138);

    assert_eq!(test_b, 66);
    println!("{}-{}", test_a, test_b);
}
