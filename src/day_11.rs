
/*
Rules:
The power level in a given fuel cell can be found through the following process:

Find the fuel cell's rack ID, which is its X coordinate plus 10.
Begin with a power level of the rack ID times the Y coordinate.
Increase the power level by the value of the grid serial number (your puzzle input).
Set the power level to itself multiplied by the rack ID.
Keep only the hundreds digit of the power level (so 12345 becomes 3; numbers with no hundreds digit become 0).
Subtract 5 from the power level.
*/
use std::collections::HashMap;

fn create_positions_for_grid(size: usize, serial: usize) -> HashMap<(usize, usize), i32> {
    let mut hmap: HashMap<(usize, usize), i32> = HashMap::new();

    // let mut positions = Vec::new();
    for x in 1..=size {
        for y in 1..=size {
            if x % 2 == 0 && y % 2 == 0 {
                hmap.entry((x, y)).or_insert(get_fuel_cell_value((x, y), serial));
            }

        }
    }
    hmap
}

fn process_position(position:(usize, usize), serial: usize) -> i32 {
    let (x, y) = position;

    // get rack ID
    let rack_id = x + 10;

    // get power level
    let mut power_level = rack_id * y;
    power_level +=serial;
    power_level *= rack_id;

    // set value
    let value = (power_level / 100) % 10; // dividing ints should result in an int
    (value as i32) - 5
}

fn get_fuel_cell_value(cell:(usize, usize), serial: usize) -> i32 {
    let mut total = 0;

    let (x, y) = cell;
    for i in x-1..=x+1 {
        for j in y-1..=y+1 {
            let temp = process_position((i, j), serial);
            total += temp;
        }
    }

    total

}

pub fn get_hashmap_max_value(hmap: &HashMap<(usize, usize), i32>) -> (usize, usize) {
    // collect the max value in the hashmap
    let mut max_key= (0, 0);
    let mut max_value = 0;
    for (key, value) in hmap.iter() {
        if *value > max_value {
            max_value = *value;
            max_key = *key;
        }
    }
    max_key
}


pub fn part_a() -> (usize, usize) {
    let serial = 1955;
    let pos = create_positions_for_grid(300, serial);
    let result = get_hashmap_max_value(&pos);
    println!("Result Part1: ({}, {}) -> {}", result.0 -1, result.1-1, pos[&result]);
    (result.0 -1, result.1-1)
}

#[test]
fn test_process_position() {
    // let serial = 8;
    let serial = 8;
    let result = process_position((3, 5), serial);
    assert_eq!(result, 4);
}

#[test]
fn test_fuel_cell() {
    let serial = 18;
    let pos = create_positions_for_grid(300, serial);
    let result = get_hashmap_max_value(&pos);
    println!("Result: ({}, {}) -> {}", result.0 -1, result.1-1, pos[&result]);
}

#[test]
fn test_fuel_cell_42() {
    let serial = 42;
    let pos = create_positions_for_grid(300, serial);
    let result = get_hashmap_max_value(&pos);
    println!("Result: ({}, {}) -> {}", result.0 -1, result.1-1, pos[&result]);
}




