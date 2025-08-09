use std::ops::{Deref, DerefMut};

pub struct Computer(Vec<u32>);

impl Computer {
    pub fn new(memory: Vec<u32>) -> Self {
        Self(memory)
    }
}
impl Deref for Computer {
    type Target = Vec<u32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Computer {
    fn deref_mut(&mut self) -> &mut Vec<u32> {
        &mut self.0
    }
}

impl Computer {
    pub fn process(&mut self) {
        let mut i = 0;
        while i < self.len() {
            match self[i] {
                1 => {
                    let pos1 = self[i + 1] as usize;
                    let pos2 = self[i + 2] as usize;
                    let pos3 = self[i + 3] as usize;
                    self[pos3] = self[pos1] + self[pos2];
                    i += 4;
                }
                2 => {
                    let pos1 = self[i + 1] as usize;
                    let pos2 = self[i + 2] as usize;
                    let pos3 = self[i + 3] as usize;
                    self[pos3] = self[pos1] * self[pos2];
                    i += 4;
                }
                99 => {
                    break;
                }
                _ => {
                    panic!("ERROR IN CODE");
                }
            }
        }
    }

    pub fn get_first_position(&self) -> u32 {
        self[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_integers() {
        let memory = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut code = Computer::new(memory);
        code.process();
        assert_eq!(code.get_first_position(), 3500);
    }
}
