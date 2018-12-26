use regex::Regex;

pub struct Point {
    x: i32,
    y: i32,
}

pub struct Board {
    points: Vec<(Point, Point)>, // position point, velocity point
    lines: Vec<Vec<char>>, // the thing to print out
    compare: (usize, usize),
}

impl Board {
    pub fn new(input: &str) -> Self {
        let points = Board::parse(input);
        Board { points, lines:Vec::new(), compare:(100_000_000, 100_000_000) }
    }

    pub fn init_board(&mut self) -> bool {
        let (w_min, w_max, width) = self.get_width_min_max_total();
        let (h_min, h_max, height) = self.get_height_min_max_total();
        if self.compare.0 < width && self.compare.1 < height {
            println!("We likely have a winner in a previous round! ^ ");
            return false;
        }
        // setup a graph
        let mut lines: Vec<Vec<char>> = Vec::new();
        for _ in 0..height {
            let mut vertical: Vec<char> = Vec::new();
            for _ in 0..width {
                vertical.push('.');
            }
            lines.push(vertical);
        }
        self.lines = lines;
//        println!("Max Range W: {} + {} + 1 = {}", w_min, w_max, width);
//        println!("Max Range H: {} + {} + 1 = {}", h_min, h_max, height);

        // fill in database with default positions - let's normalize the negatives
        // --> For example, on x: -5 to 5 means 0 to 10 so add -5.abs() to x
        for (pos, _) in self.points.iter_mut() {
            let x = if width > w_max as usize { pos.x + w_min.abs() } else { pos.x - w_min.abs() };
            let y = if height > h_max as usize { pos.y + h_min.abs() } else { pos.y - h_min.abs() };
            pos.x = x;
            pos.y = y;
            self.lines[y as usize][x as usize] = 'x';
        }


        self.compare = (width, height);

        return true;
    }

    fn get_width_min_max_total(&mut self) -> (i32, i32, usize) {
        let values = self.points.iter().map(|(pos, _)| pos.x).collect::<Vec<i32>>();
        let max = *values.iter().max().unwrap();
        let min = *values.iter().min().unwrap();

        let mut result = max + min.abs() + 1;
        if max > 0 && min > 0 {
            result = max - min + 1;
        }
        (min, max, result as usize)
    }

    fn get_height_min_max_total(&mut self) -> (i32, i32, usize) {
        let values = self.points.iter().map(|(pos, _)| pos.y).collect::<Vec<i32>>();
        let max = *values.iter().max().unwrap();
        let min = *values.iter().min().unwrap();

        let mut result = max + min.abs() + 1;
        if max > 0 && min > 0 {
            result = max - min + 1;
        }
        (min, max, result as usize)
    }


    fn parse(input: &str) -> Vec<(Point, Point)> {
        let mut start = Vec::new();
        let re = Regex::new(r"position=<\s*(-?\d+), \s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>").unwrap();

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

    pub fn update(&mut self) {
        // let's normalize the negatives
        // --> For example, on x: -5 to 5 means 0 to 10 so add -5.abs() to x
        for (pos, vel) in self.points.iter_mut() {
            let x = pos.x + vel.x;
            let y = pos.y + vel.y;
            pos.x = x;
            pos.y = y;
        }
    }

    pub fn draw_position(&self) {

        let mut line = String::new();
        for x in self.lines.iter() {
            line.clear();
            for y in x.iter() {
                line.push(*y);
            }
            println!("-> {}", line);
        }

    }
}


#[test]
fn test_doit() {

    let path = "data/day_10.txt";
    let data = ::read(path);
    let mut board = Board::new(&data);

    board.init_board();
    let mut go = true;
    while go == true {
        for _ in 0..5 {
            board.draw_position();
            board.update();
            go = board.init_board()
        }
    }
}

