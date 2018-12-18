#[derive(Default, Debug)]
struct Marbles {
    values: Vec<usize>,
    turn: usize,
    index: i64,
    players: Vec<i32>
}

impl Marbles {
    fn insert(&mut self) {
        self.turn += 1;
        self.set_index();
        self.values.insert(self.index as usize, self.turn);
    }

    fn remove(&mut self) -> usize {
        0
    }

    fn get_player(&self) -> i32 {
        let mut x = *self.players.iter().cycle().nth(self.turn - 1).expect("what?");
        x += 1;


        println!("{}", x);
        2
    }

    fn set_index(&mut self) {
        // 1. move to position between 1 & 2 marbles to the right (the marbles are in a circle)
        // 2. if the marble is a multiple of 23
        //      --> player keeps the marble and adds to their score.
        //      --> the marble 7 counter clockwise away is removed from the circle and added to their score
        //      --> the marble clockwise to the marble removed is now the 'current marble'
        if self.turn % 23 == 0 {

            self.index -= 7;
            if self.index < 0 {
                self.index += self.values.len() as i64;

            }
            println!("Points for player: {}", self.get_player());
        } else {
            self.index += 2;
            if self.index > self.values.len() as i64 {
                self.index -= self.values.len() as i64;
            }
        }
    }
}

#[test]
fn test_getting_position_value() {
    let players= vec![1i32, 2, 3, 4];
    fn get_marble(position: usize, offset: usize, players: &Vec<i32>) {
        println!("{:?}", players.iter().cycle().nth(position + offset - 1)); // [players.len() % 12]);
    }

    get_marble(3, 0, &players);
    get_marble(6, 0, &players);
}

#[test]
fn test_place_marbles() {
    let mut marbles = Marbles::default();
    marbles.turn = 0;
    marbles.players = vec![0;9];
    marbles.values.push(0);
    for x in 0..=25 {
        marbles.insert();
        println!("[{}] {:?}", x, marbles);
    }


}