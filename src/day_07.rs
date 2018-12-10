use std::collections::{HashMap, HashSet};
struct Tree {
    content: HashMap<u8, HashSet<u8>>,
    offset: u8,
    ticks: i32,
    worker_blocked_for: HashMap<u8, u8>
}


impl Tree {
    pub fn new(input: &str) -> Tree {
        let v_steps = Tree::parse_input_to_steps(input);
        let content = Tree::build_tree_content(&v_steps);
        Tree{content, offset: 64u8,  ticks:0, worker_blocked_for:HashMap::new()}
    }

    fn parse_input_to_steps(input: &str) -> Vec<(u8, u8)> {
        let mut v_steps = Vec::new();
        for line in input.lines() {
            let x = line.chars().nth(5).expect("Cannot Parse From") as u8;
            let y = line.chars().nth(36).expect("Cannot Parse To") as u8;
            v_steps.push((x, y));
        }
        v_steps
    }

    fn build_tree_content(v_steps: &[(u8, u8)]) -> HashMap<u8, HashSet<u8>>{
        // find all dependencies
        let mut dependencies: HashMap<u8, HashSet<u8>> = HashMap::new();
        for (a, b) in v_steps.iter() {
            { // scoping to avoid multiple mutable borrows.
                // build up the dependencies based on the rhs first
                let mut current = dependencies.entry(b.clone()).or_insert_with(HashSet::new);
                current.insert(a.clone());
            }
            // and also ensure that the lhs is taken care of -- it should be populated with the above
            // OR be empty since it has no parents.
            dependencies.entry(a.clone()).or_insert_with(HashSet::new);
        }
        dependencies
    }

    pub fn get_next_step(&mut self) -> Option<u8> {

        let mut nexts = Vec::new();
        for (k, v) in self.content.iter() {
            if v.is_empty() {
                nexts.push(k);
            }
        }
        nexts.sort();

        let result = nexts.first().unwrap();

        if nexts.len() == 1 {
            if self.worker_blocked_for.contains_key(result) {
                if self.worker_blocked_for.get(&result).unwrap() > &0 {
                    *self.worker_blocked_for.entry(**result).or_insert(0) -= 1;
                    return None;
                }
            } else {
                self.worker_blocked_for.insert(**result, *result - self.offset);
                return None;
            }
        }

        Some(**result)
    }
}


pub fn doit(input: &str) {
    let mut part_a = String::new();
    let mut tree = Tree::new(input);

    // trying to figure out the queue system...
    // let mut workers = vec![0u8; 2];
    let mut current_keys = vec![0u8; 2];

    while !tree.content.is_empty() && tree.ticks < 25 {
        tree.ticks += 1;

        for index in 0..2 {

            // obtaining new item -- but only do this if the current key is done
            let next_step = tree.get_next_step();
            if let Some(n) = next_step { if n != current_keys[index] {
                    // println!("{}", n as char);
                    current_keys[index] = n;
                    part_a.push(current_keys[index] as char);
                    tree.content.remove(&current_keys[index]);
                    for steps_necessary in tree.content.values_mut() {
                        steps_necessary.remove(&current_keys[index]);
                    }
                }
            }
        }

        println!("[{}] ({}, {}) -> {}",
                 tree.ticks,
                 current_keys[0] as char,
                 current_keys[1] as char,
                 part_a);
    }

    assert_eq!(part_a, "CABDFE".to_string());
    println!("Part A: {}", part_a);
    println!("Part B: {}", tree.ticks);
}

#[test]
fn test_doit() {
    assert_eq!(0, 0);
}

