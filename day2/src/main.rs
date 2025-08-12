use computer::Computer;
use parser::Parser;
use std::path::Path;

static FILE_PATH: &str = "day2/data/input.txt";

fn main() {
    let reseted_memory: Vec<i32> = Parser::from_txt(Path::new(FILE_PATH));

    'outer: for i in 0..=99 {
        for j in 0..=99 {
            let mut memory_it = reseted_memory.clone();
            memory_it[1] = i;
            memory_it[2] = j;
            let mut code = Computer::new(memory_it);
            code.process(0);

            if code.get_first_position() == 19690720 {
                println!("i: {}", i);
                println!("j: {}", j);
                println!("value: {}", code.get_first_position());
                break 'outer;
            }
        }
    }
}
