#[derive(Default, Debug)]
struct Marbles {
    values: Vec<usize>,
    turn: usize,
    index: i64,
    players: Vec<i64>,
    twist:bool
}

impl Marbles {
    fn insert(&mut self) {
        self.turn += 1;

        if self.twist {
            // this shaved off 2s from doing the remove. on a x10 structure to 36s from 38s.
            // second run was 27s a full 11s faster.
            let current = self.index as usize;

            self.index += 1;
            let length = self.values.len() as i64;
            if self.index >= length {
                self.index -= length;
            }
            let plus_one = self.index as usize;

            self.index += 1;
            let length = self.values.len() as i64;
            if self.index >= length {
                self.index -= length;
            }
            let plus_two = self.index as usize;

            self.values[current] = self.turn;
            self.values.swap(current, plus_one);
            self.values.swap(plus_one, plus_two);

            self.twist = false;

        } else {
            if self.turn % 23 == 0 {
                self.index -= 7;
                if self.index < 0 {
                    self.index += self.values.len() as i64;
                }
                let player = (0..self.players.len()).cycle().nth(self.turn).expect("what?");

                self.players[player] += self.turn as i64;
                self.players[player] += self.values[self.index as usize] as i64;
                self.twist = true;
                //self.values.remove(self.index as usize); // 38s
            } else {
                self.index += 2;
                let length = self.values.len() as i64;
                if self.index >= length {
                    self.index -= length;
                }

                self.values.insert(self.index as usize, self.turn);
                self.twist = false;
            }
        }
    }
}


pub fn part_a(players: usize, max: i32) -> i64 {

    let mut marbles = Marbles::default();
    marbles.turn = 0;
    marbles.players = vec![0;players];
    marbles.values.push(0);
    for x in 0..=max {
        if x % 300_000 == 0 {
            println!("At: {}", x);
        }
        marbles.insert();
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

//#[test]
//fn test_place_marbles() {
//    let players = 9;
//    let max = 25;
//    let mut marbles = Marbles::default();
//    marbles.turn = 0;
//    marbles.players = vec![0; players];
//    marbles.values.push(0);
//    for x in 0..=max {
//        marbles.insert();
//        println!("[{}] {:?}", x, marbles);
////        if marbles.players.iter().max() == Some(&32i64) {
////            println!("======================");
////            println!("[{}] {:?}", x, marbles);
////            break;
////        }
//    }
//
//
//}

#[test]
fn test_place_marbles_2() {
    assert_eq!(part_a(10, 1618), 8317);
    assert_eq!(part_a(13, 7999), 146373);
    assert_eq!(part_a(17, 1104), 2764);
    assert_eq!(part_a(21, 6111), 54718);
    assert_eq!(part_a(30, 5807), 37305);

}


