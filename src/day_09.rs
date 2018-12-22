use std::collections::VecDeque;

#[derive(Default, Debug)]
struct Game {
    marbles: VecDeque<usize>,
    index: i64,
    players: Vec<i64>,
    twist:bool
}

impl Game {
    // takes 6338s
    fn insert(&mut self, turn: usize) {
        if self.twist {

            let current = self.index as usize;

            self.index += 1;
            let length = self.marbles.len() as i64;
            if self.index >= length {
                self.index -= length;
            }
            let plus_one = self.index as usize;

            self.index += 1;
            let length = self.marbles.len() as i64;
            if self.index >= length {
                self.index -= length;
            }
            let plus_two = self.index as usize;

            self.marbles[current] = turn;
            self.marbles.swap(current, plus_one);
            self.marbles.swap(plus_one, plus_two);

            self.twist = false;

        } else {
            if turn % 23 == 0 {
                self.index -= 7;
                if self.index < 0 {
                    self.index += self.marbles.len() as i64;
                }
                let player = (0..self.players.len()).cycle().nth(turn).expect("what?");

                self.players[player] += turn as i64;
                self.players[player] += self.marbles[self.index as usize] as i64;
                self.twist = true;

            } else {
                self.index += 2;
                let length = self.marbles.len() as i64;
                if self.index >= length {
                    self.index -= length;
                }

                self.marbles.insert(self.index as usize, turn);
                self.twist = false;
            }
        }
    }

    // takes 386s
    fn insert2(&mut self, marble: usize) {
        if marble % 23 == 0 {
            for _ in 0..7 {
                let popped = self.marbles.pop_back().unwrap();
                self.marbles.push_front(popped);
            }

            let player = (0..self.players.len()).cycle().nth(marble).expect("what?");

            let new_value = marble + self.marbles.pop_back().unwrap();
            self.players[player] += new_value as i64;

            let popped = self.marbles.pop_front().unwrap();
            self.marbles.push_back(popped);
        } else {

            let popped = self.marbles.pop_front().unwrap();
            self.marbles.push_back(popped);
            self.marbles.push_back(marble);
        }
    }
}


pub fn part_a(players: usize, max: usize) -> i64 {

    let mut marbles = Game::default();
    marbles.players = vec![0;players];

    marbles.marbles.push_back(0);
    for turn in 1..=max {
        if turn > 0 && turn % 500_000 == 0 {
            println!("At: {}", turn);
        }
        marbles.insert2(turn);
    }

    *marbles.players.iter().max().unwrap()

}
#[test]
fn test_getting_position_value() {
    let players= vec![1i32, 2, 3, 4];
    fn get_marble(position: usize, offset: usize, players: &Vec<i32>) {
        // println!("{:?}", players.iter().cycle().nth(position + offset - 1)); // [players.len() % 12]);
    }
    get_marble(3, 0, &players);
    get_marble(6, 0, &players);
}

#[test]
fn test_place_marbles_2() {
    assert_eq!(part_a(9, 25), 32);
    assert_eq!(part_a(9, 50), 63);
    assert_eq!(part_a(10, 1618), 8317);
    assert_eq!(part_a(13, 7999), 146373);
    assert_eq!(part_a(17, 1104), 2764);
    assert_eq!(part_a(21, 6111), 54718);
    assert_eq!(part_a(30, 5807), 37305);

}


