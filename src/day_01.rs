
pub fn counter_of_text(input: &str) -> (i32, i32) {

    let numbers: Vec<i32> = input.trim_right().split("\n").map(|s| s.parse::<i32>().unwrap()).collect();
    let part_a = numbers.iter().sum();

    let mut total = 0;
    let mut seen = Vec::new();

    let not_found = true;
    while not_found {
        for num in numbers.iter() {
            total += num;
            if seen.contains(&total) {
                return (part_a, total);
            }
            seen.push(total);
        }
    }

    (0, 0)

}