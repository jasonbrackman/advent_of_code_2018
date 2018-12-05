
use std::collections::{HashSet};

// for part b I need to go through everything again, but removing the different pairs first
// then trying again per cycle looking for the shortest length.
pub fn reduce(input: &str) -> (i32, i32) {
    let mut seen = HashSet::new();
    let (mut max, pairs) = doit(&input);
    let part_a = max;

    for (c1, c2) in pairs {

        let mut s = String::from(c1.to_string());
        s.push(c2);

        let new = strip_characters(input, &s);
        if !seen.contains(&new) {
            println!("\t| Working... {}", c1);
            let (m, _) = doit(&new);
            if m < max {
                max = m;
            }
            seen.insert(new);
        }
    }
    // println!("Part A: {}", part_a);
    // println!("Part B: {}", max);
    (part_a, max)
}

fn strip_characters(original : &str, to_strip : &str) -> String {
    original.chars().filter(|&c| !to_strip.contains(c)).collect()
}


fn doit(input: &str) -> (i32, HashSet<(char, char)>) {
    let mut temp_c = ' ';

    let mut new = Vec::new();

    let mut input2 = input.to_string();
    let mut remove = 0;

    // for part b
    let mut pairs = HashSet::new();

    let mut count = 10_000_000;
    while count != input2.len() {
        let mut skip = false;
        count = input2.len();
        for c in input2.chars() {
            new.push(c);
            if !skip {
                if c != temp_c {
                    let needle = c.to_lowercase();
                    let haystack = temp_c.to_lowercase();
                    if needle.to_string() == haystack.to_string() {
                        remove += 2;

                        pairs.insert((new.pop().unwrap(), new.pop().unwrap()));

                        skip = true;

                    }
                }

                temp_c = c;

            }
        }

        input2 = new.iter().collect();
        new.clear();
        temp_c = ' ';

    }

    ((input.len()-remove) as i32, pairs)
}

#[test]
fn test_doit() {
    let input = "asdf\nfdsa";
    assert_eq!(doit(input), 0)
}


#[test]
fn test_doit2() {
    let input = "qGA";
    doit(input);
}

