mod machine;
mod value;

use self::{machine::BaselineJitCompilingMachine, value::Value};
use crate::{
    decoder::build_imac_decoder,
    instructions::{i, rvc},
    CoreMachine, DefaultMachine, Error, Instruction, Memory, Register,
};
use libc::{c_int, c_uint, c_void, size_t, uint64_t};
use memmap::MmapMut;
use std::mem;

fn is_basic_block_end_instruction(i: &Instruction) -> bool {
    match i {
        Instruction::I(i) => match i {
            i::Instruction::I(i) => match i.inst() {
                i::ItypeInstruction::JALR => true,
                _ => false,
            },
            i::Instruction::B(_) => true,
            i::Instruction::Env(_) => true,
            i::Instruction::JAL { .. } => true,
            _ => false,
        },
        Instruction::RVC(i) => match i {
            rvc::Instruction::BEQZ { .. } => true,
            rvc::Instruction::BNEZ { .. } => true,
            rvc::Instruction::JAL { .. } => true,
            rvc::Instruction::J { .. } => true,
            rvc::Instruction::JR { .. } => true,
            rvc::Instruction::JALR { .. } => true,
            rvc::Instruction::EBREAK => true,
            _ => false,
        },
        Instruction::M(_) => false,
    }
}

fn instruction_length(i: &Instruction) -> usize {
    match i {
        Instruction::RVC(_) => 2,
        _ => 4,
    }
}

// This is a C struct type
enum AsmContext {}

extern "C" {
    fn asm_new() -> *mut AsmContext;
    fn asm_finalize(c: *mut AsmContext);
    fn asm_setup(c: *mut AsmContext) -> c_int;
    fn asm_emit_prologue(c: *mut AsmContext) -> c_int;
    fn asm_emit_epilogue(c: *mut AsmContext) -> c_int;
    fn asm_mov(c: *mut AsmContext, reg: c_uint, value: uint64_t) -> c_int;
    fn asm_link(c: *mut AsmContext, szp: *mut size_t) -> c_int;
    fn asm_encode(c: *mut AsmContext, buffer: *mut c_void) -> c_int;
}

/// A JIT-based machine runner, the design here is to provide a 2-level JIT:
/// * A baseline JIT leveraging similar techniques in qemu's TCG and rv8 to
/// translate RISC-V instructions to native assembly code. Since this level
/// serves as the baseline JIT, JIT compilation speed will take priority over
/// runtime performance of generated code. As a result, only certain but not
/// all optimizations in rv8 would be introduced here, such as macro-op fusion.
/// The baseline JIT here will only work on a basic block boundary.
/// A static register allocation algorithm much like the rv8 one will also be
/// used here. To help with the next level JIT, trace points would also be
/// introduced here for profiling use.
/// * For very hot code, we might introduce a more sophisticated JIT to
/// further optimize those code pieces to further boost the performance. In
/// this level we would leverage algorithms to translate RISC-V instructions
/// to SSA form: http://compilers.cs.uni-saarland.de/papers/bbhlmz13cc.pdf, then
/// apply different optimizations to further optimize the code. We might choose
/// to leverage cranelift or MJIT to enjoy existing work. Note that unlike
/// the above baseline JIT, this path still has many uncertainties which is more
/// likely to change. Also this will has much lower priority if baseline JIT
/// is proved to be enough for CKB use.
pub fn run<'a, R: Register, M: Memory>(
    machine: &mut DefaultMachine<'a, R, M>,
) -> Result<u8, Error> {
    let asm = unsafe { asm_new() };
    let decoder = build_imac_decoder::<R>();
    machine.set_running(true);
    while machine.running() {
        // For now, we implement a TCG style to aid debugging, profiling code
        // will be added later.
        let pc = machine.pc().to_usize();
        // Fetch next basic block
        let mut current_pc = pc;
        let mut instructions = Vec::new();
        loop {
            let instruction = decoder.decode(machine.memory_mut(), current_pc)?;
            let end_instruction = is_basic_block_end_instruction(&instruction);
            current_pc += instruction_length(&instruction);
            instructions.push(instruction);

            if end_instruction {
                break;
            }
        }
        println!("Block to compile: {:16x}:", pc);
        println!("  Basic block size: {}", instructions.len());
        for inst in &instructions {
            println!("    {}", inst);
        }
        let mut compiling_machine = BaselineJitCompilingMachine::new(pc);
        for i in &instructions {
            i.execute(&mut compiling_machine)?;
        }
        unsafe {
            asm_setup(asm);
            asm_emit_prologue(asm);
        }
        for (reg, value) in compiling_machine.registers().iter().enumerate() {
            match value {
                Value::Register(reg) => (),
                Value::Imm(imm) => unsafe {
                    asm_mov(asm, reg as u32, *imm);
                },
                _ => unimplemented!(),
            }
        }
        let mut buffer_size: usize = 0;
        unsafe {
            asm_emit_epilogue(asm);
            asm_link(asm, &mut buffer_size);
        }
        let mut buffer = MmapMut::map_anon(buffer_size)?;
        unsafe {
            asm_encode(asm, buffer[..].as_mut_ptr() as *mut c_void);
        }
        let strs: Vec<String> = buffer[..].iter().map(|b| format!("{:02X}", b)).collect();
        let hex_string = strs.connect(" ");
        println!("{}", hex_string);
        let executable_buffer = buffer.make_exec()?;
        let f: fn(&mut [R]) = unsafe {
            mem::transmute(&(&executable_buffer[..])[0] as *const u8)
        };
        f(machine.registers_mut());
        println!("Machine status after first basic block: {}", machine);
        break;
    }
    unsafe {
        asm_finalize(asm);
    }
    Ok(machine.exit_code())
}
