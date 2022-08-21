use colored::Colorize;
use rand::Rng;

 

// Core
#[derive(PartialEq, Clone, Copy)]
pub enum CellState {
    Alive,
    Dead
}

pub struct ConwayMatrix {
    matrix: Vec<Vec<ConwayCell>>
}

pub struct ConwayCell {
    current_state: CellState,
    next_state: CellState,
    neighbors: u8
}

impl ConwayCell {
    fn set_next(&mut self, next_cell_state: CellState) {
        self.next_state = next_cell_state;
    }
}

impl ConwayMatrix {
    pub fn new(size: usize) -> Self {
        let mut matrix = vec![];
        for _ in 0..size {
            let mut row: Vec<ConwayCell> = vec![];
            for _ in 0..size {
                row.push(ConwayCell { current_state: CellState::Dead, next_state: CellState::Dead, neighbors: 0 });
            }
            matrix.push(row);
        }

        Self { matrix }
    }

    pub fn size(&self) -> usize {
        self.matrix.len()
    }

    pub fn tick(&mut self) {
        // set next states
        for y in 0..self.size() {
            for x in 0..self.size()  {
                let is_alive = self.matrix[y][x].current_state == CellState::Alive;
                let neighbors = self.num_neighbors_for_cell(x as i8, y as i8);
                let mut next_cell_state = CellState::Dead;
                if is_alive && (neighbors == 2 || neighbors == 3) {
                    next_cell_state = CellState::Alive;
                } else if is_alive && neighbors < 2 {
                    next_cell_state = CellState::Dead;
                } else if is_alive && neighbors > 3 {
                    next_cell_state = CellState::Dead;
                } else if !is_alive && neighbors == 3 {
                    next_cell_state = CellState::Alive;
                }
                let cell = &mut self.matrix[y][x];
                cell.set_next(next_cell_state);
                cell.next_state = next_cell_state;
                cell.neighbors = neighbors;
            }
        }

        // tick each cell
        for y in 0..self.size() {
            for x in 0..self.size() {
                self.matrix[y][x].tick();
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut s = String::from("");
        for row in &self.matrix {
            for cell in row {
                if cell.current_state == CellState::Alive {
                    match cell.neighbors {
                        1 => s.push_str(&"■ ".purple().bold().to_string()),
                        2 => s.push_str(&"■ ".magenta().bold().to_string()),
                        3 => s.push_str(&"■ ".green().bold().to_string()),
                        4 => s.push_str(&"■ ".yellow().bold().to_string()),
                        5 => s.push_str(&"■ ".red().bold().to_string()),
                        _ => s.push_str(&"■ ".white().bold().to_string())
                    }
                } else {
                    s.push_str("O ");
                }
            }
            s.push('\n');
        }
        s
    }

    fn num_neighbors_for_cell(&self, x: i8, y: i8) -> u8 {
        let mut neighbors = 0;
        for nx in x-1..x+2 {
            for ny in y-1..y+2 {
                if (nx > -1 && nx < self.size() as i8) && (ny > -1 && ny < self.size() as i8) && !(nx == x && ny == y) && (self.matrix[ny as usize][nx as usize].current_state == CellState::Alive) {
                    neighbors += 1;
                }
            }
        }
        return neighbors;
    }
}

impl ConwayCell {
    fn tick(&mut self) {
        self.current_state = self.next_state
    }
}


// Debug
pub fn put_glider_in_matrix(matrix: &mut ConwayMatrix) {
    let x_offset = rand::thread_rng().gen_range(0..matrix.size() / 2);
    let y_offset = rand::thread_rng().gen_range(0..matrix.size() / 2);
    matrix.matrix[3+y_offset][1+x_offset].current_state = CellState::Alive;
    matrix.matrix[4+y_offset][2+x_offset].current_state = CellState::Alive;
    matrix.matrix[2+y_offset][3+x_offset].current_state = CellState::Alive;
    matrix.matrix[3+y_offset][3+x_offset].current_state = CellState::Alive;
    matrix.matrix[4+y_offset][3+x_offset].current_state = CellState::Alive;
}