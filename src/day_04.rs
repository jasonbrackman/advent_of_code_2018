use regex::Regex;
use std::collections::{HashMap};

pub fn part_a(hmap: &mut HashMap<String, Vec<i32>>) -> i32{
    // find greatest amount of sleep minutes * the minute most often asleep.
    // 3203 * 44

    // get the guard
    let mut most_sleep = 0;
    let mut guard = "";
    for (k, v) in hmap.iter() {
        let sleep = v.iter().fold(0, |acc, val| acc + val);
        most_sleep = if sleep > most_sleep {
            guard = k;
            sleep
        } else { most_sleep };
    }

    // get the minute
    let mut the_minute = 0;
    let mut most_minutes = 0;
    for (k, v) in hmap.iter() {

        if k == guard {
            for (index, x) in v.iter().enumerate() {
                if x > &most_minutes {
                    most_minutes = *x;
                    the_minute = index;
                }
                // println!("{}-{}", index, x);
            }
        }
    }

    let mut guard_id = 0;
    let re_guard = Regex::new(r"Guard #(\d+).+").unwrap();
    for cap in re_guard.captures_iter(guard) {
       guard_id = cap[1].parse::<i32>().unwrap();

    }
    assert_eq!(the_minute, 44);

    guard_id * the_minute as i32
}

pub fn create_timesheet(items: Vec<Vec<String>>) -> HashMap<String, Vec<i32>> {
    let re_guard = Regex::new(r"Guard #(\d+).+").unwrap();

    let mut hmap = HashMap::new();
    let mut guard = "";
    let mut dirty = false;
    let mut sleep = 0;
    let mut awake = 0;
    for item in items.iter() {
        for cap in re_guard.captures_iter(&item[4]) {
            if cap.len() == 2 {
                guard = &item[4];
            }
        }

        if item[4].contains("asleep") {
            if item[2].parse::<i32>().unwrap() == 0 {
                sleep = item[3].parse::<i32>().unwrap();
            } else {
                sleep = 0;
            }
        }
        if item[4].contains("wakes") {
            if item[2].parse::<i32>().unwrap() == 0 {
                awake = item[3].parse::<i32>().unwrap();
            } else {
                awake = 0;
            }
            dirty = true;
        }

        /*
        ["04", "01", "00", "00", "Guard #3167 begins shift"]
        ["04", "01", "00", "53", "falls asleep"]
        ["04", "01", "00", "54", "wakes up"]
        */
        if dirty == true {
            // println!("{} - [{}][{}] {} -> {}", guard, month, day, sleep, awake);
            for i in sleep..awake {
                let mut temp = hmap.entry(guard.to_string()).or_insert(vec!(0; 60));
                temp[i as usize] += 1;
            }
            dirty = false;
        }
    }
    hmap
}

pub fn part_b(hmap: &mut HashMap<String, Vec<i32>>) -> i32 {
    // now find out which guard has the highest minute of sleep overall
    // multiply the badge id of the guard to the minute most slept.



    let mut minuteb = 0;
    let mut guardb = "";
    let mut countb = 0;
    for (k, v) in hmap.iter_mut() {
        for (index, num) in v.iter().enumerate() {
            if num > &countb {
                countb = *num;
                minuteb = index;
                guardb = k;
            }
        }
    }
    let mut guard_id = 0;
    let re_guard = Regex::new(r"Guard #(\d+).+").unwrap();
    for cap in re_guard.captures_iter(guardb) {
        guard_id = cap[1].parse::<i32>().unwrap();

    }

    guard_id * minuteb as i32
}

pub fn get_sorted_items(input: &str) -> Vec<Vec<String>> {
    let mut items = Vec::new();
    let re = Regex::new(r"^\[(\d+)\-(\d+)\-(\d+) (\d+):(\d+)\] (.*)").unwrap();

    for line in input.lines() {
        for cap in re.captures_iter(line) {
            items.push(
                vec!(
                    // cap[1].parse::<i32>().unwrap() ,
                    cap[2].parse::<String>().unwrap(),
                    cap[3].parse::<String>().unwrap(),
                    cap[4].parse::<String>().unwrap(),
                    cap[5].parse::<String>().unwrap(),
                    cap[6].parse::<String>().unwrap())
            );
        }
    }
    // sort the items
    items.sort();

    items
}


#[test]
fn test_doit() {
    assert_eq!(doit("asdf1\nasdf2"), 0)
}

