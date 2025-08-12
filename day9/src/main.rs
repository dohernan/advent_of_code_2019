use computer::Computer;
use parser::Parser;
use std::path::Path; // 0.8.2

static FILE_PATH: &str = "day9/data/input.txt";

fn main() {
    let reseted_memory: Vec<i64> = Parser::from_txt_signed(Path::new(FILE_PATH));

    let mut code = Computer::new(reseted_memory);
    code.process(2, false);
    //println!("{}", code.get_output());
}
