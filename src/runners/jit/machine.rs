use super::value::Value;
use crate::{CoreMachine, Error, Machine, Register, SparseMemory, RISCV_GENERAL_REGISTER_NUMBER};

pub struct BaselineJitCompilingMachine {
    registers: [Value; RISCV_GENERAL_REGISTER_NUMBER],
    pc: Value,
}

impl BaselineJitCompilingMachine {
    pub fn new(pc: usize) -> Self {
        let mut registers: [Value; RISCV_GENERAL_REGISTER_NUMBER] = Default::default();
        for i in 0..registers.len() {
            registers[i] = Value::Register(i);
        }
        Self {
            registers,
            pc: Value::from_usize(pc),
        }
    }
}

impl CoreMachine for BaselineJitCompilingMachine {
    type REG = Value;
    type MEM = SparseMemory;

    fn pc(&self) -> &Value {
        &self.pc
    }

    fn set_pc(&mut self, next_pc: Value) {
        self.pc = next_pc;
    }

    fn memory(&self) -> &SparseMemory {
        unimplemented!()
    }

    fn memory_mut(&mut self) -> &mut SparseMemory {
        unimplemented!()
    }

    fn registers(&self) -> &[Value] {
        &self.registers
    }

    fn set_register(&mut self, idx: usize, value: Value) {
        self.registers[idx] = value;
    }
}

impl Machine for BaselineJitCompilingMachine {
    fn ecall(&mut self) -> Result<(), Error> {
        unimplemented!()
    }

    fn ebreak(&mut self) -> Result<(), Error> {
        unimplemented!()
    }
}
