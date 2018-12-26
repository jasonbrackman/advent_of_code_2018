use std::collections::{HashMap};

pub fn parse(input: &str) -> (Vec<char>, HashMap<Vec<char>, char>) {
    let mut lines = input.lines();
    let data = lines.next().unwrap().split(' ').collect::<Vec<&str>>();

    let pots = data[2].chars().collect::<Vec<char>>();


    let mut rules = HashMap::new();
    for line in lines {
        let items = line.split(" => ").collect::<Vec<&str>>();
        if items.len() == 2 {
            rules
                .entry(String::from(items[0]).chars().collect::<Vec<char>>())
                .or_insert_with(||String::from(items[1]).chars().next().unwrap());
        }

    }

    (pots, rules)

}

pub fn process_pots(pots: &mut Vec<char>, rules: &HashMap<Vec<char>, char>, generations:i32) -> i32 {

    let mut negatives: i32 = 0;
    let mut pots_old = pots.clone();
    let mut pots_new = Vec::new();

    let mut total = 0;

    for _ in 0..generations {
        total = 0;
        pots_new.clear();

        // take care of pots to the left
        while pots_old.iter().position(|&r| r == '#').expect("Checking the start.") < 3 {
            pots_old.insert(0, '.');
            negatives -= 1;
        };
        negatives += 2;
        // take care of pots to teh right
        if pots_old[pots_old.len()-1] == '#' { pots_old.push('.')};
        if pots_old[pots_old.len()-2] == '#' { pots_old.push('.')};
        if pots_old[pots_old.len()-3] == '#' { pots_old.push('.')};
        if pots_old[pots_old.len()-4] == '#' { pots_old.push('.')};

        // CORE of the work
        for x in pots_old.windows(5) {
            let item = if rules.contains_key(x) { *rules.get(x).unwrap() } else { '.' };
            // let item = *rules.get(x).expect("Rule not found!");
            pots_new.push(item);
        }

        for (index, pot) in pots_new.iter().enumerate() {
            if *pot == '#' {
                total += index as i32 + negatives;;
            }
        }

        let mut s = String::new();
        for c in pots_new.iter() { s.push(*c) }
        // println!("GEN: [{:2}] T:[{:4}] N:[{:3}] -> {}", index+1, total, negatives, s);
        pots_old.clear();
        pots_old = pots_new.clone();
    }

    total as i32

}

#[test]
fn test_day_12_test_data() {
    let path = "data/day_12_test.txt";
    let data = ::read(path);

    let (mut pots, rules) = parse(&data);
    // println!("{}", rules.len());
    let result = process_pots(&mut pots, &rules, 20);
    assert_eq!(result, 325);
}


#[test]
fn test_day_12_push_forward_time() {
    let path = "data/day_12.txt";
    let data = ::read(path);

    let (mut pots, rules) = parse(&data);
    // at some point the pots stabilize and growth is now happening in a consistent manner.
    // sooo --- multiply the growth rate by the number of iterations left to go...
    let result = process_pots(&mut pots, &rules, 1_000);
    println!("R1: {}", result);
    println!("R1a:{}", result as i64 + (86 * (50_000_000_000-1_000)));


}


