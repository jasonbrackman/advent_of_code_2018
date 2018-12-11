// use regex::Regex;

struct Rule {
    id: usize,
    from_left: usize,
    from_top: usize,
    width: usize,
    height: usize
}

fn parse_line_to_components(input: &str) -> Rule {
    let items: Vec<&str> = input.split_whitespace().map(|s| s.trim_right_matches('.')).collect();
    let direction: Vec<&str> = items[2].split(',').collect();
    let width_height: Vec<&str> = items[3].split('x').collect();

    Rule{
        id: String::from(items[0]).trim_start_matches('#').parse::<usize>().unwrap(),
        from_left: direction[0].parse::<usize>().unwrap(),
        from_top: direction[1].trim_end_matches(':').parse::<usize>().unwrap(),
        width: width_height[0].parse::<usize>().unwrap(),
        height: width_height[1].parse::<usize>().unwrap(),
    }
}

pub fn doit(input: &str) -> (i32, usize){
    let mut cells =  vec![ [0; 1000]; 1000 ];

    for line in input.lines() {
        // println!("{}", line);
        let r = parse_line_to_components(line);

        for row in cells.iter_mut().skip(r.from_left).take(r.width) {
            for col in row.iter_mut().skip(r.from_top).take(r.height) {
                *col += 1;
            }
        }
    }

    let mut total = 0;

    for i in 0..cells.len() {
        for j in 0..cells.len() {
            if cells[i][j] >= 2 {
                total += 1;
            }
        }
    }

    // part 2
    // - determine if squares contain 2 ore more items
    let mut part_b = 0;

    for line in input.lines() {
        let mut is_good = true;
        let r = parse_line_to_components(line);

        for row in cells.iter().skip(r.from_left).take(r.width) {
            for col in row.iter().skip(r.from_top).take(r.height) {
                if *col > 1 {
                    is_good = false;
                }
            }
        }

        if is_good {
            part_b = r.id;
        }

    }

    (total, part_b)
}

//#[test]
//fn test_regex() {
//    let input = "#1 @ 151,671: 11x15";
//    let mut items = Vec::new();
//    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
//    for cap in re.captures_iter(input) {
//        items = vec!(
//            cap[1].parse::<i32>().unwrap(),
//            cap[2].parse::<i32>().unwrap(),
//            cap[3].parse::<i32>().unwrap(),
//            cap[4].parse::<i32>().unwrap(),
//            cap[5].parse::<i32>().unwrap()
//        );
//    }
//
//    for item in items.iter() {
//        println!("{:?}", item);
//    }
//}

