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
    cell_neighbours: HashMap<(usize, usize), Vec<(usize, usize)>>,
}

impl Board {
    fn new(input: &str) -> Board {
        let cells = Board::populate_cells(input);
        let cell_neighbours = Board::calculate_paths(&cells);
        Board{cells, cell_neighbours }
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
        // First attempt -- only showed me if the path was possible -- but not keeping
        // track of the actual shorest path.

        let mut visited = Vec::new();
        let mut frontier = VecDeque::new();
        frontier.insert(0, from);

        while !frontier.is_empty() {
            let current = frontier.pop_front().unwrap();

            if current == to {
                println!("Found A Path to Destination! => {:?}", visited);
                break
            }
            else {
                if !visited.contains(&current) {
                    println!("Checking neighbours... ");
                    frontier.extend(self.cell_neighbours.get(&current).unwrap().to_vec());
                }
            }

            visited.push(current);
        }

        println!("Unable to find a path to destination.");
    }

    fn calculate_shortest_path2(&mut self, from: (usize, usize), to: (usize, usize)) -> HashMap<(usize, usize), (usize, usize)> {
        // keeping track of the parents which I can use to rebuild the path --- but have not
        // proven to myself that it is the shortest path yet.

        let mut frontier = VecDeque::new();
        frontier.insert(0, from);

        let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        came_from.entry(from).or_insert((999, 999));

        while !frontier.is_empty() {
            let current = frontier.pop_front().unwrap();
            for next in self.cell_neighbours.get(&current).unwrap().to_vec() {
                if !came_from.contains_key(&next) {
                    frontier.push_back(next);
                    came_from.entry(next).or_insert(current);
                }
            }
        }
        return came_from;
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
    let mut buffer = Vec::new();
    let to = (1, 5);
    let results: HashMap<(usize, usize), (usize, usize)> = board.calculate_shortest_path2((4,1), to);
    let mut current = &to;
    while !(current == &(999, 999) || !results.contains_key(&current)) {
        buffer.push(current.clone());
        current = &results[&current];
    }
    println!("Shortest Path: {:?}", buffer);
}







