struct Marbles {
    values: Vec<i32>,
    turn: usize,
    index: usize,
    players: Vec<i32>
}

impl Marbles {
    fn rules(&mut self, value: usize) {
        // 1. move to position between 1 & 2 marbles to the right (the marbles are in a circle)
        // 2. if the marble is a multiple of 23
        //      --> player keeps the marble and adds to their score.
        //      --> the marble 7 counter clockwise away is removed from the circle and added to their score
        //      --> the marble clockwise to the marble removed is now the 'current marble'
        if value % 23 == 0 {
            self.players[self.turn & self.players.len()];
        }

    }
}