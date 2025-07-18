use std::ops::Div;



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Instruction {
    ADV = 0,
    BXL = 1,
    BST = 2,
    JNZ = 3,
    BXC = 4,
    OUT = 5,
    BDV = 6,
    CDV = 7,
}
impl Instruction {
    pub fn from(n: u8) -> Instruction{
        use Instruction::*;
        assert!(n < 8);
        match n {
            0 => ADV, 1 => BXL, 2 => BST, 3 => JNZ,
            4 => BXC, 5 => OUT, 6 => BDV, 7 => CDV,
            _ => unreachable!()
        }
    }
}


#[derive(Debug, Clone)]
pub struct Computer {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,

    intsr_ptr: usize,
    /// input contains instructions AND operands in a vector  
    /// Gurantess on creation that all elements are < 8.
    input: Vec<u8>,

    has_output_something: bool,
    is_running: bool,

    output: String,
}

impl Computer {
    /// returns true if output is a substring of full from the left only
    /// 
    /// e.g. true if output currently is `2,3,4` and full is `2,3,4,1`
    ///      false if output is currently `2,3,4` and full is `1,2,3,4`
    pub fn output_coult_match(&self, full: &String) -> bool {
        full[0..self.output.len()] == self.output
    }

    /// Execute all instructions until is_running is false
    pub fn run_until_not_match(&mut self, full: &String) {
        while self.is_running && self.output_coult_match(full){
            self.execute_next_instr();
        }
    }

    /// Execute all instructions until is_running is false
    pub fn run_to_completion(&mut self) {
        while self.is_running {
            self.execute_next_instr();
        }
    }

    /// Returns the current output of the machine
    pub fn get_output(&self) -> String {
        self.output.clone()
    }

    /// Assumes operand is a number in range [0,7)
    pub fn operand_to_combo(&self, operand: u8) -> u64 {
        assert!(operand < 7);
        match operand {
            0 => 0, 1 => 1, 2=> 2, 3 => 3,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => {panic!("7 should never appear as an operand")},
            _ => unreachable!()
        }
    }

    pub fn new(reg_a: u64, reg_b: u64, reg_c: u64, input: Vec<u8>) -> Computer {
        Computer {
            reg_a, reg_b, reg_c,
            intsr_ptr: 0, input,
            has_output_something: false,
            is_running: true,
            output: String::new()}
    }

    /// attempts to execute the next instruction and increment machine state.  
    /// Returns:
    ///     `true` if the machine ran the instruction.  
    ///     `false` if the machine failed to run the instruction and is now halted
    pub fn execute_next_instr(&mut self) -> bool {
        if let Some((opcode, operand)) = self.try_read_curr_code_and_operand() {
            self.perform_operation(opcode, operand);
        }
        self.is_running
    }

    /// tries to read current opcode and operation.
    /// If `instr_ptr` is out of bounds, or 1 away from out of bounds, the computer halts and `None` is returned.
    fn try_read_curr_code_and_operand(&mut self) -> Option<(u8, u8)> {
        if self.intsr_ptr >= self.input.len()-1 {
            self.is_running = false;
            None
        } else {
            Some((self.input[self.intsr_ptr], self.input[self.intsr_ptr+1]))
        }
    }

    /// Assumes that opcode and operand are 0-7
    fn perform_operation(&mut self, opcode: u8, operand: u8) {
        assert!(opcode < 8); assert!(operand < 8);
        use Instruction::*;
        match Instruction::from(opcode) {
            ADV => {self.adv(operand); self.instr_ptr_plus_2();},
            BXL => {self.bxl(operand); self.instr_ptr_plus_2();},
            BST => {self.bst(operand); self.instr_ptr_plus_2();},
            JNZ => {if !self.jnz(operand) {self.instr_ptr_plus_2();} },
            BXC => {self.bxc(operand); self.instr_ptr_plus_2();},
            OUT => {self.out(operand); self.instr_ptr_plus_2();},
            BDV => {self.bdv(operand); self.instr_ptr_plus_2();},
            CDV => {self.cdv(operand); self.instr_ptr_plus_2();},
        }
    }

    fn instr_ptr_plus_2(&mut self) {
        self.intsr_ptr += 2;
    }

    /// operand is a raw combo operand
    fn adv(&mut self, operand: u8) {
        self.reg_a = self.dv(operand);
    }

    /// operand is a literal operand
    fn bxl(&mut self, operand: u8) {
        self.reg_b = self.reg_b ^ (operand as u64);
    }

    /// operand is a raw combo operand
    fn bst(&mut self, operand: u8) {
        self.reg_b = self.operand_to_combo(operand) % 8;
    }

    /// operand is a literal operand.  
    /// returns true if jumped
    fn jnz(&mut self, operand: u8) -> bool {
        if self.reg_a != 0 {
            self.intsr_ptr = operand as usize;
            true
        } else { false }
    }

    /// operand doesn't matter
    fn bxc(&mut self, _operand: u8) {
        self.reg_b = self.reg_b ^ self.reg_c;
    }

    /// operand is a literal operand
    fn out(&mut self, operand: u8) {
        if self.has_output_something {
            self.output.push_str(",");
        } else {
            self.has_output_something = true;
        }
        let output: u64 = self.operand_to_combo(operand) % 8;
        self.output.push_str(&format!("{}", output).to_string());
    }

    /// operand is raw combo operand
    fn bdv(&mut self, operand: u8) {
        self.reg_b = self.dv(operand);
    }

    /// operand is raw combo operand
    fn cdv(&mut self, operand: u8) {
        self.reg_c = self.dv(operand);
    }

    /// operand is a raw combo operand
    /// numerator is assumed to be register A
    fn dv(&self, operand: u8) -> u64 {
        let operand: u64 = self.operand_to_combo(operand);
        self.reg_a.div(2_u32.pow(operand as u32) as u64)
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_spec() {

        let mut comp: Computer = Computer::new(729, 0, 0, vec![0,1,5,4,3,0]);
        comp.run_to_completion();
        assert_eq!(comp.get_output(), "4,6,3,5,6,3,5,2,1,0");
    }
}