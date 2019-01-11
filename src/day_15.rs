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
    paths: HashMap<(usize, usize), Vec<(usize, usize)>>,
    visited: Vec<(usize, usize)>
}

impl Board {
    fn new(input: &str) -> Board {
        let cells = Board::populate_cells(input);
        let paths = Board::calculate_paths(&cells);
        Board{cells, paths, visited:Vec::new()}
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

                let x_min = if x > 0 { x-1 } else { x };
                let x_max= if x == cells.len()-1 { x } else { x + 1 };

                let y_min = if y > 0 { y-1 } else { y };
                let y_max= if y == cells[x].len()-1 { y } else { y + 1 };

                for i in x_min..=x_max {
                    for j in y_min..=y_max {
                        match cells[i][j] {
                            '.' | 'G' | 'E' => collection.push((i, j)),
                            _ => (),
                        }
                    }
                }

                hmap.entry((x, y)).or_insert(collection);
            }
        }

        hmap
    }

    fn calculate_shortest_path(&mut self, from: (usize, usize), to: (usize, usize)) {
        if self.visited.contains(&from) { return }

        let mut neighbours: Vec<(usize, usize)> = self.paths.get(&from).unwrap().to_vec();
        println!("{:?}", neighbours);

        for neighbour in neighbours.iter() {
            if *neighbour == to {
                println!("Found A Path to Destination!");
                break
            }
            else {
                self.visited.push(*neighbour);
                self.calculate_shortest_path(*neighbour, to);
            }
        }
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
    // board.display();
    board.calculate_shortest_path((2,4), (3,3));
}







