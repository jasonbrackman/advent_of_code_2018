extern crate regex;
extern crate serde;

use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
#[allow(dead_code)]
mod day_05;
mod day_06;
mod day_07;
mod day_08;

fn read(path: &str) -> String {
    let mut f = File::open(path).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    contents
}


fn day_01_run() {
    let path = "data/day_01.txt";
    let data = read(path);

    let (part_a, part_b) = day_01::counter_of_text(&data);

    assert_eq!(part_a, 533);
    assert_eq!(part_b, 73272);

    println!("Day 01: Part A: {}; Part B: {}", part_a, part_b);

}


fn day_02_run() {
    let path = "data/day_02.txt";
    let data = read(path);
    let part_a = day_02::count_character_repetition(&data);
    let part_b = day_02::diff_lines_of_input(&data);

    assert_eq!(part_a, 4980);
    assert_eq!(part_b, "qysdtrkloagnfozuwujmhrbvx".to_string());

    println!("Day 02: Part A: {}; Part B: {}", part_a, part_b);

}

fn day_03_run() {
    let path = "data/day_03.txt";
    let data = read(path);
    let (part_a, part_b) = day_03::doit(&data);

    assert_eq!(part_a, 124_850);
    assert_eq!(part_b, 1097);

    println!("Day 03: Part A: {}; Part B: {}", part_a, part_b);

}

fn day_04_run() {
    let path = "data/day_04.txt";
    let data = read(path);

    let items = day_04::get_sorted_items(&data);
    let mut hmap = day_04::create_timesheet(&items);


    let part_a = day_04::part_a(&mut hmap);
    let part_b = day_04::part_b(&mut hmap);
    assert_eq!(part_a, 140_932);
    assert_eq!(part_b, 51_232);

    println!("Day 04: Part A: {}; Part B: {}", part_a, part_b);

}

fn day_05_run() {
    let path = "data/day_05.txt";
    let data = read(path);

    // based off of a python implementation
    let (part_a, part_b) = day_05::testing(&data);

    // original solution
    // let (part_a, part_b) = day_05::reduce(&data);

    assert_eq!(part_a, 11668);
    assert_eq!(part_b, 4652);

    println!("Day 05: Part A: {}; Part B: {}", part_a, part_b);

}

fn day_06_run() {
    let path = "data/day_06.txt";
    let data = read(path);

    let part_a = day_06::parse_coordinates(&data);
    let part_b = day_06::calculate_all_squares_manhattan_distance_to_list(&data);

    assert_eq!(part_a, 2342);
    assert_eq!(part_b, 43302);

    println!("Day 06: Part A: {}; Part B: {}", part_a, part_b);

}

fn day_07_run() {
    let path = "data/day_07.txt";
    let data = read(path);

    let (part_a, part_b) = day_07::doit(&data);

    assert_eq!(part_a, "ABLCFNSXZPRHVEGUYKDIMQTWJO".to_string());
    assert_eq!(part_b, 1157);

    println!("Day 07: Part A: {}; Part B: {}", part_a, part_b);

}

fn day_08_run() {
    let path = "data/day_08.txt";
    let data = read(path);
    let mut tree = day_08::Tree::new(&data);
    let root = tree.get_next_node();

    let part_a = root.add_metadata();
    let part_b = root.add_metadata2();
    assert_eq!(part_a, 44838);
    assert_eq!(part_b, 22198);

    println!("Day 08: Part A: {}; Part B: {}", part_a, part_b);

}

pub fn time_it(func: fn() -> ()) {
    // Marker for benchmarking start
    let start = Instant::now();

    func();

    // Benchmarking
    let time = Instant::now() - start;
    let time_secs = time.as_secs();
    let time_millis = time.subsec_millis();

    println!("\t|-> Done in {} seconds.", time_secs as f32 + time_millis as f32 / 1000.0);
}

fn main() {
    time_it(day_01_run);
    time_it(day_02_run);
    time_it(day_03_run);
    time_it(day_04_run);
    time_it(day_05_run);
    time_it(day_06_run);
    time_it(day_07_run);
    time_it(day_08_run);

}

