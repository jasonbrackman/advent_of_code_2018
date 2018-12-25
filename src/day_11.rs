
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



fn create_positions_for_grid(step: usize, serial: usize) -> HashMap<(usize, usize), i32> {
    let total = 300;

    let mut hmap: HashMap<(usize, usize), i32> = HashMap::new();

    for x in 1..=total {
        for y in 1..=total {
            hmap
                .entry((x, y))
                .or_insert_with(||get_fuel_cell_value((x, y), step, serial));
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
    power_level += serial;
    power_level *= rack_id;

    // set value
    let value = (power_level / 100) % 10; // dividing ints should result in an int
    (value as i32) - 5
}

fn get_fuel_cell_value(cell:(usize, usize), step: usize, serial: usize) -> i32 {

    let (x, y) = cell;

    let mut total = 0;
    for i in x..x+step {
        for j in y..y+step {
           total += process_position((i, j), serial);
        }
    }

    total

}

pub fn get_hashmap_max_value(hmap: &HashMap<(usize, usize), i32>) -> (usize, usize) {
    // collect the max value in the hashmap
    let mut max_key= (0, 0);
    let mut max_value = -999_999_999;
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
    let pos = create_positions_for_grid(3, serial);
    let result = get_hashmap_max_value(&pos);
    // println!("Result Part1: ({}, {}) -> {}", result.0 -1, result.1-1, pos[&result]);
    (result.0, result.1)
}

pub fn part_b() -> (usize, usize, usize) {
    let serial = 1955;

    let mut return_value = (0, 0, 0);

    // pattern appears to exist where the squares total converges at a max point.
    // Once max is reached it will lower its values.
    let mut current_max = -999_999_999;

    for index in 1..300 {
        let pos = create_positions_for_grid(index, serial);
        let result = get_hashmap_max_value(&pos);

        let total = pos[&result];
        if total > current_max {
            current_max = total;
            return_value = (result.0, result.1, index);
        }
        else {
            return return_value;
        }
    }

    return_value
}

#[test]
fn test_part_b () {
    println!("{:?}", part_b());
}
#[test]
fn test_process_position_w_8() {
    let result = process_position((3, 5), 8);
    assert_eq!(result, 4);
}

#[test]
fn test_process_position_w_57() {
    let result = process_position((122, 79), 57);
    assert_eq!(result, -5);
}

#[test]
fn test_process_position_w_71() {
    let result = process_position((101, 153), 71);
    assert_eq!(result, 4);
}

#[test]
fn test_fuel_cell() {
    let serial = 18;
    let pos = create_positions_for_grid(3, serial);
    let result = get_hashmap_max_value(&pos);
    // println!("Result: ({}, {}) -> {}", result.0 -1, result.1-1, pos[&result]);
    assert_eq!(result, (33,45));
}

//#[test]
//fn test_fuel_cell_42() {
//    let serial = 42;
//    let pos = create_positions_for_grid(3, serial);
//    let result = get_hashmap_max_value(&pos);
//    println!("Result: ({}, {}) -> {}", result.0 -1, result.1-1, pos[&result]);
//}




