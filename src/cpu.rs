pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    /*
     * Track current position in the program
     */
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
        }
    }

    /*
     * LDA (0xA9)
     * Loads a byte of memory into the accumulator setting the zero and negative flags as appropriate.
     */
    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    /**
     * TAX (0xAA)
     * Copies the current contents of the accumulator into the X register and sets the zero and negative flags as appropriate.
     */
    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    /**
     * INX (0xE8)
     * Adds one to the X register setting the zero and negative flags as appropriate.
     */
    pub fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        // set zero flag
        // xxxx_xx{ZERO_FLAG}x
        if result == 0 {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }

        // set negaive flag (if bit 7 of A is set)
        if result & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }

    /**
     * self must be mutable as we need to update `register_a`
     */
    pub fn interpret(&mut self, program: Vec<u8>) {
        /*
         * CPU cycle:
         *  1. Fetch next instruction
         *  2. Decode the instruction
         *  3. Execute the instruction
         *  4. Repeat the cycle
         */
        self.program_counter = 0;

        loop {
            let opscode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opscode {
                 // BRK
                0x00 => return,
                 // LDA
                0xA9 => {
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;

                    self.lda(param);
                }
                 // TAX
                0xAA => self.tax(),
                // INX
                0xE8 => self.inx(),
                _ => todo!()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let mut cpu = CPU::new();
        // LDA 0x05
        // BRK
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        // LDA 0x00
        // BRK
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        // LDX
        // BRK
        cpu.interpret(vec![0xaa, 0x00]);
        assert_eq!(cpu.register_x, 10);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        // LDA 0xC0
        // LDX
        // INX
        // BRK
        cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1);
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        // INX
        // INX
        // BRK
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 1);
    }
}
