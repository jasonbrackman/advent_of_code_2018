use regex::Regex;
use std::collections::{HashMap, HashSet};


pub fn manhattan_distance(x: &[i32], y: &[i32]) -> i32 {
    let mut collection = 0;

    for (a, b) in x.iter().zip(y.iter()) {
        collection += (a-b).abs();
    }

    collection

}

pub fn calculate_all_squares_manhattan_distance_to_list(input: &str) -> i32 {
    let nums = parse_lines_to_points(input);

    let mut board = [[99; 400]; 400];

    for i in 0..400 {
        for j in 0..400 {

            if i == 0 || j == 0 || i == 400-1 || j == 400-1 {
                board[i][j] = 10_001;
            } else {
                let mut total = 0;
                for (a, b) in nums.iter() {
                    let temp = manhattan_distance(&[i as i32, j as i32], &[*a, *b]);
                    total += if temp < 10_000 { temp } else { 10_001 };
                }
                board[i][j] = total;
            }
        }
    }

    let mut part_b = 0;
    for i in 0..400 {
        for j in 0..400 {
            if board[i][j] < 10_000 {
                part_b += 1;
            }
        }
    }

    part_b

}

pub fn parse_coordinates(input: &str) -> i32 {
    let nums = parse_lines_to_points(input);

    let mut off_limits: HashSet<i32> = HashSet::new();
    off_limits.insert(99);

    let mut board = [[99; 400]; 400];

    for i in 0..400 {
        for j in 0..400 {
            let distance: Vec<i32> = nums
                .iter()
                .map(|(a, b)| manhattan_distance(&[i, j], &[*a, *b]))
                .collect();

            let min = distance.iter().min().unwrap();

            if distance.iter().filter(|&n| *n == *min).count() > 1 {
                board[i as usize][j as usize] = 99;
            }

            else {
                let index = distance.iter().position(|&r| r == *min).unwrap();
                board[i as usize][j as usize] = index as i32;
            }

            if i == 0 || j == 0 || i == 400-1 || j == 400-1 {
                off_limits.insert(board[i as usize][j as usize]);
            }
        }
    }


    let mut hmap = HashMap::new();
    for i in 0..400  {
        for j in 0..400 {
            if !off_limits.contains(&board[i][j]) {
                let x = hmap.entry(board[i][j]).or_insert(0);
                *x += 1;
            }
        }
    }

    get_hashmap_max_value(&mut hmap)
}

pub fn get_hashmap_max_value(hmap: &mut HashMap<i32, i32>) -> i32 {
    // collect the max value in the hashmap
    let mut max_value = 0;
    for (_key, value) in hmap.iter() {
        if *value > max_value {
            max_value = *value;
        }
    }
    max_value
}

fn parse_lines_to_points(input: &str) -> Vec<(i32, i32)> {
    // take in a &str and parse it into x,y i32 coordinates.

    let re = Regex::new(r"^(\d+), (\d+)").unwrap();

    let mut nums: Vec<(i32, i32)> = Vec::new();

    for line in input.lines() {
        for cap in re.captures_iter(line) {
            nums.push(( cap[1].parse::<i32>().unwrap(), cap[2].parse::<i32>().unwrap()) );
        }
    }

    nums
}

#[test]
fn test_doit() {
    println!("MD: {}", manhattan_distance(&[0, 0],&[1, 6]));
    println!("MD: {}", manhattan_distance(&[0,1], &[1, 6]));
    println!("MD: {}", manhattan_distance(&[0,2], &[1, 6]));
    println!("MD: {}", manhattan_distance(&[0,3], &[1, 6]));
    println!("MD: {}", manhattan_distance(&[0,0], &[8, 3]));
    println!("MD: {}", manhattan_distance(&[0,1], &[8, 3]));
    println!("MD: {}", manhattan_distance(&[0,2], &[8, 3]));
    println!("MD: {}", manhattan_distance(&[0,3], &[8, 3]));
    assert_eq!(0, 0);
}

