use computer::amplifier::AmplificationCircuit;
use itertools::Itertools;
use parser::Parser;
use std::path::Path; // 0.8.2

static FILE_PATH: &str = "day7/data/input.txt";

fn main() {
    let amplification_program: Vec<i32> = Parser::from_txt_signed(Path::new(FILE_PATH));

    let phase_settings_vector = vec![5, 6, 7, 8, 9];
    let permutations = phase_settings_vector.into_iter().permutations(5).unique();

    let mut outputs: Vec<i32> = vec![];
    for permutation in permutations {
        //dbg!(&permutation);
        let mut amplification_circuit = AmplificationCircuit::from(amplification_program.clone());
        amplification_circuit.set_phase_setting(permutation);
        outputs.push(amplification_circuit.process());
    }
    outputs.sort();
    println!("{}", outputs[outputs.len() - 1]);
}
