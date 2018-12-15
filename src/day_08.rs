use std::fmt;
use std::collections::VecDeque;

struct Node {
    level: i32,
    children: i32,
    metadata: Vec<i32>
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parent: {}  with Children: {} -> Metadata: {:?}", self.level, self.children, self.metadata)
    }
}

struct Tree {
    data: VecDeque<i32>,
    nodes: Vec<Node>,
}

impl Tree {
    fn new(input: &str) -> Tree {
        let mut data = Tree::parse_data(input);
        Tree { data, nodes: Vec::new() }
    }

    fn parse_data(input: &str) -> VecDeque<i32> {
        input
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<VecDeque<i32>>()
    }

    pub fn get_next_node(&mut self, level: i32) {
        // This node is unique as the start and end of the information is
        // guaranteed to be related.

        // First two numbers indicate the children & metacount
        let children = self.data.pop_front().unwrap();
        let metacount = self.data.pop_front().unwrap();

        let mut metadata = Vec::new();
        if level == 0 {
            // For the root node the metacount is the last number of digits of the input.

            for m in 0..metacount {
                metadata.push(self.data.pop_back().unwrap());
            }
        }

        self.nodes.push(Node { level, children, metadata });
    }

    pub fn add_metadata(&self) -> i32 {
        let mut total = 0;
        for node in self.nodes.iter() {
            total += node.metadata.iter().sum::<i32>();
        }
        total
    }


}

fn get_info(input: &str) {
    let mut tree = Tree::new(input);
    tree.get_next_node(0);

    for node in tree.nodes.iter() {
        println!("Tree example: {}", node);
    }

    println!("Metadata Total: {}", tree.add_metadata());




}
#[test]
fn test_parse_data() {
    let data = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2".to_string();
    get_info(&data);
}