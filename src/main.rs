use bytes::{Bytes, BytesMut};
fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Cpu {
    pub program_counter: usize,
    pub register_a: u8,
    memory: BytesMut,
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
        }
    }
    #[allow(dead_code)]
    fn run_program(&mut self, program: &Bytes) {
        let mut halt = false;
        while halt == false && self.program_counter < program.len() {
            let op_code = program[self.program_counter];
            match op_code {
                b'\x00' => halt = true,

                b'\x01' => {
                    let value = program[self.program_counter + 1];
                    self.register_a = value;
                    self.program_counter = self.program_counter + 2
                }
                b'\x02' => {
                    let value = program[self.program_counter + 1];
                    self.register_a = self.register_a + value;
                    self.program_counter = self.program_counter + 2;
                }
                b'\x03' => {
                    let mem_location: u8 = program[self.program_counter + 1];
                    self.memory[mem_location as usize] = self.register_a;
                    self.program_counter = self.program_counter + 2
                }
                _ => {
                    print!("Unknown opcode");
                    self.program_counter = self.program_counter + 1
                }
            }
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
