use computer::robot::Scenario;
use parser::Parser;
use std::path::Path; // 0.8.2

static FILE_PATH: &str = "day11/data/input.txt";

fn main() {
    let reseted_memory: Vec<i64> = Parser::from_txt_signed(Path::new(FILE_PATH));

    let mut code = Scenario::new(reseted_memory);
    println!("RESULT {}", code.execute_scenario());
    code.print_grid();
}
