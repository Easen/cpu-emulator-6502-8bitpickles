fn main() {
    println!("Hello, world!");
}

trait Executable {
    fn execute(&self, cpu: &mut Cpu);
}

struct BRK {}
impl Executable for BRK {
    fn execute(&self, cpu: &mut Cpu) {
        cpu.halt();
    }
}
struct LDA {}
impl Executable for LDA {
    fn execute(&self, cpu: &mut Cpu) {
        cpu.advance();
        cpu.register_a = cpu.current();
    }
}

struct ADC {}
impl Executable for ADC {
    fn execute(&self, cpu: &mut Cpu) {
        cpu.advance();
        cpu.register_a += cpu.current();
    }
}

struct STA {}
impl Executable for STA {
    fn execute(&self, cpu: &mut Cpu) {
        cpu.advance();
        let mem_location: i32 = cpu.current();
        cpu.memory[mem_location as usize] = cpu.register_a;
    }
}
struct LDX {}
impl Executable for LDX {
    fn execute(&self, cpu: &mut Cpu) {
        cpu.advance();
        cpu.register_x = cpu.current();
    }
}
struct INX {}
impl Executable for INX {
    fn execute(&self, cpu: &mut Cpu) {
        cpu.register_x += 1;
    }
}
struct CMY {}
impl Executable for CMY {
    fn execute(&self, cpu: &mut Cpu) {
        cpu.advance();
        cpu.flags = cpu.current() == cpu.register_y;
    }
}
struct BNE {}
impl Executable for BNE {
    fn execute(&self, cpu: &mut Cpu) {
        cpu.advance();
        if cpu.flags == false {
            cpu.program_counter = (cpu.program_counter as i32 - 1 + cpu.current()) as usize;
        }
    }
}
#[allow(non_camel_case_types)]
struct STA_X {}
impl Executable for STA_X {
    fn execute(&self, cpu: &mut Cpu) {
        cpu.memory[cpu.register_x as usize] = cpu.register_a;
    }
}
struct DEY {}
impl Executable for DEY {
    fn execute(&self, cpu: &mut Cpu) {
        cpu.register_y += -1;
    }
}
struct LDY {}
impl Executable for LDY {
    fn execute(&self, cpu: &mut Cpu) {
        cpu.advance();
        cpu.register_y = cpu.current();
    }
}
struct JSR {}
impl Executable for JSR {
    fn execute(&self, cpu: &mut Cpu) {
        cpu.advance();
        cpu.memory[cpu.stack_pointer] = cpu.program_counter as i32;
        cpu.stack_pointer -= 1;
        cpu.program_counter = (cpu.current() - 1).min(0) as usize;
    }
}
struct RTS {}
impl Executable for RTS {
    fn execute(&self, cpu: &mut Cpu) {
        cpu.stack_pointer += 1;
        cpu.program_counter = cpu.memory[cpu.stack_pointer].min(0) as usize + 1;
    }
}

#[derive(Debug)]
enum LookupError {
    OpCodeNotFound,
}

trait CpuInstructionSet {
    fn lookup_op_code(&self, op_code: i32) -> LookupResult;
}

type LookupResult = Result<Box<dyn Executable>, LookupError>;
#[derive(Debug)]
struct MOS6502CpuInstructionSet {}
impl CpuInstructionSet for MOS6502CpuInstructionSet {
    fn lookup_op_code(&self, op_code: i32) -> LookupResult {
        match op_code {
            0 => Ok(Box::new(BRK {})),
            1 => Ok(Box::new(LDA {})),
            2 => Ok(Box::new(ADC {})),
            3 => Ok(Box::new(STA {})),
            4 => Ok(Box::new(LDX {})),
            5 => Ok(Box::new(INX {})),
            6 => Ok(Box::new(CMY {})),
            7 => Ok(Box::new(BNE {})),
            8 => Ok(Box::new(STA_X {})),
            9 => Ok(Box::new(DEY {})),
            10 => Ok(Box::new(LDY {})),
            11 => Ok(Box::new(JSR {})),
            12 => Ok(Box::new(RTS {})),
            _ => Err(LookupError::OpCodeNotFound),
        }
    }
}

#[derive(Debug)]
enum CpuError {
    ProgramExceedsMemory,
}

struct Cpu {
    pub program_counter: usize,
    pub stack_pointer: usize,
    pub register_a: i32,
    pub register_x: i32,
    pub register_y: i32,
    pub flags: bool,
    memory: Vec<i32>,
    halt: bool,
    cpu_instruction_set: Box<dyn CpuInstructionSet>,
}
impl Cpu {
    #[inline]
    #[allow(dead_code)]
    pub fn new() -> Cpu {
        let memory = vec![0; 512];
        Cpu {
            program_counter: 0,
            stack_pointer: memory.len() - 1,
            register_a: 0,
            register_x: 0,
            register_y: 0,
            flags: false,
            memory: memory,
            halt: false,
            cpu_instruction_set: Box::new(MOS6502CpuInstructionSet {}),
        }
    }

    fn halt(&mut self) {
        self.halt = true;
    }

    fn advance(&mut self) {
        self.program_counter = self.program_counter + 1;
    }

    fn current(&mut self) -> i32 {
        self.memory[self.program_counter]
    }

    #[allow(dead_code)]
    fn load_program(&mut self, program: &[i32]) -> Result<(), CpuError> {
        if program.len() > self.memory.len() {
            return Err(CpuError::ProgramExceedsMemory);
        }

        self.memory[..program.len()].copy_from_slice(&program);
        Ok(())
    }

    #[allow(dead_code)]
    fn run(&mut self) {
        while self.halt == false && self.program_counter < self.memory.len() {
            let op_code = self.memory[self.program_counter];
            self.cpu_instruction_set
                .lookup_op_code(op_code)
                .unwrap()
                .execute(self);
            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_dojo1() {
        // https://raw.githubusercontent.com/timpickles/cpu-dojo/master/resources/worksheets/CPU%20Dojo%201%20-%20Introduction.pdf
        let program = [
            0x01, 0x64, //LDA #$64
            0x02, 0x07, //ADC #$07
            0x03, 0x0f, //STA $15
            0x00, // BRK
        ];
        let mut cpu = Cpu::new();
        cpu.load_program(&program).unwrap();
        cpu.run();
        assert_eq!(7, cpu.program_counter);
        assert_eq!(107, cpu.memory[15]);
    }

    #[test]
    fn test_dojo2() {
        // https://raw.githubusercontent.com/timpickles/cpu-dojo/master/resources/worksheets/CPU%20Dojo%202%20-%20Branching.pdf
        let program = vec![
            4, 128, 1, 0x77, 8, 5, 1, 0x68, 8, 5, 1, 0x6F, 8, 5, 1, 0x20, 8, 5, 1, 0x6c, 8, 5, 1,
            0x65, 8, 5, 1, 0x74, 8, 5, 1, 0x20, 8, 5, 1, 0x74, 8, 5, 1, 0x68, 8, 5, 1, 0x65, 8, 5,
            1, 0x20, 8, 5, 1, 0x64, 8, 5, 1, 0x6F, 8, 5, 1, 0x67, 8, 5, 1, 0x73, 8, 5, 1, 0x20, 8,
            5, 1, 0x6F, 8, 5, 1, 0x75, 8, 5, 1, 0x74, 8, 5, 1, 0x20, 8, 5, 10, 3, 1, 0x77, 8, 5, 1,
            0x68, 8, 5, 1, 0x6F, 8, 5, 1, 0x20, 8, 5, 9, 6, 0, 7, -20, 0,
        ];
        let mut cpu = Cpu::new();
        cpu.load_program(&program).unwrap();
        cpu.run();
        let string: String = cpu.memory[128..255]
            .into_iter()
            .filter(|x| x.is_positive())
            .map(|d| d.abs())
            .map(|d| std::char::from_u32(d as u32))
            .filter(|r| r.is_some())
            .map(|r| r.unwrap())
            .collect();
        assert_eq!("who let the dogs out who who who ", string);
    }

    #[test]
    fn test_dojo3() {
        // https://raw.githubusercontent.com/timpickles/cpu-dojo/master/resources/worksheets/CPU%20Dojo%203%20-%20Subroutines.pdf
        let program = [
            11, 3, // JSR 3
            0, // BRK
            1, 0x64, //LDA #$64
            2, 0x07, //ADC #$07
            3, 0x0f, //STA $15
            12,   // RST
        ];
        let mut cpu = Cpu::new();
        cpu.load_program(&program).unwrap();
        cpu.run();
        assert_eq!(3, cpu.program_counter);
        assert_eq!(107, cpu.memory[15]);
    }
}
