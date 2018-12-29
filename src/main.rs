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
#[allow(dead_code)]
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;

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
#[allow(dead_code)]
fn day_09_run() {
    let part_a = day_09::part_a(459, 71790);
    let part_b = day_09::part_a(459, 71790 * 100);

    assert_eq!(part_a, 386_151);
    // assert_eq!(part_b, 3_211_264_152); // takes 6338 seconds currently... :(
    println!("Day 09: Part A: {}; Part B: {}", part_a, part_b);

}

fn day_10_run() {
    let path = "data/day_10.txt";
    let data = read(path);
    let mut board = day_10::Board::new(&data);

    let mut counter = 0;
    let mut go = true;
    while go {
        counter += 1;
        board.update();
        if counter > 10_000 { // just guessing to get to this 10_000; looking for easier iteration
            go = board.init_board();
        }
    }
    board.draw_position();
    // println!("Took this long: {}", counter);
    let part_a = "CRXKEZPZ";
    /*
    -> .xxxx...xxxxx...x....x..x....x..xxxxxx..xxxxxx..xxxxx...xxxxxx
    -> x....x..x....x..x....x..x...x...x............x..x....x.......x
    -> x.......x....x...x..x...x..x....x............x..x....x.......x
    -> x.......x....x...x..x...x.x.....x...........x...x....x......x.
    -> x.......xxxxx.....xx....xx......xxxxx......x....xxxxx......x..
    -> x.......x..x......xx....xx......x.........x.....x.........x...
    -> x.......x...x....x..x...x.x.....x........x......x........x....
    -> x.......x...x....x..x...x..x....x.......x.......x.......x.....
    -> x....x..x....x..x....x..x...x...x.......x.......x.......x.....
    -> .xxxx...x....x..x....x..x....x..xxxxxx..xxxxxx..x.......xxxxxx
    */
    let part_b = counter -1;
    assert_eq!(part_b, 10081);
    println!("Day 10: Part A: {}; Part B: {}", part_a, part_b);

}

fn day_11_run() {
    let part_a = day_11::part_a();
    let part_b = day_11::part_b();

    assert_eq!(part_a, (21,93));
    assert_eq!(part_b, (231, 108, 14)); // takes 6338 seconds currently... :(
    println!("Day 11: Part A: {:?}; Part B: {:?}", part_a, part_b);

}

fn day_12_run() {
    let path = "data/day_12.txt";
    let data = read(path);

    let (mut pots, rules) = day_12::parse(&data);
    let part_a = day_12::process_pots(&mut pots, &rules, 20);


    // 50B seems like a big enough number to cause the iterations to overflow.
    // But, at some point the pots stabilize and growth is now happening in a consistent manner.
    // sooo --- multiply the growth rate by the number of iterations left to go...
    let (mut pots, rules) = day_12::parse(&data);
    let result = day_12::process_pots(&mut pots, &rules, 1_000);
    let part_b = i64::from(result) + (86 * (50_000_000_000 - 1_000));;

    assert_eq!(part_a, 3337);
    assert_eq!(part_b, 4_300_000_000_349);

    println!("Day 12: Part A: {}; Part B: {}", part_a, part_b);

}

fn day_13_run() {
    let path = "data/day_13.txt";
    let data = ::read(path);

    let part_a = day_13::part_a(&data);
    let part_b = day_13::part_b(&data);

    assert_eq!(part_a, (48, 20));
    assert_eq!(part_b, (59, 64));

    println!("Day 13: Part A: {:?}; Part B: {:?}", part_a, part_b);

}

fn day_14_run() {
    let part_a = day_14::part_a("37", 84_601, None);
    let part_b = day_14::part_a("37", 100_000, Some("084601"));
    assert_eq!(part_a, "2688510125");
    assert_eq!(part_b.len(), 20188250);

    println!("Day 14: Part A: {:?}; Part B: {:?}", part_a, part_b.len());

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
    // time_it(day_09_run); / 797s -- so only uncomment if you can wait.
    time_it(day_10_run);
    time_it(day_11_run);
    time_it(day_12_run);
    time_it(day_13_run);
    time_it(day_14_run);
}

