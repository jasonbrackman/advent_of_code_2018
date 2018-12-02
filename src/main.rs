extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

mod day_01;
mod day_02;

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
    day_01_run();
    day_02_run();
}
