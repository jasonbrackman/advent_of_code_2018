// use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn doit(input: &str) {

    let v_steps = parse_input_to_steps(input);

    // find all dependencies
    let mut dependencies: HashMap<&String, HashSet<&String>> = HashMap::new();
    for (a, b) in v_steps.iter() {
        { // scoping to avoid multiple mutable borrows.
            // build up the dependencies based on the rhs first
            let mut current = dependencies.entry(b).or_insert_with(HashSet::new);
            current.insert(a);
        }
        // and also ensure that the lhs is taken care of -- it should be populated with the above
        // OR be empty since it has no parents.
        dependencies.entry(a).or_insert_with(HashSet::new);
    }

    let mut part_a = String::new();
    let mut ticks = 0;

    // What I care about:
    // 1. that a thread is working when available
    // 2. that a thread can't know about future work.  Just work to do.
    //
    let mut workers = vec![0u8; 2];
    let mut qMap: HashMap<String, bool> = HashMap::new();
    let mut current_key = String::from("");

    while !dependencies.is_empty() {
        ticks += 1;


        let should_wait = false; // qMap.iter().any(|(_x, y)| y == &true);
        for (_index, worker) in workers.iter_mut().enumerate() {
            if *worker == 0 && !should_wait {
                // obtaining new item -- but only do this if the current key is done
                let (next_step, wait) = get_next_step(&mut dependencies);
                if next_step != current_key {
                    qMap.remove(&current_key);
                    current_key = next_step.clone();
                    qMap.entry(current_key.clone()).or_insert(wait);

                    *worker = next_step.parse::<char>().unwrap() as u8 - 64;

                }
                if worker == &mut 0u8 {

                    part_a.push(current_key.parse::<char>().unwrap());
                    dependencies.remove(&current_key);
                    for steps_necessary in dependencies.values_mut() {
                        steps_necessary.remove(&current_key);
                    }
                }

            }
        }

        for worker in workers.iter_mut() {
            if worker > &mut 0u8 {
                *worker -= 1;
                // qMap.get_key_value()
            }

        }
        println!("[{}] {} Workers: {:?} -> {:?}", ticks, current_key, workers, qMap);


    }


    println!("Part A: {}", part_a);
    println!("Part B: {}", ticks);
}

fn get_next_step(dependencies: &mut HashMap<&String, HashSet<&String>>) -> (String, bool) {
    //let mut next = "zzzzz".to_string();
    // only accept an answer that is sorted to first.
    let mut nexts = Vec::new();
    for (k, v) in dependencies.iter() {
        if v.is_empty() {
            nexts.push(k.to_string());
        }
    }
    nexts.sort();
    (nexts.first().unwrap().to_string(), nexts.len() == 1)
}

fn parse_input_to_steps(input: &str) -> Vec<(String, String)> {
    let mut v_steps = Vec::new();
    for line in input.lines() {
        let x = line.chars().nth(5).expect("Cannot Parse From").to_string();
        let y = line.chars().nth(36).expect("Cannot Parse To").to_string();
        v_steps.push((x, y));
    }
    v_steps
}

//    println!("================");
//    let alpha = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
//
//    // step 01: Find out where to start
//    // --> can only appear on the lhs and only appear once
//    let mut lhs = Vec::new();
//    let mut rhs = Vec::new();
//    for (x, y) in v_steps.iter() {
//        lhs.push(x);
//        rhs.push(y);
//    }
//
//    // start point won't have any parents
//    let mut start_points = Vec::new();
//    for l in lhs.iter() {
//        if get_parents(&v_steps, l).is_empty() {
//            start_points.push(l);
//        }
//    }
//    start_points.dedup();
//    start_points.sort();
//
//    // end points don't have any children.
//    let mut end_point = String::new();
//    for a in alpha.chars() {
//        if !lhs.contains(&&a.to_string()) && rhs.contains(&&a.to_string()) {
//            end_point.push(a);
//        }
//    }
//
//
//    println!("{:?} ----> {}", start_points, end_point);
//    for item in start_points.iter() {
//        println!("{}--> {:?}", item, get_children(item, &v_steps));
//    }

    // level 1
    //    for item in start_points.iter() {
    //        for (x, y) in v_steps.iter() {
    //            if x.contains(item) {
    //                println!("{} -> {}", x, y);
    //
    //            }
    //        }
    //    }

//    println!("current: {}", end_point);
//    let collection = get_parents(&v_steps, &end_point);
//    println!("Parents: {}-{:?}", end_point, collection);
//    for item in collection.iter() {
//        println!("{}-{:?}", item, get_parents(&v_steps, &item));
//    }
/*
fn get_parents(two_stack: &[(String, String)], end_point: &str) -> Vec<String> {
    let mut collection = Vec::new();
    for (x, y) in two_stack.iter() {
        if y == end_point {
            collection.push(x.to_string());
        }
    }
    collection.sort();
    collection.reverse();
    collection
}

fn get_children(start_point: &str, steps: &[(String, String)]) -> Vec<String> {
    let mut stack = Vec::new();

    for (a, b) in steps.iter() {
        if a == start_point {
            stack.push(b.clone());
        }
    }

    stack.sort();
    stack
}
*/
#[test]
fn test_doit() {
    assert_eq!(0, 0);
}

// counter example
//let mut start_point = "";
//for l in lhs.iter() {
//    if lhs.iter().filter(|n| *n == l).count() == 1 {
//        start_point = *l;
//        break;
//    }
//}
