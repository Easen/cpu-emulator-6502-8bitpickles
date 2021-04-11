use bytes::{Bytes, BytesMut};
fn main() {
    println!("Hello, world!");
}

trait Executable {
    fn execute(&self, cpu: &mut Cpu, program: &Bytes);
}

struct BRK {}
impl Executable for BRK {
    fn execute(&self, cpu: &mut Cpu, _program: &Bytes) {
        cpu.halt();
    }
}
struct LDA {}
impl Executable for LDA {
    fn execute(&self, cpu: &mut Cpu, program: &Bytes) {
        let value = program[cpu.program_counter + 1];
        cpu.register_a = value;
        cpu.program_counter = cpu.program_counter + 2
    }
}

struct ADC {}
impl Executable for ADC {
    fn execute(&self, cpu: &mut Cpu, program: &Bytes) {
        let value = program[cpu.program_counter + 1];
        cpu.register_a = cpu.register_a + value;
        cpu.program_counter = cpu.program_counter + 2;
    }
}

struct STA {}
impl Executable for STA {
    fn execute(&self, cpu: &mut Cpu, program: &Bytes) {
        let mem_location: u8 = program[cpu.program_counter + 1];
        cpu.memory[mem_location as usize] = cpu.register_a;
        cpu.program_counter = cpu.program_counter + 2
    }
}

#[derive(Debug)]
enum LookupError {
    OpCodeNotFound,
}
type LookupResult = Result<Box<dyn Executable>, LookupError>;
struct CpuInstruction {}
impl CpuInstruction {
    fn lookup_op_code(op_code: u8) -> LookupResult {
        match op_code {
            b'\x00' => Ok(Box::new(BRK {})),
            b'\x01' => Ok(Box::new(LDA {})),
            b'\x02' => Ok(Box::new(ADC {})),
            b'\x03' => Ok(Box::new(STA {})),
            _ => Err(LookupError::OpCodeNotFound),
        }
    }
}

#[derive(Debug)]
struct Cpu {
    pub program_counter: usize,
    pub register_a: u8,
    memory: BytesMut,
    halt: bool,
}
impl Cpu {
    #[inline]
    #[allow(dead_code)]
    pub fn with_memory(memory_size: usize) -> Cpu {
        let mut memory = BytesMut::with_capacity(memory_size);
        unsafe {
            memory.set_len(memory_size);
        }
        Cpu {
            program_counter: 0,
            register_a: 0,
            memory: memory,
            halt: false,
        }
    }
    fn halt(&mut self) {
        self.halt = true;
    }

    #[allow(dead_code)]
    fn run_program(&mut self, program: &Bytes) {
        while self.halt == false && self.program_counter < program.len() {
            let op_code = program[self.program_counter];
            CpuInstruction::lookup_op_code(op_code)
                .unwrap()
                .execute(self, program);
        }
    }
}

#[cfg(test)]
mod tests {

    use bytes::{BufMut, BytesMut};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_dojo1() {
        //https://raw.githubusercontent.com/timpickles/cpu-dojo/master/resources/worksheets/CPU%20Dojo%201%20-%20Introduction.pdf
        let mut program = BytesMut::with_capacity(1024);
        program.put(&b"\x01\x64"[..]); //LDA #$64
        program.put(&b"\x02\x07"[..]); //ADC #$07
        program.put(&b"\x03\x0F"[..]); //STA $15
        program.put(&b"\x00"[..]); // BRK
        let mut cpu = Cpu::with_memory(16);
        cpu.run_program(&program.freeze());
        assert_eq!(6, cpu.program_counter);
        assert_eq!(107, cpu.memory[15]);
    }
}
