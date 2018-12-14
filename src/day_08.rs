use std::fmt;

fn parse_data(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

struct Node {
    level: i32,
    children: i32,
    metadata: Vec<i32>
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] Children: {} -> Metadata: {:?}", self.level, self.children, self.metadata)
    }

}
fn get_info(input: &str) {
    // header
    // child nodes
    // metadata

    let data = parse_data(input).iter();
    let mut nodes = Vec::new();
    let mut level = 0;
    {
        let info: Vec<&i32> = data.take(2).collect();
        let children = info[0];
        let metacount = info[1];

        let mut metadata = Vec::new();
        for m in 0..*metacount {
            metadata.push(m);
        }

        for index in 0..*children {
            nodes.push(Node { level, children: *children, metadata });
        }
        level += 1;
    }

    for node in nodes.iter() {
        println!("Tree example: {}", node);
    }




}
#[test]
fn test_parse_data() {
    let data = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2".to_string();
    get_info(&data);
}