use regex::Regex;

pub struct Point {
    x: i32,
    y: i32,
}

pub struct Board {
    points: Vec<(Point, Point)>, // position point, velocity point
    width: usize,
    height: usize,
}

impl Board {
    pub fn new(input: &str) -> Self {
        let points = Board::parse(input);
        let width: i32 = *points.iter().map(|(pos, vel)| pos.x).collect::<Vec<i32>>().iter().max().unwrap();
        let height: i32 = *points.iter().map(|(pos, vel)| pos.y).collect::<Vec<i32>>().iter().max().unwrap();

        Board { points, width: width as usize, height: height as usize }
    }

    fn parse(input: &str) -> Vec<(Point, Point)> {
        let mut start = Vec::new();
        let re = Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s(-?\d+),\s*(-?\d+)>").unwrap();
        for line in input.lines() {
            for cap in re.captures_iter(line) {
                let x1 = cap[1].parse::<i32>().expect("Missing pos.x");
                let y1 = cap[2].parse::<i32>().expect("Missing pos.y");
                let x2 = cap[3].parse::<i32>().expect("Missing vel.x");
                let y2 = cap[4].parse::<i32>().expect("Missing vel.y");

                let position = Point{x: x1, y: y1};
                let velocity = Point{x: x2, y: y2};

                start.push((position, velocity));
            }
        }

        start
    }

    fn draw_position(&self) {

        let w = self.width;
        let h = self.height;
        let mut display = [['.'; 20]; 20];
        for (pos, vel) in self.points.iter() {
            display[pos.x as usize][pos.y as usize] = '#';
        }

        println!("{:?}", display);
    }
}


#[test]
fn test_doit() {

    let path = "data/day_10_test.txt";
    let data = ::read(path);
    let board = Board::new(&data);
    board.draw_position();

    println!("Hello, world");
}

