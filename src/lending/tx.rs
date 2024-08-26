use std::Bitcoin;

use Bitcoin::TX;
use Bitcoin::TXInput;
use Bitcoin::TXOutput;
use Bitcoin::Script;
use lightning::TX;

impl TX {
    pub fn new() -> TX {
        TX {
            inputs: Vec::new(),
            outputs: Vec::new(),
            lock_time: 0,
        }
    }
}
pub fn add_input(&mut self, input: TXInput) {
        self.inputs.push(input);
    }

    pub fn add_output(&mut self, output: TXOutput) {
        self.outputs.push(output);
    }

    pub fn set_lock_time(&mut self, lock_time: u32) {
        self.lock_time = lock_time;
    }

    pub fn get_lock_time(&self) -> u32 {
        self.lock_time
    }

    pub fn get_input(&self, index: usize) -> &TXInput {
        &self.inputs[index]
    }

    pub fn get_output(&self, index: usize) -> &TXOutput {
        &self.outputs[index]
    }
impl lightning::TX for TX {
    fn add_input(&mut self, input: TXInput) {
        self.inputs.push(input);
    }
    fn add_output(&mut self, output: TXOutput) {
        self.outputs.push(output);
    }
}