use regex::Regex;

pub struct Point {
    x: i32,
    y: i32,
}

pub struct Board {
    points: Vec<(Point, Point)>, // position point, velocity point
    width: usize,
    height: usize,
    items: [ [char; 9]; 11]
}

impl Board {
    pub fn new(input: &str) -> Self {
        let points = Board::parse(input);

        let w = points.iter().map(|(pos, vel)| pos.x).collect::<Vec<i32>>();
        let w_max = w.iter().max().unwrap();
        let w_min = w.iter().min().unwrap();
        let width = (w_max + w_min.abs() + 1) as usize;

        let h = points.iter().map(|(pos, vel)| pos.y).collect::<Vec<i32>>();
        let h_max = h.iter().max().unwrap();
        let h_min = h.iter().min().unwrap();
        let height = (h_max + h_min.abs() + 1) as usize;

        let mut items = [ ['.'; 9]; 11];
        for x in 0..items.len() {
            for y in 0..items[x].len() {
                items[x][y] = '.';
            }
        }

        Board { points, width, height, items }
    }

    fn parse(input: &str) -> Vec<(Point, Point)> {
        let mut start = Vec::new();
        let re = Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s(-?\d+),\s*(-?\d+)>").unwrap();
        for line in input.lines() {
            for cap in re.captures_iter(line) {

                let position = Point{
                    x: cap[1].parse::<i32>().expect("Missing pos.x"),
                    y: cap[2].parse::<i32>().expect("Missing pos.y")
                };

                let velocity = Point{
                    x: cap[3].parse::<i32>().expect("Missing vel.x"),
                    y: cap[4].parse::<i32>().expect("Missing vel.y")
                };

                start.push((position, velocity));
            }
        }

        start
    }

    fn draw_position(&self) {

        for x in self.items.iter() {

        }
        println!("{:?}", self.items);
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

