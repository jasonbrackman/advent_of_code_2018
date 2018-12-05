extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;

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
    let mut hmap = day_04::create_timesheet(items);


    let part_a = day_04::part_a(&mut hmap);
    let part_b = day_04::part_b(&mut hmap);
    assert_eq!(part_a, 140_932);
    assert_eq!(part_b, 51_232);

    println!("Day 04: Part A: {}; Part B: {}", part_a, part_b);

}

fn day_05_run() {
    let path = "data/day_05.txt";
    let data = read(path);
    // day_05::doit(&data);
    let (part_a, part_b) = day_05::reduce(&data);

    assert_eq!(part_a, 11668);
    assert_eq!(part_b, 4652);

    println!("Day 05: Part A: {}; Part B: {}", part_a, part_b);

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
}

