
#[derive(Debug)]
enum Move {
    LEFT,
    STRAIGHT,
    RIGHT,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Face {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
struct Cart {
    facing: Face,
    intersections: Move,
    pos: (usize, usize),
}

impl Cart {
    fn new(piece: &char, pos:(usize, usize)) -> Cart {
        let facing = match piece {
            '>' => Face::RIGHT,
            '<' => Face::LEFT,
            'v' => Face::DOWN,
            '^' => Face::UP,
            _ => unimplemented!()
        };

        Cart{facing, intersections:Move::LEFT, pos}
    }

    fn set_cart_next_pos(&mut self) {
        let next_pos = match self.facing {
            Face::UP => (self.pos.0 - 1, self.pos.1),
            Face::DOWN => (self.pos.0 + 1, self.pos.1),
            Face::LEFT => (self.pos.0, self.pos.1 - 1),
            Face::RIGHT => (self.pos.0, self.pos.1 + 1),
        };

        self.pos = next_pos;
    }

    fn set_cart_next_facing(&mut self, next: &char) {

        let facing = match next {
            '-' => if self.facing == Face::LEFT { Face::LEFT } else { Face::RIGHT },
            '|' => if self.facing == Face::UP { Face::UP } else { Face::DOWN },
            '/' => match self.facing {
                Face::RIGHT => Face::UP,
                Face::LEFT => Face::DOWN,
                Face::UP => Face::RIGHT,
                Face::DOWN => Face::LEFT,
            },
            '\\' => match self.facing {
                Face::RIGHT => Face::DOWN,
                Face::LEFT => Face::UP,
                Face::UP => Face::LEFT,
                Face::DOWN => Face::RIGHT,
            },
            '+' => match self.intersections {
                Move::LEFT => {
                    self.intersections = Move::STRAIGHT;
                    match self.facing {
                        Face::LEFT => Face::DOWN,
                        Face::RIGHT => Face::UP,
                        Face::UP => Face::LEFT,
                        Face::DOWN => Face::RIGHT,
                    }
                },

                Move::STRAIGHT => {
                    self.intersections = Move::RIGHT;
                    self.facing.clone()
                }

                Move::RIGHT => {
                    self.intersections = Move::LEFT;
                    match self.facing {
                        Face::LEFT => Face::UP,
                        Face::RIGHT => Face::DOWN,
                        Face::UP => Face::RIGHT,
                        Face::DOWN => Face::LEFT,
                    }
                },
            }
            _ => unimplemented!("Something went wrong! {}", next),
        };

        self.facing = facing;
    }


}

struct Board {
    tracks: Vec<Vec<char>>,
    carts: Vec<Cart>
}

impl Board {
    pub fn new(data: &str) -> Board {
        let mut carts = vec![];

        let mut tracks = vec![];
        for line in data.lines() {
            tracks.push(line.chars().collect::<Vec<char>>());
        }

        for row in 0..tracks.len() {
            for col in 0..tracks[row].len() {
                let space = match tracks[row][col] {
                    '<' | '>' => {
                        carts.push(Cart::new(&tracks[row][col], (row, col)));
                        '-'
                    },
                    'v' | '^' => {
                        carts.push(Cart::new(&tracks[row][col], (row, col)));
                        '|'
                    },
                    _ => tracks[row][col]
                };

                tracks[row][col] = space;
            }
        }

        Board{ tracks, carts }
    }

    fn pprint(&self) {
        let mut temp = self.tracks.clone();
        for cart in self.carts.iter() {
            let (x, y) = cart.pos;
            temp[x][y] = match cart.facing {
                Face::UP => '^',
                Face::DOWN => 'v',
                Face::LEFT => '<',
                Face::RIGHT => '>',
            }
        };

        for row in 0..temp.len() {
            let mut buffer = String::new();
            buffer += &format!("{:2} ", row);
            for col in 0..temp[row].len() {
                buffer.push(temp[row][col]);
            }
            println!("{}", buffer);
        }
    }

    fn tick(&mut self) -> Vec<(usize, usize)> {
        let debug = false;

        let mut crash_position_order = Vec::new();
        let mut position_cache = self.carts.iter().map(|x| x.pos).collect::<Vec<(usize, usize)>>();

        // ensure the order of carts always goes from top left to right then down
        self.carts.sort_by_key( |x| x.pos);

        for (index, cart) in self.carts.iter_mut().enumerate() {
            if crash_position_order.contains(&cart.pos) {
                // skip out --
                // A tick covers all the carts -- but a crashed cart should not move which could be
                // encountered if the 2nd cart isn't checked for its crashed position.
                continue;
            }

            if debug {
                println!("Old Char: {}", self.tracks[cart.pos.0][cart.pos.1]);
                println!("Old Faceing -> {:?}", cart.facing);
                println!("Old Pos: {:?}", cart.pos);
            }

            cart.set_cart_next_pos();
            let next = self.tracks[cart.pos.0][cart.pos.1];
            cart.set_cart_next_facing(&next);

            if debug {
                println!("New Pos: {:?} ", cart.pos);
                println!("New Facing: {:?}", cart.facing);
                println!("New Char: {}", next);
                println!("-------------------------------");
            }

            if position_cache.contains(&cart.pos) {
                crash_position_order.push(cart.pos);
            }

            position_cache[index] = cart.pos;
        }

        crash_position_order
    }

}

pub fn part_a(data: &str) -> (usize, usize) {
    let mut board = Board::new(data);

    loop {
        let crashes = board.tick();
        if !crashes.is_empty() {
            let x = crashes.first().unwrap();
            return (x.1, x.0);
        }
    }
}

pub fn part_b(data: &str) -> (usize, usize) {
    let mut board = Board::new(data);
    loop {
        let crashes = board.tick();
        for crash in crashes.iter() {
            board.carts.retain(|x| x.pos != *crash);
        }
        if board.carts.len() == 1 {
            let result = board.carts[0].pos;
            return (result.1, result.0)
        }
    }
}

#[test]
fn test_day_13_straight_line() {
    let path = "data/day_13_test_a.txt";
    let data = ::read(path);
    let result = part_a(&data);
    assert_eq!(result, (0, 3));
}

#[test]
fn test_day_13_tracks() {
    let path = "data/day_13_test_b.txt";
    let data = ::read(path);
    let result = part_a(&data);
    assert_eq!(result, (7, 3));
}

#[test]
fn test_day_13_last_cart_position() {
    let path = "data/day_13_test_c.txt";
    let data = ::read(path);
    let result = part_b(&data);
    assert_eq!(result, (6, 4));
}



