use regex::Regex;

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

    let x = Rule{
        id: String::from(items[0]).trim_start_matches('#').parse::<usize>().unwrap(),
        from_left: direction[0].parse::<usize>().unwrap(),
        from_top: direction[1].trim_end_matches(':').parse::<usize>().unwrap(),
        width: width_height[0].parse::<usize>().unwrap(),
        height: width_height[1].parse::<usize>().unwrap(),
    };
    x
}

pub fn doit(input: &str) -> (i32, usize){
    let mut cells =  [ [0; 1000]; 1000 ];

    for line in input.lines() {
        // println!("{}", line);
        let r = parse_line_to_components(line);

        for i in r.from_left..r.from_left + r.width {
            for j in r.from_top..r.from_top + r.height {
                // println!("{}{}", i, j);
                cells[i][j] += 1;
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

    // println!("Total Squares with 2 or more: {}", total);

    // part 2
    let mut part_b = 0;
    for line in input.lines() {
        let mut is_good = true;
        let r = parse_line_to_components(line);

        for i in r.from_left..r.from_left + r.width {
            for j in r.from_top..r.from_top + r.height {

                if cells[i][j] > 1 {
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

#[test]
fn test_regex() {
    let input = "#1 @ 151,671: 11x15";
    let mut items = Vec::new();
    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    for cap in re.captures_iter(input) {
        items = vec!(
            cap[1].parse::<i32>().unwrap(),
            cap[2].parse::<i32>().unwrap(),
            cap[3].parse::<i32>().unwrap(),
            cap[4].parse::<i32>().unwrap(),
            cap[5].parse::<i32>().unwrap()
        );
    }

    for item in items.iter() {
        println!("{:?}", item);
    }
}

