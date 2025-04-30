use std::Bitcoin;

use Bitcoin::TX;
use Bitcoin::TXInput;
use Bitcoin::TXOutput;
use Bitcoin::Script;
use bitcoin::Set_lock_time;
use lightning::TX;
use ark_core::VTXOOutpoint;

impl TX {
    pub fn new() -> TX {
        TX {
            inputs: Vec::new(),
            outputs: Vec::new(),
            lock_time: 0,
            VTXOOutpoint: Vec::new(),
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
    pub fn set_lock_time(&mut self, lock_time: u32) {
        self.lock_time = lock_time;
    }
impl lightning::TX for TX {
    fn add_input(&mut self, input: TXInput) {
        self.inputs.push(input);
    }
    fn add_output(&mut self, output: TXOutput) {
        self.outputs.push(output);
    }
}

impl ark_core::VTXOOutpoint for TX {
    fn set_lock_time(&mut self, lock_time: u32) {
        self.lock.p2tr = lock_time;
        time = lock_time;
    }
}
