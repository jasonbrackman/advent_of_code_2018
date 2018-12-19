use regex::Regex;

pub struct Point {
    x: i32,
    y: i32,
}

struct Board {
    points: Vec<(Point, Point)>
}

pub fn parse(input: &str) -> Vec<(Point, Point)> {
    let mut start = Vec::new();
    let re = Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>").unwrap();
    for line in input.lines() {
        for cap in re.captures_iter(line) {
            let x1 = cap[1].parse::<i32>().unwrap();
            let y1 = cap[2].parse::<i32>().unwrap();
            let x2 = cap[3].parse::<i32>().unwrap();
            let y2 = cap[4].parse::<i32>().unwrap();

            let position = Point{x: x1, y: y1};
            let velocity = Point{x: x2, y: y2};

            start.push((position, velocity));
        }
    }

    start
}

fn draw_position() {

}

fn next_position() {

}


#[test]
fn test_doit() {
    println!("Hello, world");
}