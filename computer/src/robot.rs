use std::cell::RefCell;

use crate::Computer;

#[derive(Default, Clone, Copy, PartialEq)]
enum Direction {
    #[default]
    North,
    West,
    South,
    East,
}

impl Direction {
    fn turn_left(&mut self) {
        *self = match *self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
    fn turn_right(&mut self) {
        *self = match *self {
            Direction::North => Direction::East,
            Direction::West => Direction::North,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
struct Panel {
    color: i64,
    painted: bool,
}

impl Panel {
    fn paint_color(&mut self, color: i64) {
        if color > 1 {
            panic!();
        }
        self.painted = true;

        self.color = color;
    }
    fn print(&self) {
        if self.color == 0 {
            print!(".");
        } else {
            print!("#");
        }
    }
}

type Grid = Vec<Vec<RefCell<Panel>>>;

#[derive(Default)]
struct Robot {
    position_x: usize,
    position_y: usize,
    facing: Direction,
    grid: Grid,
}

impl Robot {
    fn default() -> Self {
        let grid = vec![vec![RefCell::new(Panel::default()); 100]; 100];
        grid[50][50].borrow_mut().paint_color(1);
        Robot {
            position_x: 50,
            position_y: 50,
            facing: Direction::North,
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
    fn turn(&mut self, direction: i64) {
        match direction {
            0 => self.facing.turn_left(),
            1 => self.facing.turn_right(),
            _ => {
                println!("cheeeee")
            }
        }
    }
    fn advance(&mut self) {
        //println!("{}-{}", self.position_x, self.position_y);
        (self.position_x, self.position_y) = match self.facing {
            Direction::North => (self.position_x, self.position_y - 1),
            Direction::West => (self.position_x - 1, self.position_y),
            Direction::South => (self.position_x, self.position_y + 1),
            Direction::East => (self.position_x + 1, self.position_y),
        };
    }
    pub fn paint_color(&mut self, color: i64) {
        (*self.grid[self.position_y][self.position_x].borrow_mut()).paint_color(color);
    }
    pub fn paint_turn_and_advance(&mut self, color: i64, direction: i64) -> Result<(), String> {
        self.paint_color(color);
        self.turn(direction);
        self.advance();
        Ok(())
    }
    pub fn how_many_painted(&self) -> usize {
        // self.grid
        //     .iter()
        //     .map(|row| row.iter().map(|cell| (*cell.borrow()).painted).map(|value| 1 if value))
        //     .sum()
        let mut total = 0;
        for row in &self.grid {
            for column in row {
                if column.borrow().painted {
                    total += 1;
                }
            }
        }
        total
    }
    pub fn get_current_color(&self) -> i64 {
        self.grid[self.position_y][self.position_x].borrow().color
    }
}

#[derive(Default)]
pub struct Scenario {
    computer: Computer,
    robot: Robot,
}

impl Scenario {
    pub fn new(memory: Vec<i64>) -> Self {
        Scenario {
            computer: Computer::new(memory),
            robot: Robot::default(),
        }
    }
    pub fn execute_scenario(&mut self) -> usize {
        let mut run_scenario = true;
        while run_scenario {
            let terminate = self.computer.process(self.robot.get_current_color(), true);
            if terminate {
                break;
            }
            let color = self.computer.get_output();
            self.computer.process(0, true);
            let direction = self.computer.get_output();
            let _ = self.robot.paint_turn_and_advance(color, direction);
            run_scenario = !self.computer.is_finished();
        }
        self.robot.how_many_painted()
    }

    pub fn print_grid(&self) {
        self.robot.print_grid();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_integers() {
        let mut robot = Robot::default();
        let _ = robot.paint_turn_and_advance(1, 0);
        let _ = robot.paint_turn_and_advance(0, 0);
        let _ = robot.paint_turn_and_advance(1, 0);
        let _ = robot.paint_turn_and_advance(1, 0);
        let _ = robot.paint_turn_and_advance(0, 1);
        let _ = robot.paint_turn_and_advance(1, 0);
        let _ = robot.paint_turn_and_advance(1, 0);

        assert_eq!(robot.how_many_painted(), 6);
    }
}
