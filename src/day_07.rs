use regex::Regex;
//use std::collections::{HashMap, HashSet};

pub fn doit(input: &str) {
    // let mut hmap: HashMap<char, char> = HashMap::new();
    let re = Regex::new(r"^.+\s(.)\s.*\s(.)\s.*").unwrap();


    let mut two_stack = Vec::new();
    for line in input.lines() {
        for caps in re.captures_iter(line) {
            // println!("{} {}", &caps[1], &caps[2]);
            two_stack.push((caps[1].to_string(), caps[2].to_string()) )
        }
    }

    let alpha = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();

    let mut left_side = Vec::new();
    for (x, y) in two_stack.iter() {
        left_side.push(x);
    }

    let mut end_point = String::new();
    for a in alpha.chars() {
        if !left_side.contains(&&a.to_string()) {
            end_point.push(a);
        }
    };

    println!("Start: {}", end_point);
    let collection = get_parents(&mut two_stack, &mut end_point);
    println!("{:?}", collection);


}

fn get_parents(two_stack: &mut Vec<(String, String)>, end_point: &mut String) -> Vec<&String> {
    let mut collection = Vec::new();
    for (x, y) in two_stack.iter() {
        if y == &end_point {
            collection.push(x);
        }
    }
    collection.sort();
    collection.reverse();
    collection
}

#[test]
fn test_doit() {
    assert_eq!(0, 0);
}