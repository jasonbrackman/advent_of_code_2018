use std::collections::VecDeque;
use std::fmt;

struct Node {
    children: i32,
    metadata: Vec<i32>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Children: {} -> Contains [{}] Metadata: {:?}",
            self.children, self.metadata.len(), self.metadata
        )
    }
}

pub struct Tree {
    data: VecDeque<i32>,
    nodes: Vec<Node>,
}

impl Tree {
    pub fn new(input: &str) -> Tree {
        let data = Tree::parse_data(input);
        Tree {
            data,
            nodes: Vec::new(),
        }
    }

    pub fn print_nodes(&self) {
        for item in self.nodes.iter() {
            println!("{}", item);
        }
    }
    fn parse_data(input: &str) -> VecDeque<i32> {
        input
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<VecDeque<i32>>()
    }

    pub fn get_next_node(&mut self, take: i32) -> bool {
        for _ in 0..take {

            // This node is unique as the start and end of the information is
            // guaranteed to be related.
            if self.data.is_empty() || take == 0 {
                return false;
            }

            // First two numbers indicate the children & metacount
            let children = self
                .data
                .pop_front()
                .expect("Child Count could not be determined");

            let metacount = self.data.pop_front().expect("Missing metacount?");


            let mut metadata = Vec::new();

            if children != 0 {
                // println!("Dealing +1 [{}] children <Recursive> ...", children);
                // need to collect all the children nodes and what is left over is the metadata
                self.get_next_node(children);
            }
            for _ in 0..metacount {
                match self.data.pop_front() {
                    Some(n) => metadata.push(n),
                    _ => (),
                }
            }

            self.nodes.push(Node {
                children,
                metadata,
            });
        }
        true
    }

    pub fn add_metadata(&self) -> i32 {
        let mut total = 0;
        for node in self.nodes.iter() {
            total += node.metadata.iter().sum::<i32>();
        }
        total
    }
}

#[test]
fn test_parse_data() {
    let data = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2".to_string();
    let mut tree = Tree::new(&data);
    tree.get_next_node(1);
    assert_eq!(tree.add_metadata(), 138);
}
