use std::collections::HashSet;

pub fn counter_of_text(input: &str) -> (i32, i32) {

    let numbers: Vec<i32> = input
        .lines()
        .map(|m| m.parse::<i32>().unwrap()).collect();

    let part_a = numbers.iter().sum();

    let mut total = 0;
    let mut seen = HashSet::new(); // changing this from Vec to Set
                                                           // improved speed in release from
                                                           // 2 seconds to 0.019!

    for num in numbers.iter().cycle() {
        total += num;
        if seen.contains(&total) {
            return (part_a, total);
        }
        seen.insert(total);
    }

    (0, 0)

}