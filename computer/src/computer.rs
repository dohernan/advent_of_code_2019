use std::ops::{Deref, DerefMut};

#[derive(Default)]
pub struct Computer {
    memory: Vec<i32>,
    pointer: usize,
    input_instruction: i32,
    output: i32,
    phase_setting: i32,
    phase_set: bool,
    status: Status,
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
    Add(ModedValue, ModedValue, usize),
    Multiply(ModedValue, ModedValue, usize),
    Input(usize),
    Output(usize),
    JumpIfTrue(ModedValue, ModedValue),
    JumpIfFalse(ModedValue, ModedValue),
    LessThan(ModedValue, ModedValue, usize),
    Equals(ModedValue, ModedValue, usize),
    Terminate,
}

#[derive(Debug)]
enum ModedValue {
    Position(usize),
    Immediate(i32),
}

impl ModedValue {
    fn new(is_immediate: i32, value: i32) -> Result<Self, String> {
        match is_immediate {
            1 => Ok(Self::Immediate(value)),
            0 => Ok(Self::Position(value as usize)),
            _ => Err(String::from("Mode not valid")),
        }
    }
}

impl TryFrom<&[i32]> for Instruction {
    type Error = String;
    fn try_from(value: &[i32]) -> Result<Self, Self::Error> {
        type Error = String;
        let opcode = value[0];
        let instruction_code = opcode % 100;
        match instruction_code {
            1..=2 | 7..=8 => {
                let destiny = value[3];
                let mode3 = (opcode / 10000) % 10;
                if mode3 != 0 {
                    return Err(Error::from("Mode of destiny must be 0"));
                }
                let mode1 = (opcode / 100) % 10;
                let mode2 = (opcode / 1000) % 10;
                if instruction_code == 1 {
                    Ok(Instruction::Add(
                        ModedValue::new(mode1, value[1]).unwrap(),
                        ModedValue::new(mode2, value[2]).unwrap(),
                        destiny as usize,
                    ))
                } else if instruction_code == 2 {
                    Ok(Instruction::Multiply(
                        ModedValue::new(mode1, value[1]).unwrap(),
                        ModedValue::new(mode2, value[2]).unwrap(),
                        destiny as usize,
                    ))
                } else if instruction_code == 7 {
                    Ok(Instruction::LessThan(
                        ModedValue::new(mode1, value[1]).unwrap(),
                        ModedValue::new(mode2, value[2]).unwrap(),
                        destiny as usize,
                    ))
                } else {
                    Ok(Instruction::Equals(
                        ModedValue::new(mode1, value[1]).unwrap(),
                        ModedValue::new(mode2, value[2]).unwrap(),
                        destiny as usize,
                    ))
                }
            }
            3..=4 => {
                if instruction_code == 3 {
                    Ok(Instruction::Input(value[1] as usize))
                } else {
                    Ok(Instruction::Output(value[1] as usize))
                }
            }
            5..=6 => {
                let mode1: i32 = (opcode / 100) % 10;
                let mode2: i32 = (opcode / 1000) % 10;
                if instruction_code == 5 {
                    Ok(Instruction::JumpIfTrue(
                        ModedValue::new(mode1, value[1]).unwrap(),
                        ModedValue::new(mode2, value[2]).unwrap(),
                    ))
                } else {
                    Ok(Instruction::JumpIfFalse(
                        ModedValue::new(mode1, value[1]).unwrap(),
                        ModedValue::new(mode2, value[2]).unwrap(),
                    )) // can be substituted with jump if true, try
                }
            }
            99 => Ok(Instruction::Terminate),
            _ => Err(String::from("ERROR IN Instruction")),
        }
    }
}

impl Deref for Computer {
    type Target = Vec<i32>;
    fn deref(&self) -> &Self::Target {
        &self.memory
    }
}

impl DerefMut for Computer {
    fn deref_mut(&mut self) -> &mut Vec<i32> {
        &mut self.memory
    }
}

impl Computer {
    pub fn new(memory: Vec<i32>) -> Self {
        Self {
            memory,
            pointer: 0,
            input_instruction: 0,
            output: 0,
            phase_setting: 0,
            phase_set: false,
            status: Status::Halt,
        }
    }
    pub fn set_phase_setting(&mut self, phase_setting: i32) {
        self.phase_setting = phase_setting;
        self.phase_set = true;
    }
    fn get_value(&self, modedvalue: ModedValue) -> i32 {
        match modedvalue {
            ModedValue::Position(pos) => self[pos],
            ModedValue::Immediate(value) => value,
        }
    }
    fn execute_next_instruction(&mut self) -> Result<Option<i32>, String> {
        let instruction_code = self[self.pointer] % 100;
        let mut output = None;
        let mut next_pointer = match instruction_code {
            1..=2 | 7..=8 => self.pointer + 4,
            3..=4 => self.pointer + 2,
            5..=6 => self.pointer + 3,
            _ => self.pointer + 1,
        };
        let instruction = self[self.pointer..next_pointer].try_into()?;
        match instruction {
            Instruction::Add(a, b, c) => self[c] = self.get_value(a) + self.get_value(b),
            Instruction::Multiply(a, b, c) => self[c] = self.get_value(a) * self.get_value(b),
            Instruction::Input(a) => {
                let input_value = if self.pointer == 0 && self.phase_set {
                    self.phase_setting
                } else {
                    self.input_instruction
                };
                //println!("Input value: {}", input_value);
                self[a] = input_value;
            }
            Instruction::Output(a) => {
                output = Some(self[a]);
                //println!("Output {}", self[a])
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
                    self[c] = 1;
                } else {
                    self[c] = 0;
                }
            }
            Instruction::Equals(a, b, c) => {
                if self.get_value(a) == self.get_value(b) {
                    self[c] = 1;
                } else {
                    self[c] = 0;
                }
            }
            Instruction::Terminate => {
                self.status = Status::Halt;
            }
        }
        self.pointer = next_pointer;
        Ok(output)
    }

    pub fn process(&mut self, input_instruction: i32) {
        self.input_instruction = input_instruction;
        self.status = Status::Running;
        while self.status == Status::Running {
            let result = self.execute_next_instruction();
            if let Err(a) = &result {
                println!("{} {}", a, self.pointer);
            }
            if let Some(result_value) = &result.unwrap().clone() {
                self.output = *result_value;
                self.status = Status::Paused;
            }
        }
        //println!("Processed");
    }

    pub fn get_first_position(&self) -> i32 {
        self[0]
    }

    pub fn is_finished(&self) -> bool {
        self.status == Status::Halt
    }

    pub fn get_output(&self) -> i32 {
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
        code.process(0);
        assert_eq!(code.get_first_position(), 3500);
    }

    #[test]
    fn test_from_integers2() {
        let memory = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut code = Computer::new(memory);
        code.process(1);
    }
}
