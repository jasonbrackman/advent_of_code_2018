use std::collections::{HashMap, VecDeque};
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
    paths: HashMap<(usize, usize), Vec<(usize, usize)>>,
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

                let mut neighbours = Vec::new();

                // up first then left then right, then down
                if x > 0 { neighbours.push((x-1, y)) }
                if y > 0 { neighbours.push((x, y-1)) };
                if x < cells.len()-1 { neighbours.push((x + 1, y)) };
                if y < cells[x].len()-1 { neighbours.push((x, y+1)) };

                for (i, j) in neighbours.into_iter() {
                    match cells[i][j] {
                        '.' | 'G' | 'E' => collection.push((i, j)),
                        _ => (),
                    }
                }

                hmap.entry((x, y)).or_insert(collection);
            }
        }

        hmap
    }

    fn calculate_shortest_path(&mut self, from: (usize, usize), to: (usize, usize)) {
        let mut visited = Vec::new();
        let mut to_visit = VecDeque::new();

        to_visit.extend(self.paths.get(&from).unwrap().to_vec());

        while !to_visit.is_empty() {
            let neighbour = to_visit.pop_front().unwrap();

            if neighbour == to {
                println!("Found A Path to Destination! => {:?}", visited);
                break
            }
            else {
                if !visited.contains(&neighbour) {
                    println!("Checking neighbours... ");
                    to_visit.extend(self.paths.get(&neighbour).unwrap().to_vec());
                }
            }

            visited.push(neighbour);
        }

        println!("Unable to find a path to destination.");
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
    let mut board = Board::new(&data);
    board.display();

    // currenty just confirms there is a connection...
    board.calculate_shortest_path((2,2), (2,5));
}







