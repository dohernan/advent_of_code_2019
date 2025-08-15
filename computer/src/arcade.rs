use crate::Computer;
use std::{time,cell::RefCell, thread::sleep};

#[derive(Default, Clone, Copy, PartialEq)]
enum Tile {
    #[default]
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl From<i64> for Tile {
    fn from(value: i64) -> Self {
        match value {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => {
                println!("Tile not supported");
                Tile::Empty
            }
        }
    }
}

impl Tile {
    fn print(&self) {
        let tile = match self {
            Tile::Ball => 'o',
            Tile::Block => '#',
            Tile::Paddle => '_',
            Tile::Wall => '|',
            _ => ' ',
        };
        print!("{}", tile);
    }
}

type Grid = Vec<Vec<RefCell<Tile>>>;

#[derive(Default)]
struct Arcade {
    position_x: usize,
    position_y: usize,
    grid: Grid,
}

impl Arcade {
    fn default() -> Self {
        let grid: Vec<Vec<RefCell<Tile>>> = vec![vec![RefCell::new(Tile::default()); 42]; 24];
        Arcade {
            position_x: 0,
            position_y: 0,
            grid,
        }
    }
    pub fn print_grid(&self) {
        for row in &self.grid {
            for cell in row {
                cell.borrow().print();
            }
            println!();
        }
    }

    fn move_to(&mut self, x: i64, y: i64) {
        (self.position_x, self.position_y) = (x as usize, y as usize);
    }

    pub fn set_tile(&mut self, tile: i64) {
        (*self.grid[self.position_y][self.position_x].borrow_mut()) = Tile::from(tile);
    }
    pub fn set_tile_at(&mut self, x: i64, y: i64, tile: i64) {
        self.move_to(x, y);
        self.set_tile(tile);
    }
    pub fn how_many_block(&self) -> usize {
        // self.grid
        //     .iter()
        //     .map(|row| row.iter().map(|cell| (*cell.borrow()).painted).map(|value| 1 if value))
        //     .sum()
        let mut total = 0;
        for row in &self.grid {
            for column in row {
                if *column.borrow() == Tile::Block {
                    total += 1;
                }
            }
        }
        total
    }
}

#[derive(Default)]
pub struct Scenario {
    computer: Computer,
    arcade: Arcade,
}

impl Scenario {
    pub fn new(memory: Vec<i64>) -> Self {
        let mut computer = Computer::new(memory);
        computer.is_automatic_input = true;
        Scenario {
            computer,
            arcade: Arcade::default(),
        }
    }
    pub fn execute_scenario(&mut self) -> usize {
        let mut run_scenario = true;
        println!("Insert 2 coins: ");
        let mut input_instruction = 0;
        let mut paddle_pos = 0;
        while run_scenario {
            let terminate = self.computer.process(input_instruction, true);
            if terminate {
                break;
            }
            let x: i64 = self.computer.get_output();
            self.computer.process(input_instruction, true);
            let y = self.computer.get_output();
            self.computer.process(input_instruction, true);
            let tile = self.computer.get_output();
            if tile == 4 {
                let ball_pos = x;
                if ball_pos > paddle_pos {
                    input_instruction = 1;
                } else if ball_pos < paddle_pos {
                    input_instruction = -1;
                } else {
                    input_instruction = 0;
                }
            }

            if tile == 3 {
                paddle_pos = x;
            }
            if x == -1 {
                println!("Score: {}", tile);
                self.arcade.print_grid();
                sleep(time::Duration::from_millis(100));
            } else {
                let _ = self.arcade.set_tile_at(x, y, tile);
            }

            run_scenario = !self.computer.is_finished();
        }
        self.arcade.how_many_block()
    }

    pub fn print_grid(&self) {
        self.arcade.print_grid();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_integers() {
        let mut arcade = Arcade::default();
        let _ = arcade.set_tile_at(1, 2, 3);
        let _ = arcade.set_tile_at(6, 5, 4);
        arcade.print_grid();
        assert_eq!(arcade.how_many_block(), 0);
    }
}
