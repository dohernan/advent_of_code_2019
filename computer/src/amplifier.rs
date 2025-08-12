use crate::computer::Computer;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct Amplifier {
    computer: Computer,
    phase_setting: Rc<RefCell<i64>>,
}

#[derive(Default)]
pub struct AmplificationCircuit {
    amplifiers: [Amplifier; 5],
    phase_settings: PhaseSettings,
}

pub type PhaseSettings = Vec<Rc<RefCell<i64>>>;

impl From<Vec<i64>> for AmplificationCircuit {
    fn from(program: Vec<i64>) -> Self {
        let mut result: AmplificationCircuit = AmplificationCircuit::default();
        let phase_settings: PhaseSettings = vec![
            Rc::new(RefCell::new(0)),
            Rc::new(RefCell::new(1)),
            Rc::new(RefCell::new(2)),
            Rc::new(RefCell::new(3)),
            Rc::new(RefCell::new(4)),
        ];
        for i in 0..phase_settings.len() {
            result.amplifiers[i] = Amplifier {
                computer: Computer::new(program.clone()),
                phase_setting: Rc::clone(&phase_settings[i]),
            };
        }
        result.phase_settings = phase_settings;
        result
    }
}

impl AmplificationCircuit {
    pub fn set_phase_setting(&mut self, phase_settings: Vec<i64>) {
        for (i, setting) in phase_settings.iter().enumerate() {
            *self.phase_settings[i].borrow_mut() = *setting;
        }
        for amplifier in self.amplifiers.iter_mut() {
            amplifier
                .computer
                .set_phase_setting(*amplifier.phase_setting.borrow());
        }
    }
    pub fn process(&mut self) -> i64 {
        let mut input_signal = 0;
        let mut last_running = true;
        while last_running {
            for amplifier in self.amplifiers.iter_mut() {
                amplifier.computer.process(input_signal, true);
                input_signal = amplifier.computer.get_output();
            }
            last_running = !self.amplifiers[4].computer.is_finished();
        }
        input_signal
    }
}
