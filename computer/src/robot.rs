use std::cell::RefCell;
use std::rc::Rc;
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

#[derive(Default, Clone, PartialEq)]
struct Panel {
    color: i64,
    painted: bool,
    reference_to_parent_panel: Option<Rc<RefCell<Panel>>>,
    is_finish: bool,
}

impl Panel {
    fn paint_color(&mut self, color: i64) {
        self.painted = true;
        self.color = color;
    }
    fn print(&self) {
        match self.color {
            0 => print!(" "),
            1 => print!("#"),
            -1 => print!("S"),
            3 => print!("."),
            _ => print!("{}",self.color),
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
        let grid = vec![vec![RefCell::new(Panel::default()); 45]; 6];
        grid[0][0].borrow_mut().paint_color(1);
        Robot {
            position_x: 0,
            position_y: 0,
            facing: Direction::North,
            grid,
        }
    }

    fn set_position(&mut self, x: usize, y: usize) {
        self.position_x = x;
        self.position_y = y;
    }

    fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![RefCell::new(Panel::default()); width]; height];
        Robot {
            position_x: width/2,
            position_y: height/2,
            facing: Direction::North,
            grid,
        }
    }
    pub fn get_facing(&self) -> i64 {
      self.get_direction(self.facing)
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

    fn set_direction(&mut self, direction: Direction) {
        self.facing = direction;
    }

    fn get_oposite_direction(&self) -> Direction {
        match self.facing {
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
        }
    }

    fn calculate_path_length(&self, panel: &RefCell<Panel>) -> i32 {
        let mut length = 0;
        let mut current_panel = panel;
        let mut reference_to_parent_panel = current_panel.borrow().reference_to_parent_panel.clone();
        while let Some(parent_panel) = reference_to_parent_panel {
            current_panel = &parent_panel;
            length += 1;
            current_panel.borrow_mut().paint_color(0);
            self.print_grid();
            reference_to_parent_panel = current_panel.borrow().reference_to_parent_panel.clone();
        }
        length
    }

    pub fn get_shortest_path_to_finish_from_start(&mut self) -> i32 {
        for panel in self.grid.iter().flat_map(|row| row.iter()) {
                                let mut panel_mut = panel.borrow_mut();
                                panel_mut.painted = false;

                            }
        let mut path: Vec<(RefCell<Panel>,usize, usize,usize)> = Vec::new();
        let  current = self.get_current_panel();
        let mut tiefe = 0;
        let mut max_tiefe = 0;
        let mut minutes = 0;
        current.borrow_mut().paint_color(0);
        current.borrow_mut().is_finish = false;
        path.push((RefCell::new((*current.borrow()).clone()),self.position_x, self.position_y, tiefe));
        while path.len() > 0 {
            let  (current_panel,x,y,tiefe) = path.pop().unwrap();
            if current_panel.borrow().is_finish {
                current_panel.borrow_mut().is_finish = false;
            }
            if tiefe > max_tiefe {
                max_tiefe = tiefe;
                println!("Max Tiefe: {}", max_tiefe);
            }
            if path.len() == 0{
            }
            for (child_panel,x,y) in self.get_adjacent_panels(&current_panel, x,y) {
                if !child_panel.borrow().painted {
                    child_panel.borrow_mut().paint_color(-1);
                    child_panel.borrow_mut().reference_to_parent_panel = Some(Rc::new(current_panel.clone()));
                    path.push((child_panel.clone(),x,y,tiefe+1));
                }
            } 
            self.print_grid();       
        }
                                println!("Max Tiefe: {}", max_tiefe);

        return -1; // No path found

    }

    fn get_direction(&self, direction:Direction) -> i64 {
        match direction {
            Direction::North => 1,
            Direction::West => 3,
            Direction::South => 2,
            Direction::East => 4,
        }
    }

    fn get_direction_from_int(&self, direction: i64) -> Direction {
        match direction {
            1 => Direction::North,
            3 => Direction::East,
            2 => Direction::South,
            4 => Direction::West,
            _ => panic!("Invalid direction"),
        }
    }

    fn get_next_position(&self) -> (usize, usize) {
        match self.facing {
            Direction::North => (self.position_x, self.position_y - 1),
            Direction::West => (self.position_x - 1, self.position_y),
            Direction::South => (self.position_x, self.position_y + 1),
            Direction::East => (self.position_x + 1, self.position_y),
        }
    }

    fn get_panel(&self,xy : (usize,  usize)) -> & RefCell<Panel> {
        &self.grid[xy.1][xy.0]
    }

    fn get_panel_in_direction(&self, direction: Direction) -> &RefCell<Panel> {
        let (x, y) = match direction {
            Direction::North => (self.position_x, self.position_y - 1),
            Direction::West => (self.position_x - 1, self.position_y),
            Direction::South => (self.position_x, self.position_y + 1),
            Direction::East => (self.position_x + 1, self.position_y),
        };
        &self.grid[y][x]
    }

    fn get_panel_in_direction_xy(&self, direction: Direction,x0:usize,y0:usize) -> (&RefCell<Panel>, usize, usize) {
        let (x, y) = match direction {
            Direction::North => (x0, y0 - 1),
            Direction::West => (x0 - 1, y0),
            Direction::South => (x0, y0 + 1),
            Direction::East => (x0 + 1, y0),
        };
        (&self.grid[y][x], x, y)
    }

    fn advance(&mut self) {
        //println!("{}-{}", self.position_x, self.position_y);
        (self.position_x, self.position_y) = self.get_next_position();
    }
    pub fn paint_next_position_color(&mut self, color: i64) {
        let (x,y) = self.get_next_position();
        (*self.grid[y][x].borrow_mut()).paint_color(color);
    }
    pub fn is_next_position_painted(&self) -> bool {
        let (x, y) = self.get_next_position();
        self.grid[y][x].borrow().painted
    }

    fn get_panel_at(&self, direction:Direction) -> &RefCell<Panel> {
        match direction {
            Direction::North => &self.grid[self.position_y - 1][self.position_x],
            Direction::West => &self.grid[self.position_y][self.position_x - 1],
            Direction::South => &self.grid[self.position_y + 1][self.position_x],
            Direction::East => &self.grid[self.position_y][self.position_x + 1],
        }
    }

    fn are_there_unpainted_neighbors(&self) -> bool {
        let neighbors = [
            (self.position_x, self.position_y - 1), // North
            (self.position_x - 1, self.position_y), // West
            (self.position_x, self.position_y + 1), // South
            (self.position_x + 1, self.position_y), // East
        ];
        for &(nx, ny) in &neighbors {
            if !self.grid[ny][nx].borrow().painted {
                return true;
            }
        }
        false
    }
    pub fn paint_color(&mut self, color: i64) {
        (*self.get_current_panel().borrow_mut()).paint_color(color);
    }

     fn is_current_position_painted(&self) -> bool {
        self.get_current_panel().borrow().painted
    }
    pub fn paint_turn_and_advance(&mut self, color: i64, direction: i64) -> Result<(), String> {
        self.paint_color(color);
        self.turn(direction);
        self.advance();
        Ok(())
    }

    fn get_adjacent_panels(&self, panel_ref: &RefCell<Panel>,x: usize,y: usize) -> Vec<(&RefCell<Panel>,usize,usize)> {
        let mut adjacent_panels = Vec::new();
        let directions = [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ];
        for direction in directions.iter() {
            let (next_panel,x,y) = self.get_panel_in_direction_xy(*direction,x,y);
            if (next_panel.borrow().color == 0) && next_panel.borrow().painted == false {
                next_panel.borrow_mut().reference_to_parent_panel = Some(Rc::new(panel_ref.clone()));
                adjacent_panels.push((next_panel,x,y));
            }
        }
        adjacent_panels
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
        self.get_current_panel().borrow().color
    }
    pub fn get_current_panel(&self) -> &RefCell<Panel> {
        &self.grid[self.position_y][self.position_x]
    }
}

#[derive(Default)]
pub struct Scenario {
    computer: Computer,
    robot: Robot,
}

impl Scenario {
    pub fn new(memory: Vec<i64>, width: usize, height: usize) -> Self {
        Scenario {
            computer: Computer::new(memory),
            robot: Robot::new(width, height),
        }
    }
    pub fn execute_paint_scenario(&mut self) -> usize {
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
    
    pub fn push_adjacent_directions(
        &self,
        directions_stack: &mut Vec<Direction>) {
        directions_stack.push(self.robot.get_oposite_direction());
        for i in 1..=4 {
            let direction = self.robot.get_direction_from_int(i as i64);
            if direction != self.robot.get_oposite_direction()
             && !self.robot.get_panel_in_direction(direction).borrow().painted {
                let  mut panel_in_direction = self.robot.get_panel_in_direction(direction).borrow_mut();
               panel_in_direction.reference_to_parent_panel = Some(Rc::new(self.robot.get_current_panel().clone()));
                directions_stack.push(direction);
             }
        }      
    }

    pub fn execute_search_oxigen(&mut self)  {
        let mut color = 3;
        self.robot.set_position(self.robot.grid[0].len()/2, self.robot.grid.len()/2);
        self.robot.paint_color(-1); // Start with the initial color painted
        let mut directions_stack = Vec::<Direction>::new();
        let mut rounds = 1;
        self.push_adjacent_directions(&mut directions_stack );
        while rounds >= 0
        {
            while directions_stack.len() > 0 {
                let try_this_direction =directions_stack.pop().unwrap();
                self.robot.set_direction(try_this_direction);
                let terminate = self.computer.process(self.robot.get_facing(), true);
                if terminate {
                    break;
                }
                match self.computer.get_output()
                {
                    0 => {
                        self.robot.paint_next_position_color(1);
                    },                   
                    1 => {
                        if !self.robot.are_there_unpainted_neighbors() {
                            if directions_stack.len() != 1 {
                                self.robot.paint_color(0);}
                        }
                        self.robot.advance();
                        if !self.robot.is_current_position_painted(){
                            self.push_adjacent_directions(&mut directions_stack);
                            self.robot.paint_color(3);
                        }
                        else{
                            
                            color-=1;
                        }
                        
                        color+=1;
                        
                    },
                    2 => {self.robot.paint_color(0);self.robot.advance();self.robot.paint_color(0);
                        if rounds > 0{
                            rounds -= 1;
                            for panel in self.robot.grid.iter().flat_map(|row| row.iter()) {
                                let mut panel_mut = panel.borrow_mut();
                                panel_mut.painted = false;

                            }
                                            self.robot.print_grid();


                        }
                        else{
                                        self.robot.print_grid();

                            break;
                        }
                                                         self.robot.get_current_panel().borrow_mut().paint_color(-1);
           self.robot.print_grid();

                        //break;
                    },
                    _ => panic!("Unexpected output"),
                }
            }
            rounds -= 1;
                    //self.robot.set_position(self.robot.grid[0].len()/2, self.robot.grid.len()/2);

               }
               
        println!("Result: {}", self.robot.get_shortest_path_to_finish_from_start());
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
