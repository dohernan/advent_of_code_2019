use computer::arcade::Scenario;
use parser::Parser;
use std::path::Path; // 0.8.2

static FILE_PATH: &str = "day13/data/input.txt";

fn main() {
    let reseted_memory: Vec<i64> = Parser::from_txt_signed(Path::new(FILE_PATH));

    let mut code = Scenario::new(reseted_memory);
    println!("RESULT {}", code.execute_scenario());
}
