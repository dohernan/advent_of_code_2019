#[derive(Default)]
pub struct Computer {
    pub memory: Vec<i64>,
    aditional_memory: Vec<i64>,
    pointer: usize,
    input_instruction: i64,
    output: i64,
    phase_setting: i64,
    phase_set: bool,
    status: Status,
    relative_base: i64,
}

#[derive(Default, Clone, Copy, PartialEq)]
enum Status {
    #[default]
    Halt,
    Paused,
    Running,
}

#[derive(Debug)]
enum Instruction {
    Add(ModedValue, ModedValue, ModedValue),
    Multiply(ModedValue, ModedValue, ModedValue),
    Input(ModedValue),
    Output(ModedValue),
    JumpIfTrue(ModedValue, ModedValue),
    JumpIfFalse(ModedValue, ModedValue),
    LessThan(ModedValue, ModedValue, ModedValue),
    Equals(ModedValue, ModedValue, ModedValue),
    RelativeBaseOffset(ModedValue),
    Terminate,
}

#[derive(Debug, Clone)]
enum ModedValue {
    Position(bool, usize),
    Immediate(i64),
    Relative(bool, i64),
}

impl ModedValue {
    fn new(mode: i64, value: i64, is_write: bool) -> Result<Self, String> {
        match mode {
            0 => Ok(Self::Position(is_write, value as usize)),
            1 => Ok(Self::Immediate(value)),
            2 => Ok(Self::Relative(is_write, value)),
            _ => Err(String::from("Mode not valid")),
        }
    }
}

impl TryFrom<&[i64]> for Instruction {
    type Error = String;
    fn try_from(value: &[i64]) -> Result<Self, Self::Error> {
        type Error = String;
        let opcode = value[0];
        let instruction_code = opcode % 100;
        match instruction_code {
            1..=2 | 7..=8 => {
                let mut mode3 = (opcode / 10000) % 10;
                if mode3 == 0 {
                    mode3 = 1;
                }
                let mode1 = (opcode / 100) % 10;
                let mode2 = (opcode / 1000) % 10;
                if instruction_code == 1 {
                    Ok(Instruction::Add(
                        ModedValue::new(mode1, value[1], false).unwrap(),
                        ModedValue::new(mode2, value[2], false).unwrap(),
                        ModedValue::new(mode3, value[3], true).unwrap(),
                    ))
                } else if instruction_code == 2 {
                    Ok(Instruction::Multiply(
                        ModedValue::new(mode1, value[1], false).unwrap(),
                        ModedValue::new(mode2, value[2], false).unwrap(),
                        ModedValue::new(mode3, value[3], true).unwrap(),
                    ))
                } else if instruction_code == 7 {
                    Ok(Instruction::LessThan(
                        ModedValue::new(mode1, value[1], false).unwrap(),
                        ModedValue::new(mode2, value[2], false).unwrap(),
                        ModedValue::new(mode3, value[3], true).unwrap(),
                    ))
                } else {
                    Ok(Instruction::Equals(
                        ModedValue::new(mode1, value[1], false).unwrap(),
                        ModedValue::new(mode2, value[2], false).unwrap(),
                        ModedValue::new(mode3, value[3], true).unwrap(),
                    ))
                }
            }
            3..=4 => {
                if instruction_code == 3 {
                    let mode1: i64 = (opcode / 100) % 10;
                    Ok(Instruction::Input(
                        ModedValue::new(mode1, value[1], true).unwrap(),
                    ))
                } else {
                    let mode1: i64 = (opcode / 100) % 10;
                    Ok(Instruction::Output(
                        ModedValue::new(mode1, value[1], false).unwrap(),
                    ))
                }
            }
            5..=6 => {
                let mode1: i64 = (opcode / 100) % 10;
                let mode2: i64 = (opcode / 1000) % 10;
                if instruction_code == 5 {
                    Ok(Instruction::JumpIfTrue(
                        ModedValue::new(mode1, value[1], false).unwrap(),
                        ModedValue::new(mode2, value[2], false).unwrap(),
                    ))
                } else {
                    Ok(Instruction::JumpIfFalse(
                        ModedValue::new(mode1, value[1], false).unwrap(),
                        ModedValue::new(mode2, value[2], false).unwrap(),
                    )) // can be substituted with jump if true, try
                }
            }
            9 => {
                let mode1: i64 = (opcode / 100) % 10;
                Ok(Instruction::RelativeBaseOffset(
                    ModedValue::new(mode1, value[1], false).unwrap(),
                ))
            }
            99 => Ok(Instruction::Terminate),
            _ => Err(Error::from("ERROR IN Instruction")),
        }
    }
}

impl Computer {
    pub fn new(memory: Vec<i64>) -> Self {
        Self {
            memory,
            aditional_memory: vec![0; 1000000],
            pointer: 0,
            input_instruction: 0,
            output: 0,
            phase_setting: 0,
            phase_set: false,
            status: Status::Halt,
            relative_base: 0,
        }
    }

    fn set_memory_at(&mut self, position: usize, value: i64) {
        if position < self.memory.len() {
            self.memory[position] = value;
        } else {
            let position: usize = position - self.memory.len();
            self.aditional_memory[position] = value;
        }
    }

    fn get_memory_at(&self, position: usize) -> i64 {
        if position < self.memory.len() {
            self.memory[position]
        } else {
            let position = position - self.memory.len();
            self.aditional_memory[position]
        }
    }

    fn get_memory_slice(&self, start: usize, end: usize) -> &[i64] {
        &self.memory[start..end]
    }

    pub fn set_phase_setting(&mut self, phase_setting: i64) {
        self.phase_setting = phase_setting;
        self.phase_set = true;
    }
    fn get_value(&self, modedvalue: ModedValue) -> i64 {
        match modedvalue {
            ModedValue::Position(is_write, pos) => {
                if is_write {
                    pos as i64
                } else {
                    self.get_memory_at(pos)
                }
            }
            ModedValue::Immediate(value) => value,
            ModedValue::Relative(is_write, value) => {
                if is_write {
                    return self.relative_base + value;
                }
                self.get_memory_at((self.relative_base + value) as usize)
            }
        }
    }
    fn execute_next_instruction(&mut self) -> Result<Option<i64>, String> {
        let instruction_code = self.get_memory_at(self.pointer) % 100;
        let mut output = None;
        let mut next_pointer = match instruction_code {
            1..=2 | 7..=8 => self.pointer + 4,
            3..=4 | 9 => self.pointer + 2,
            5..=6 => self.pointer + 3,
            _ => self.pointer + 1,
        };
        let instruction = Instruction::try_from(self.get_memory_slice(self.pointer, next_pointer))?;
        match instruction {
            Instruction::Add(a, b, c) => self.set_memory_at(
                self.get_value(c) as usize,
                self.get_value(a) + self.get_value(b),
            ),
            Instruction::Multiply(a, b, c) => self.set_memory_at(
                self.get_value(c) as usize,
                self.get_value(a) * self.get_value(b),
            ),
            Instruction::Input(a) => {
                let input_value = if self.pointer == 0 && self.phase_set {
                    self.phase_setting
                } else {
                    self.input_instruction
                };
                //println!("Input value: {}", input_value);
                let b = self.get_value(a);
                self.set_memory_at(b as usize, input_value)
            }
            Instruction::Output(a) => {
                output = Some(self.get_value(a));
            }
            Instruction::JumpIfTrue(a, position) => {
                if self.get_value(a) != 0 {
                    next_pointer = self.get_value(position) as usize;
                }
            }
            Instruction::JumpIfFalse(a, position) => {
                if self.get_value(a) == 0 {
                    next_pointer = self.get_value(position) as usize;
                }
            }
            Instruction::LessThan(a, b, c) => {
                if self.get_value(a) < self.get_value(b) {
                    self.set_memory_at(self.get_value(c) as usize, 1)
                } else {
                    self.set_memory_at(self.get_value(c) as usize, 0)
                }
            }
            Instruction::Equals(a, b, c) => {
                if self.get_value(a) == self.get_value(b) {
                    self.set_memory_at(self.get_value(c) as usize, 1)
                } else {
                    self.set_memory_at(self.get_value(c) as usize, 0)
                }
            }
            Instruction::Terminate => {
                self.status = Status::Halt;
            }
            Instruction::RelativeBaseOffset(value) => {
                self.relative_base += self.get_value(value);
            }
        }
        self.pointer = next_pointer;
        Ok(output)
    }

    pub fn process(&mut self, input_instruction: i64, pause_if_output: bool) -> bool {
        self.input_instruction = input_instruction;
        self.status = Status::Running;
        while self.status == Status::Running {
            let result = self.execute_next_instruction();
            if let Err(a) = &result {
                println!("{} {}", a, self.pointer);
            }
            if let Some(result_value) = &result.unwrap().clone() {
                self.output = *result_value;
                //println!("{}", self.output);
                if pause_if_output {
                    self.status = Status::Paused;
                }
            }
        }
        self.status == Status::Halt
        //println!("Processed");
    }

    pub fn get_first_position(&self) -> i64 {
        self.get_memory_at(0)
    }

    pub fn is_finished(&self) -> bool {
        self.status == Status::Halt
    }

    pub fn get_output(&self) -> i64 {
        self.output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_integers() {
        let memory = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut code = Computer::new(memory);
        code.process(0, false);
        assert_eq!(code.get_first_position(), 3500);
    }

    #[test]
    fn test_from_integers2() {
        let memory = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut code = Computer::new(memory);
        code.process(1, false);
    }

    #[test]
    fn test_from_integers3() {
        let memory = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut code = Computer::new(memory);
        code.process(1, false);
    }

    #[test]
    fn test_from_integers4() {
        let memory = vec![
            3,
            8,
            1005,
            8,
            326,
            1106,
            0,
            11,
            0,
            0,
            0,
            104,
            1,
            104,
            0,
            3,
            8,
            102,
            -1,
            8,
            10,
            101,
            1,
            10,
            10,
            4,
            10,
            1008,
            8,
            1,
            10,
            4,
            10,
            1001,
            8,
            0,
            29,
            2,
            1003,
            17,
            10,
            1006,
            0,
            22,
            2,
            106,
            5,
            10,
            1006,
            0,
            87,
            3,
            8,
            102,
            -1,
            8,
            10,
            101,
            1,
            10,
            10,
            4,
            10,
            1008,
            8,
            1,
            10,
            4,
            10,
            1001,
            8,
            0,
            65,
            2,
            7,
            20,
            10,
            2,
            9,
            17,
            10,
            2,
            6,
            16,
            10,
            3,
            8,
            102,
            -1,
            8,
            10,
            1001,
            10,
            1,
            10,
            4,
            10,
            1008,
            8,
            0,
            10,
            4,
            10,
            101,
            0,
            8,
            99,
            1006,
            0,
            69,
            1006,
            0,
            40,
            3,
            8,
            102,
            -1,
            8,
            10,
            1001,
            10,
            1,
            10,
            4,
            10,
            1008,
            8,
            1,
            10,
            4,
            10,
            101,
            0,
            8,
            127,
            1006,
            0,
            51,
            2,
            102,
            17,
            10,
            3,
            8,
            1002,
            8,
            -1,
            10,
            1001,
            10,
            1,
            10,
            4,
            10,
            108,
            1,
            8,
            10,
            4,
            10,
            1002,
            8,
            1,
            155,
            1006,
            0,
            42,
            3,
            8,
            1002,
            8,
            -1,
            10,
            101,
            1,
            10,
            10,
            4,
            10,
            108,
            0,
            8,
            10,
            4,
            10,
            101,
            0,
            8,
            180,
            1,
            106,
            4,
            10,
            2,
            1103,
            0,
            10,
            1006,
            0,
            14,
            3,
            8,
            102,
            -1,
            8,
            10,
            1001,
            10,
            1,
            10,
            4,
            10,
            108,
            0,
            8,
            10,
            4,
            10,
            1001,
            8,
            0,
            213,
            1,
            1009,
            0,
            10,
            3,
            8,
            1002,
            8,
            -1,
            10,
            1001,
            10,
            1,
            10,
            4,
            10,
            108,
            0,
            8,
            10,
            4,
            10,
            1002,
            8,
            1,
            239,
            1006,
            0,
            5,
            2,
            108,
            5,
            10,
            2,
            1104,
            7,
            10,
            3,
            8,
            102,
            -1,
            8,
            10,
            101,
            1,
            10,
            10,
            4,
            10,
            108,
            0,
            8,
            10,
            4,
            10,
            102,
            1,
            8,
            272,
            2,
            1104,
            12,
            10,
            1,
            1109,
            10,
            10,
            3,
            8,
            102,
            -1,
            8,
            10,
            1001,
            10,
            1,
            10,
            4,
            10,
            108,
            1,
            8,
            10,
            4,
            10,
            102,
            1,
            8,
            302,
            1006,
            0,
            35,
            101,
            1,
            9,
            9,
            1007,
            9,
            1095,
            10,
            1005,
            10,
            15,
            99,
            109,
            648,
            104,
            0,
            104,
            1,
            21102,
            937268449940,
            1,
            1,
            21102,
            1,
            343,
            0,
            1105,
            1,
            447,
            21101,
            387365315480,
            0,
            1,
            21102,
            1,
            354,
            0,
            1105,
            1,
            447,
            3,
            10,
            104,
            0,
            104,
            1,
            3,
            10,
            104,
            0,
            104,
            0,
            3,
            10,
            104,
            0,
            104,
            1,
            3,
            10,
            104,
            0,
            104,
            1,
            3,
            10,
            104,
            0,
            104,
            0,
            3,
            10,
            104,
            0,
            104,
            1,
            21101,
            0,
            29220891795,
            1,
            21102,
            1,
            401,
            0,
            1106,
            0,
            447,
            21101,
            0,
            248075283623,
            1,
            21102,
            412,
            1,
            0,
            1105,
            1,
            447,
            3,
            10,
            104,
            0,
            104,
            0,
            3,
            10,
            104,
            0,
            104,
            0,
            21101,
            0,
            984353760012,
            1,
            21102,
            1,
            435,
            0,
            1105,
            1,
            447,
            21102,
            1,
            718078227200,
            1,
            21102,
            1,
            446,
            0,
            1105,
            1,
            447,
            99,
            109,
            2,
            21202,
            -1,
            1,
            1,
            21102,
            40,
            1,
            2,
            21101,
            0,
            478,
            3,
            21101,
            468,
            0,
            0,
            1106,
            0,
            511,
            109,
            -2,
            2106,
            0,
            0,
            0,
            1,
            0,
            0,
            1,
            109,
            2,
            3,
            10,
            204,
            -1,
            1001,
            473,
            474,
            489,
            4,
            0,
            1001,
            473,
            1,
            473,
            108,
            4,
            473,
            10,
            1006,
            10,
            505,
            1102,
            1,
            0,
            473,
            109,
            -2,
            2105,
            1,
            0,
            0,
            109,
            4,
            1202,
            -1,
            1,
            510,
            1207,
            -3,
            0,
            10,
            1006,
            10,
            528,
            21102,
            1,
            0,
            -3,
            22102,
            1,
            -3,
            1,
            22101,
            0,
            -2,
            2,
            21101,
            0,
            1,
            3,
            21102,
            1,
            547,
            0,
            1105,
            1,
            552,
            109,
            -4,
            2105,
            1,
            0,
            109,
            5,
            1207,
            -3,
            1,
            10,
            1006,
            10,
            575,
            2207,
            -4,
            -2,
            10,
            1006,
            10,
            575,
            21202,
            -4,
            1,
            -4,
            1105,
            1,
            643,
            21202,
            -4,
            1,
            1,
            21201,
            -3,
            -1,
            2,
            21202,
            -2,
            2,
            3,
            21102,
            1,
            594,
            0,
            1106,
            0,
            552,
            22102,
            1,
            1,
            -4,
            21101,
            1,
            0,
            -1,
            2207,
            -4,
            -2,
            10,
            1006,
            10,
            613,
            21101,
            0,
            0,
            -1,
            22202,
            -2,
            -1,
            -2,
            2107,
            0,
            -3,
            10,
            1006,
            10,
            635,
            22101,
            0,
            -1,
            1,
            21101,
            0,
            635,
            0,
            106,
            0,
            510,
            21202,
            -2,
            -1,
            -2,
            22201,
            -4,
            -2,
            -4,
            109,
            -5,
            2105,
            1,
            0,
        ];
        let mut code = Computer::new(memory);
        code.process(0, true);
        code.process(0, true);
        code.process(0, true);
        code.process(0, true);
        code.process(0, true);
        code.process(0, true);
        code.process(0, true);
        code.process(0, true);
        code.process(0, true);
        code.process(0, true);
        code.process(0, true);
        code.process(0, true);
    }
}
