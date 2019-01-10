use std::collections::HashMap;
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

struct Board {
    cells: Vec<Vec<char>>,
    paths: HashMap<(usize, usize), Vec<(usize, usize)>>
}

impl Board {
    fn new(input: &str) -> Board {
        let cells = Board::populate_cells(input);
        let paths = Board::calculate_paths(&cells);
        Board{cells, paths}
    }

    fn populate_cells(input: &str) -> Vec<Vec<char>> {
        let mut cells = Vec::new();
        for line in input.lines() {
            let cols: Vec<char> = line.chars().collect();
            cells.push(cols);
        }

        cells
    }

    fn calculate_paths(cells: &Vec<Vec<char>>) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
        let mut hmap: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

        for x in 0..cells.len() {
            for y in 0..cells[x].len() {
                // find valid children
                let mut collection = Vec::new();

                for i in x-1..=x+1 {
                    for j in y-1..=y+1{
                        if i > 0 && j > 0 {

                        }
                    }
                }
                hmap.entry((x, y)).or_insert(collection);
            }
        }

        hmap
    }

    fn find_valid_children(&self, from: (usize, usize)) {

    }

    fn display(&self) {
        for x in 0..self.cells.len() {
            let mut buffer = String::new();
            for y in 0..self.cells[x].len() {
                buffer.push(self.cells[x][y]);
                println!("({},{})", x, y);
            }
            println!("{}", buffer);
        }
    }
}

#[test]
fn test_scan_actors_in_read_order() {
    let path = "data/day_15_test_scan.txt";
    let data = ::read(path);
    let board = Board::new(&data);
    board.display();
}



