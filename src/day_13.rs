
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

    fn tick(&mut self) -> Option<(usize, usize)> {
        let debug = false;

        let mut old_data = self.carts.iter().map(|x| x.pos).collect::<Vec<(usize, usize)>>();

        // ensure the order of carts always goes from top left to right then down
        self.carts.sort_by_key( |x| x.pos);

        for (index, cart) in self.carts.iter_mut().enumerate() {
            if debug {
                println!("Old Char: {}", self.tracks[cart.pos.0][cart.pos.1]);
                println!("Old Faceing -> {:?}", cart.facing);
            }

            let next_move = match cart.facing {
                Face::UP => (cart.pos.0 - 1, cart.pos.1),
                Face::DOWN => (cart.pos.0 + 1, cart.pos.1),
                Face::LEFT => (cart.pos.0, cart.pos.1 - 1),
                Face::RIGHT => (cart.pos.0, cart.pos.1 + 1),
            };

            if debug {
                println!("Old Pos: {:?}", cart.pos);
                println!("New Pos: {:?}", next_move);
            }

            if old_data.contains(&next_move) {
                return Some(next_move);
            }


            let next = self.tracks[next_move.0][next_move.1];

            let facing = match next {
                '-' => if cart.facing == Face::LEFT { Face::LEFT } else { Face::RIGHT },
                '|' => if cart.facing == Face::UP { Face::UP } else { Face::DOWN },
                '/' => match cart.facing {
                        Face::RIGHT => Face::UP,
                        Face::LEFT => Face::DOWN,
                        Face::UP => Face::RIGHT,
                        Face::DOWN => Face::LEFT,
                },
                '\\' => match cart.facing {
                        Face::RIGHT => Face::DOWN,
                        Face::LEFT => Face::UP,
                        Face::UP => Face::LEFT,
                        Face::DOWN => Face::RIGHT,
                },
                '+' => match cart.intersections {
                    Move::LEFT => {

                        cart.intersections = Move::STRAIGHT;
                        match cart.facing {
                            Face::LEFT => Face::DOWN,
                            Face::RIGHT => Face::UP,
                            Face::UP => Face::LEFT,
                            Face::DOWN => Face::RIGHT,
                        }

                    },
                    Move::STRAIGHT => {
                        cart.intersections = Move::RIGHT;
                        cart.facing.clone()
                    },
                    Move::RIGHT => {
                        cart.intersections = Move::LEFT;
                        match cart.facing {
                            Face::LEFT => Face::UP,
                            Face::RIGHT => Face::DOWN,
                            Face::UP => Face::RIGHT,
                            Face::DOWN => Face::LEFT,
                        }
                    },
                }
                _ => unimplemented!("Something went wrong! {}", next),
            };

            if debug {
                println!("New Facing: {:?}", facing);
                println!("New Char: {}", next);
                println!("-------------------------------");
            }

            cart.pos = next_move;
            cart.facing = facing;
            old_data.push(next_move);
            old_data.remove(index);
        }

        None
    }
}

pub fn part_a() -> (usize, usize) {
    let path = "data/day_13.txt";
    let data = ::read(path);
    let mut board = Board::new(&data);

    for index in 0.. {
        // board.pprint();
        match board.tick() {
            Some(x) => {
                println!("Crash: {},{}", x.1, x.0);
                return (x.1, x.0);
            },
            _ => continue,
        }
    }
    return (0, 0);
}
#[test]
fn test_day_13_straight_line() {
    let path = "data/day_13_test_a.txt";
    let data = ::read(path);
    let mut board = Board::new(&data);
    board.pprint();
    for index in 0..10 {
        println!("Tick -> {}", index);
        match board.tick() {
            Some(x) => {
                println!("Crash: {:?}", x);
                break
            },
            _ => continue,
        }
    }

}

#[test]
fn test_day_13_tracks() {
    let path = "data/day_13_test_a.txt";
    let data = ::read(path);
    let mut board = Board::new(&data);

    for index in 0.. {
        board.pprint();
        // println!("Tick -> {}", index);
        match board.tick() {
            Some(x) => {
                println!("Crash: {:?}", x);
                break
            },
            _ => continue,
        }
    }
}


