#[derive(Default)]
pub struct Computer {
    memory: Vec<i64>,
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

#[derive(Debug)]
enum ModedValue {
    Position(usize),
    Immediate(i64),
    Relative(bool, i64),
}

impl ModedValue {
    fn new(mode: i64, value: i64, is_immediate: bool) -> Result<Self, String> {
        match mode {
            0 => Ok(Self::Position(value as usize)),
            1 => Ok(Self::Immediate(value)),
            2 => Ok(Self::Relative(is_immediate, value)),
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
                let mut is_immediate = false;
                let mut mode3 = (opcode / 10000) % 10;
                if mode3 == 1 {
                    return Err(Error::from("Mode of destiny cannot be immediate"));
                }
                if mode3 == 2 {
                    is_immediate = true;
                }
                if mode3 == 0 {
                    mode3 = 1;
                }
                let mode1 = (opcode / 100) % 10;
                let mode2 = (opcode / 1000) % 10;
                if instruction_code == 1 {
                    Ok(Instruction::Add(
                        ModedValue::new(mode1, value[1], is_immediate).unwrap(),
                        ModedValue::new(mode2, value[2], is_immediate).unwrap(),
                        ModedValue::new(mode3, value[3], is_immediate).unwrap(),
                    ))
                } else if instruction_code == 2 {
                    Ok(Instruction::Multiply(
                        ModedValue::new(mode1, value[1], is_immediate).unwrap(),
                        ModedValue::new(mode2, value[2], is_immediate).unwrap(),
                        ModedValue::new(mode3, value[3], is_immediate).unwrap(),
                    ))
                } else if instruction_code == 7 {
                    Ok(Instruction::LessThan(
                        ModedValue::new(mode1, value[1], is_immediate).unwrap(),
                        ModedValue::new(mode2, value[2], is_immediate).unwrap(),
                        ModedValue::new(mode3, value[3], is_immediate).unwrap(),
                    ))
                } else {
                    Ok(Instruction::Equals(
                        ModedValue::new(mode1, value[1], is_immediate).unwrap(),
                        ModedValue::new(mode2, value[2], is_immediate).unwrap(),
                        ModedValue::new(mode3, value[3], is_immediate).unwrap(),
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
            _ => Err(String::from("ERROR IN Instruction")),
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
            let position = position - self.memory.len();
            if position >= self.aditional_memory.len() {
                self.aditional_memory.resize(dbg!(position + 10000000), 0);
            }
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
            ModedValue::Position(pos) => self.get_memory_at(pos),
            ModedValue::Immediate(value) => value,
            ModedValue::Relative(is_immediate, value) => {
                if is_immediate {
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
                println!("Input value: {}", input_value);
                let b = self.get_value(a);
                println!("at position: {}", b);
                self.set_memory_at(b as usize, input_value)
            }
            Instruction::Output(a) => {
                output = Some(self.get_value(a));
                println!("{}", output.unwrap())
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

    pub fn process(&mut self, input_instruction: i64, pause_if_output: bool) {
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
        let memory = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut code = Computer::new(memory);
        code.process(1, false);
    }
}
