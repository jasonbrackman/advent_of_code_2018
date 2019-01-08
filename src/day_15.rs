// walls = '#'
// cavern = '.'
// goblin = 'G'
// elf = 'E'

struct Combat {}
impl Combat {
    fn round() {
        /*
        1. Identify all ENEMY TARGETS
        2. Identify all valid open squares ('.')
            -> Identify shortest path to enemy and identify the next square it should occupy.
        unit_takes_turn
            -> if not in RANGE {
                |-> Tries to MOVE into range of enemy
               } else {
                |-> ATTACK
               }
        * units only move DIAGNALLY AND HORIZONTALLY

        */
    }
}

struct Board {}

impl Board {
    fn new(input: &str) {
        let mut cells = Vec::new();

        for line in input.lines() {
            let cols: Vec<char> = line.chars().collect();
            cells.push(cols);
        }

        for x in 0..cells.len() {
            for y in 0..cells[x].len() {
                println!("({},{}) -> {}", x, y, cells[x][y]);
            }
        }

    }
}

#[test]
fn test_scan_actors_in_read_order() {
    let result = "#######
#.1.2.#
    #3.4.5#
    #.6.7.#
    #######";

    let board = Board::new(result);

}