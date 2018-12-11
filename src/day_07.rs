use std::collections::{HashMap, HashSet};
struct Tree {
    content: HashMap<u8, HashSet<u8>>,
    offset: u8,
    ticks: i32,
    worker_blocked_for: HashMap<u8, u8>,
    workers: [u8; 2]
}


impl Tree {
    pub fn new(input: &str) -> Tree {
        let v_steps = Tree::parse_input_to_steps(input);
        let content = Tree::build_tree_content(&v_steps);
        Tree{content, offset: 65u8,  ticks:0, worker_blocked_for:HashMap::new(), workers: [0; 2]}
    }

    fn tick(&mut self) {
        self.ticks += 1;
        for (_, value) in self.worker_blocked_for.iter_mut() {
            if value > &mut 0 {
                *value -= 1;
            }
        }
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

        if nexts.len() == 0 {
            // we're done
            return None
        }

        let result = nexts.first().expect("something went wrong.");
        self.worker_blocked_for.entry(**result).or_insert(*result - self.offset);
//        if self.worker_blocked_for.get(&result).expect("what the what") > &0 {
//            return None;
//        }

        Some(**result)
    }
}

pub fn doit(input: &str) {
    let mut part_a = String::new();
    let mut tree = Tree::new(input);

    // trying to figure out the queue system...
    // let mut workers = vec![0u8; 2];
    let mut workers = vec![0u8; 2];

    while !tree.content.is_empty() && tree.ticks < 400 {

        for index in 0..tree.workers.len() {

            // obtaining new item -- but only do somethign with it if the current key is done
            let next_step = tree.get_next_step();

            if let Some(n) = next_step {
                let currently_being_worked_on = workers.iter().any(|x| x == &n);
                if !currently_being_worked_on {
                    workers[index] = n;
                }
                else {
                    if tree.worker_blocked_for.get(&n).expect("what the what") == &0 {
                        part_a.push(n as char);
                        tree.content.remove(&n);
                        for steps_necessary in tree.content.values_mut() {
                            steps_necessary.remove(&n);
                        }
                    }
                }


            }
        }

        println!("[{}] ({}, {}) -> {}",
                 tree.ticks,
                 workers[0] as char,
                 workers[1] as char,
                 part_a);
        tree.tick();
    }

    assert_eq!(part_a, "CABDFE".to_string());
    println!("Part A: {}", part_a);
    println!("Part B: {}", tree.ticks);
}

#[test]
fn test_doit() {
    assert_eq!(0, 0);
}

