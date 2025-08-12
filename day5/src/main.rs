use computer::Computer;
use parser::Parser;
use std::path::Path;

static FILE_PATH: &str = "day5/data/input.txt";

fn main() {
    let reseted_memory: Vec<i64> = Parser::from_txt_signed(Path::new(FILE_PATH));

    let mut code = Computer::new(reseted_memory);
    code.process(5, false);
    println!("{}", code.get_output());
}
