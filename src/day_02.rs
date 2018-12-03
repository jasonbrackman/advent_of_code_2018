use std::collections::HashMap;

/// Count the repetition of strings that have exactly:
///     -> 2 of the same character and/or
///     -> 3 of the same character.
pub fn count_character_repetition(input: &str) -> i64 {
    let mut count_of_2 = 0;
    let mut count_of_3 = 0;

    for line in input.lines() {
        let mut hmap: HashMap<char, i32> = HashMap::new();
        for c in line.chars() {
            let mut x = hmap.entry(c).or_insert(0);
            *x += 1;
        }

        if hmap.iter().any(|(_, &v)| v == 2) { count_of_2 += 1};
        if hmap.iter().any(|(_, &v)| v == 3) { count_of_3 += 1};
    }

    count_of_2 * count_of_3
}

/// Dif every string against all other strings to see which one has the least amount of difference
/// --> return the result without the character(s) that are different.
pub fn diff_lines_of_input(input: &str) -> String {
    let mut result = "".to_string();
    let mut total = 0;
    let lines: Vec<&str> = input.lines().collect();

    for line1 in lines.iter() {
        for line2 in lines.iter() {
            let (score, bad) = how_similar(line1, line2);
            if score > total && score != 26 { // nothing is perfect!
                total = score;
                result = "".to_string();
                for c in line1.chars() {
                    if !bad.contains(&c) { result.push(c) };
                }
//                println!("{} -> {} - {:?}", line1, total, bad );
            }
        }
    }

    result
}


pub fn how_similar(input1: &str, input2: &str) -> (i64, Vec<char>) {
    let mut score = 0;
    let mut bad = Vec::new();
    for (a, b) in input1.chars().zip(input2.chars()) {
        if a == b {
            score += 1;
        }
        else {
            bad.push(a);
        }
    }

    (score, bad)
}


#[test]
fn test_input() {
    let input1 = "fghij";
    let input2 = "fguij";
    assert_eq!(how_similar(&input1, &input2), (4, vec!('h', 'u')));
}

