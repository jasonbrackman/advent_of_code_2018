use std::fmt;
use std::collections::{HashMap, HashSet};


struct Tree {
    content: HashMap<u8, HashSet<u8>>,
    offset: u8,
    ticks: i32,
    worker_blocked_for: HashMap<u8, u8>,
    workers: Vec<u8>,
    output: String
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[{}] -> {}", self.ticks, self.output)?;
        for (index, item) in self.workers.iter().enumerate() {
            writeln!(f, "\tWorker {}: {} -> {:?}",
                     index,
                     self.workers[index] as char,
                     self.worker_blocked_for.get(item))?;
        }
        write!(f, "")
    }
}

impl Tree {
    pub fn new(input: &str, worker_count: usize, offset: u8) -> Tree {
        let v_steps = Tree::parse_input_to_steps(input);
        let content = Tree::build_tree_content(&v_steps);
        let mut workers = Vec::new();
        workers.resize(worker_count, 0u8);

        Tree{
            content,
            offset,
            ticks:0,
            worker_blocked_for:HashMap::new(),
            workers,
            output: String::from("")
        }
    }

    fn tick(&mut self) {

        // reduce time to block
        for (_, value) in self.worker_blocked_for.iter_mut() {
            if *value > 0 {
                *value -= 1;
            }
        }



        // remove completed content to the output and clean up the workers.
        for (k, v) in self.worker_blocked_for.iter() {
            if v == &0 {
                if !self.output.contains(*k as char) {
                    self.output.push(*k as char);
                    for worker in self.workers.iter_mut() {
                        if worker == k {
                            *worker = 0;
                        }
                    }
                    self.content.remove(&k);
                    for steps_necessary in self.content.values_mut() {
                        steps_necessary.remove(&k);
                    }
                }
            }
        }

        self.get_next_step();
        // println!("{}", self);
        self.ticks += 1;
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

    pub fn get_next_step(&mut self) {

        let mut nexts = Vec::new();

        // go through the workers and fill them up if free
        for index in 0..self.workers.len() {
            if self.workers[index] == 0 {

                // obtaining new item -- but only do somethign with it if the current key is done
                if nexts.is_empty() {
                    for (k, v) in self.content.iter() {
                        if v.is_empty() {
                            nexts.push(*k);
                        }
                        nexts.sort();
                        nexts.reverse();
                    }
                }

                if let Some(n) = nexts.pop() {
                    let currently_being_worked_on = self.workers.iter().any(|x| x == &n);
                    if !currently_being_worked_on {
                        self.workers[index] = n;
                        self.worker_blocked_for.entry(n).or_insert(n - self.offset);
                    }
                }
            }
        }
    }
}

pub fn doit(input: &str) -> (String, i32) {

    let mut tree_a = Tree::new(input, 1, 64u8);
    while !tree_a.content.is_empty() {
        tree_a.tick();
    }

    let mut tree_b = Tree::new(input, 5, 4u8);
    while !tree_b.content.is_empty() {
        tree_b.tick();
    }

    (tree_a.output, tree_b.ticks-1)
}

#[test]
fn test_doit() {
    assert_eq!(0, 0);
}

