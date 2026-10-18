use std::collections::HashMap;

#[derive(Debug)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}

pub struct OpCode {
    pub code: u8,
    pub name: &'static str,
    pub len: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl OpCode {
    fn new(code: u8, name: &'static str, len: u8, cycles: u8, mode: AddressingMode) -> Self {
        OpCode {
            code,
            name,
            len,
            cycles,
            mode,
        }
    }
}

pub struct OpCodeTable {
    pub opcodes: HashMap<u8, OpCode>,
}
impl OpCodeTable {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        let codes = vec![
            OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing),
            OpCode::new(0xEA, "NOP", 1, 2, AddressingMode::NoneAddressing),
            
            OpCode::new(0xA9, "LDA", 2, 2, AddressingMode::Immediate),
            OpCode::new(0xA5, "LDA", 2, 3, AddressingMode::ZeroPage),
            OpCode::new(0xB5, "LDA", 2, 4, AddressingMode::ZeroPage_X),
            OpCode::new(0xAD, "LDA", 3, 4, AddressingMode::Absolute),
            OpCode::new(0xBD, "LDA", 3, 4, AddressingMode::Absolute_X),
            OpCode::new(0xB9, "LDA", 3, 4, AddressingMode::Absolute_Y),
            OpCode::new(0xA1, "LDA", 2, 6, AddressingMode::Indirect_X),
            OpCode::new(0xB1, "LDA", 2, 5, AddressingMode::Indirect_Y),
            
            OpCode::new(0xA2, "LDX", 2, 2, AddressingMode::Immediate),
            OpCode::new(0xA6, "LDX", 2, 3, AddressingMode::ZeroPage),
            OpCode::new(0xB6, "LDX", 2, 4, AddressingMode::ZeroPage_Y),
            OpCode::new(0xAE, "LDX", 3, 4, AddressingMode::Absolute),
            OpCode::new(0xBE, "LDX", 3, 4, AddressingMode::Absolute_Y),
            
            OpCode::new(0xA0, "LDY", 2, 2, AddressingMode::Immediate),
            OpCode::new(0xA4, "LDY", 2, 3, AddressingMode::ZeroPage),
            OpCode::new(0xB4, "LDY", 2, 4, AddressingMode::ZeroPage_X),
            OpCode::new(0xAC, "LDY", 3, 4, AddressingMode::Absolute),
            OpCode::new(0xBC, "LDY", 3, 4, AddressingMode::Absolute_X),
            
            OpCode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage),
            OpCode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPage_X),
            OpCode::new(0x8D, "STA", 3, 4, AddressingMode::Absolute),
            OpCode::new(0x9D, "STA", 3, 4, AddressingMode::Absolute_X),
            OpCode::new(0x99, "STA", 3, 4, AddressingMode::Absolute_Y),
            OpCode::new(0x81, "STA", 2, 6, AddressingMode::Indirect_X),
            OpCode::new(0x91, "STA", 2, 6, AddressingMode::Indirect_Y),
            
            OpCode::new(0x86, "STX", 2, 3, AddressingMode::ZeroPage),
            OpCode::new(0x96, "STX", 2, 4, AddressingMode::ZeroPage_Y),
            OpCode::new(0x8E, "STX", 3, 4, AddressingMode::Absolute),
            OpCode::new(0x84, "STY", 2, 3, AddressingMode::ZeroPage),
            OpCode::new(0x94, "STY", 2, 4, AddressingMode::ZeroPage_X),
            OpCode::new(0x8C, "STY", 3, 4, AddressingMode::Absolute),
            
            OpCode::new(0xAA, "TAX", 1, 2, AddressingMode::NoneAddressing),
            OpCode::new(0x8A, "TXA", 1, 2, AddressingMode::NoneAddressing),
            OpCode::new(0xA8, "TAY", 1, 2, AddressingMode::NoneAddressing),
            OpCode::new(0x98, "TYA", 1, 2, AddressingMode::NoneAddressing),
            OpCode::new(0xBA, "TSX", 1, 2, AddressingMode::NoneAddressing),
            OpCode::new(0x9A, "TXS", 1, 2, AddressingMode::NoneAddressing),
            
            OpCode::new(0x18, "CLC", 1, 2, AddressingMode::NoneAddressing),
            OpCode::new(0x38, "SEC", 1, 2, AddressingMode::NoneAddressing),
            OpCode::new(0x58, "CLI", 1, 2, AddressingMode::NoneAddressing),
            OpCode::new(0x78, "SEI", 1, 2, AddressingMode::NoneAddressing),
            OpCode::new(0xB8, "CLV", 1, 2, AddressingMode::NoneAddressing),
            OpCode::new(0xD8, "CLD", 1, 2, AddressingMode::NoneAddressing),
            OpCode::new(0xF8, "SED", 1, 2, AddressingMode::NoneAddressing),
            
            OpCode::new(0x48, "PHA", 1, 3, AddressingMode::NoneAddressing),
            OpCode::new(0x68, "PLA", 1, 4, AddressingMode::NoneAddressing),
            OpCode::new(0x08, "PHP", 1, 3, AddressingMode::NoneAddressing),
            OpCode::new(0x28, "PLP", 1, 4, AddressingMode::NoneAddressing),
            
            OpCode::new(0x69, "ADC", 2, 2, AddressingMode::Immediate),
            OpCode::new(0x65, "ADC", 2, 3, AddressingMode::ZeroPage),
            OpCode::new(0x75, "ADC", 2, 4, AddressingMode::ZeroPage_X),
            OpCode::new(0x6D, "ADC", 3, 4, AddressingMode::Absolute),
            OpCode::new(0x7D, "ADC", 3, 4, AddressingMode::Absolute_X),
            OpCode::new(0x79, "ADC", 3, 4, AddressingMode::Absolute_Y),
            OpCode::new(0x61, "ADC", 2, 6, AddressingMode::Indirect_X),
            OpCode::new(0x71, "ADC", 2, 5, AddressingMode::Indirect_Y),
            
            OpCode::new(0xE9, "SBC", 2, 2, AddressingMode::Immediate),
            OpCode::new(0xE5, "SBC", 2, 3, AddressingMode::ZeroPage),
            OpCode::new(0xF5, "SBC", 2, 4, AddressingMode::ZeroPage_X),
            OpCode::new(0xED, "SBC", 3, 4, AddressingMode::Absolute),
            OpCode::new(0xFD, "SBC", 3, 4, AddressingMode::Absolute_X),
            OpCode::new(0xF9, "SBC", 3, 4, AddressingMode::Absolute_Y),
            OpCode::new(0xE1, "SBC", 2, 6, AddressingMode::Indirect_X),
            OpCode::new(0xF1, "SBC", 2, 5, AddressingMode::Indirect_Y),
            
            OpCode::new(0x29, "AND", 2, 2, AddressingMode::Immediate),
            OpCode::new(0x25, "AND", 2, 3, AddressingMode::ZeroPage),
            OpCode::new(0x35, "AND", 2, 4, AddressingMode::ZeroPage_X),
            OpCode::new(0x2D, "AND", 3, 4, AddressingMode::Absolute),
            OpCode::new(0x3D, "AND", 3, 4, AddressingMode::Absolute_X),
            OpCode::new(0x39, "AND", 3, 4, AddressingMode::Absolute_Y),
            OpCode::new(0x21, "AND", 2, 6, AddressingMode::Indirect_X),
            OpCode::new(0x31, "AND", 2, 5, AddressingMode::Indirect_Y),
            OpCode::new(0x09, "ORA", 2, 2, AddressingMode::Immediate),
            OpCode::new(0x05, "ORA", 2, 3, AddressingMode::ZeroPage),
            OpCode::new(0x15, "ORA", 2, 4, AddressingMode::ZeroPage_X),
            OpCode::new(0x0D, "ORA", 3, 4, AddressingMode::Absolute),
            OpCode::new(0x1D, "ORA", 3, 4, AddressingMode::Absolute_X),
            OpCode::new(0x19, "ORA", 3, 4, AddressingMode::Absolute_Y),
            OpCode::new(0x01, "ORA", 2, 6, AddressingMode::Indirect_X),
            OpCode::new(0x11, "ORA", 2, 5, AddressingMode::Indirect_Y),
            OpCode::new(0x49, "EOR", 2, 2, AddressingMode::Immediate),
            OpCode::new(0x45, "EOR", 2, 3, AddressingMode::ZeroPage),
            OpCode::new(0x55, "EOR", 2, 4, AddressingMode::ZeroPage_X),
            OpCode::new(0x4D, "EOR", 3, 4, AddressingMode::Absolute),
            OpCode::new(0x5D, "EOR", 3, 4, AddressingMode::Absolute_X),
            OpCode::new(0x59, "EOR", 3, 4, AddressingMode::Absolute_Y),
            OpCode::new(0x41, "EOR", 2, 6, AddressingMode::Indirect_X),
            OpCode::new(0x51, "EOR", 2, 5, AddressingMode::Indirect_Y),
            
            OpCode::new(0xE8, "INX", 1, 2, AddressingMode::NoneAddressing),
            OpCode::new(0xC8, "INY", 1, 2, AddressingMode::NoneAddressing),
            OpCode::new(0xCA, "DEX", 1, 2, AddressingMode::NoneAddressing),
            OpCode::new(0x88, "DEY", 1, 2, AddressingMode::NoneAddressing),
            OpCode::new(0xE6, "INC", 2, 5, AddressingMode::ZeroPage),
            OpCode::new(0xF6, "INC", 2, 6, AddressingMode::ZeroPage_X),
            OpCode::new(0xEE, "INC", 3, 6, AddressingMode::Absolute),
            OpCode::new(0xFE, "INC", 3, 7, AddressingMode::Absolute_X),
            OpCode::new(0xC6, "DEC", 2, 5, AddressingMode::ZeroPage),
            OpCode::new(0xD6, "DEC", 2, 6, AddressingMode::ZeroPage_X),
            OpCode::new(0xCE, "DEC", 3, 6, AddressingMode::Absolute),
            OpCode::new(0xDE, "DEC", 3, 7, AddressingMode::Absolute_X),
            
            OpCode::new(0xC9, "CMP", 2, 2, AddressingMode::Immediate),
            OpCode::new(0xC5, "CMP", 2, 3, AddressingMode::ZeroPage),
            OpCode::new(0xD5, "CMP", 2, 4, AddressingMode::ZeroPage_X),
            OpCode::new(0xCD, "CMP", 3, 4, AddressingMode::Absolute),
            OpCode::new(0xDD, "CMP", 3, 4, AddressingMode::Absolute_X),
            OpCode::new(0xD9, "CMP", 3, 4, AddressingMode::Absolute_Y),
            OpCode::new(0xC1, "CMP", 2, 6, AddressingMode::Indirect_X),
            OpCode::new(0xD1, "CMP", 2, 5, AddressingMode::Indirect_Y),
            OpCode::new(0xE0, "CPX", 2, 2, AddressingMode::Immediate),
            OpCode::new(0xE4, "CPX", 2, 3, AddressingMode::ZeroPage),
            OpCode::new(0xEC, "CPX", 3, 4, AddressingMode::Absolute),
            OpCode::new(0xC0, "CPY", 2, 2, AddressingMode::Immediate),
            OpCode::new(0xC4, "CPY", 2, 3, AddressingMode::ZeroPage),
            OpCode::new(0xCC, "CPY", 3, 4, AddressingMode::Absolute),
            OpCode::new(0x24, "BIT", 2, 3, AddressingMode::ZeroPage),
            OpCode::new(0x2C, "BIT", 3, 4, AddressingMode::Absolute),
            
            OpCode::new(0x0A, "ASL", 1, 2, AddressingMode::NoneAddressing),
            OpCode::new(0x06, "ASL", 2, 5, AddressingMode::ZeroPage),
            OpCode::new(0x16, "ASL", 2, 6, AddressingMode::ZeroPage_X),
            OpCode::new(0x0E, "ASL", 3, 6, AddressingMode::Absolute),
            OpCode::new(0x1E, "ASL", 3, 7, AddressingMode::Absolute_X),
            OpCode::new(0x4A, "LSR", 1, 2, AddressingMode::NoneAddressing),
            OpCode::new(0x46, "LSR", 2, 5, AddressingMode::ZeroPage),
            OpCode::new(0x56, "LSR", 2, 6, AddressingMode::ZeroPage_X),
            OpCode::new(0x4E, "LSR", 3, 6, AddressingMode::Absolute),
            OpCode::new(0x5E, "LSR", 3, 7, AddressingMode::Absolute_X),
            
            OpCode::new(0x2A, "ROL", 1, 2, AddressingMode::NoneAddressing),
            OpCode::new(0x26, "ROL", 2, 5, AddressingMode::ZeroPage),
            OpCode::new(0x36, "ROL", 2, 6, AddressingMode::ZeroPage_X),
            OpCode::new(0x2E, "ROL", 3, 6, AddressingMode::Absolute),
            OpCode::new(0x3E, "ROL", 3, 7, AddressingMode::Absolute_X),
            OpCode::new(0x6A, "ROR", 1, 2, AddressingMode::NoneAddressing),
            OpCode::new(0x66, "ROR", 2, 5, AddressingMode::ZeroPage),
            OpCode::new(0x76, "ROR", 2, 6, AddressingMode::ZeroPage_X),
            OpCode::new(0x6E, "ROR", 3, 6, AddressingMode::Absolute),
            OpCode::new(0x7E, "ROR", 3, 7, AddressingMode::Absolute_X),
            
            OpCode::new(0x4C, "JMP", 3, 3, AddressingMode::Absolute),
            OpCode::new(0x6C, "JMP", 3, 5, AddressingMode::NoneAddressing),
            OpCode::new(0x20, "JSR", 3, 6, AddressingMode::Absolute),
            OpCode::new(0x60, "RTS", 1, 6, AddressingMode::NoneAddressing),
            OpCode::new(0x40, "RTI", 1, 6, AddressingMode::NoneAddressing),
            OpCode::new(0x90, "BCC", 2, 2, AddressingMode::NoneAddressing),
            OpCode::new(0xB0, "BCS", 2, 2, AddressingMode::NoneAddressing),
            OpCode::new(0xF0, "BEQ", 2, 2, AddressingMode::NoneAddressing),
            OpCode::new(0x30, "BMI", 2, 2, AddressingMode::NoneAddressing),
            OpCode::new(0xD0, "BNE", 2, 2, AddressingMode::NoneAddressing),
            OpCode::new(0x10, "BPL", 2, 2, AddressingMode::NoneAddressing),
            OpCode::new(0x50, "BVC", 2, 2, AddressingMode::NoneAddressing),
            OpCode::new(0x70, "BVS", 2, 2, AddressingMode::NoneAddressing),
        ];

        for opcode in codes {
            map.insert(opcode.code, opcode);
        }
        OpCodeTable { opcodes: map }
    }
}
pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub stack_pointer: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0b0010_0100,
            stack_pointer: 0xFF,
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }

    pub fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }
    pub fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub fn mem_read_u16(&self, addr: u16) -> u16 {
        let lo = self.mem_read(addr) as u16;
        let hi = self.mem_read(addr + 1) as u16;
        (hi << 8) | lo
    }

    pub fn mem_write_u16(&mut self, addr: u16, data: u16) {
        self.mem_write(addr, (data & 0xFF) as u8);
        self.mem_write(addr + 1, (data >> 8) as u8);
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.stack_pointer = 0xFF;
        self.status = 0b0010_0100;
        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn load(&mut self, program: Vec<u8>) {
        for (i, x) in program.iter().enumerate() {
            self.mem_write(0x8000 + i as u16, *x);
        }
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }
    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.program_counter);
                pos.wrapping_add(self.register_x) as u16
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.program_counter);
                pos.wrapping_add(self.register_y) as u16
            }
            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(self.program_counter);
                base.wrapping_add(self.register_x as u16)
            }
            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(self.program_counter);
                base.wrapping_add(self.register_y as u16)
            }
            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);
                let ptr = base.wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16) as u16;
                let hi = self.mem_read(ptr.wrapping_add(1) as u16) as u16;
                (hi << 8) | lo
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);
                let lo = self.mem_read(base as u16) as u16;
                let hi = self.mem_read(base.wrapping_add(1) as u16) as u16;
                let deref_base = (hi << 8) | lo;
                deref_base.wrapping_add(self.register_y as u16)
            }
            AddressingMode::NoneAddressing => panic!("Addressing mode not supported"),
        }
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status |= 0b0000_0010;
        } else {
            self.status &= 0b1111_1101;
        }
        if result & 0x80 != 0 {
            self.status |= 0b1000_0000;
        } else {
            self.status &= 0b0111_1111;
        }
    }
    fn stack_push(&mut self, data: u8) {
        self.mem_write(0x0100 + self.stack_pointer as u16, data);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }
    fn stack_pop(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        self.mem_read(0x0100 + self.stack_pointer as u16)
    }
    fn stack_push_u16(&mut self, data: u16) {
        self.stack_push((data >> 8) as u8);
        self.stack_push((data & 0xFF) as u8);
    }
    fn stack_pop_u16(&mut self) -> u16 {
        let lo = self.stack_pop() as u16;
        let hi = self.stack_pop() as u16;
        (hi << 8) | lo
    }
    fn set_carry(&mut self) {
        self.status |= 0b0000_0001;
    }
    fn clear_carry(&mut self) {
        self.status &= 0b1110_1110;
    }
    fn clc(&mut self) {
        self.status &= 0b1111_1110;
    }
    fn sec(&mut self) {
        self.status |= 0b0000_0001;
    }
    fn cli(&mut self) {
        self.status &= 0b1111_1011;
    }
    fn sei(&mut self) {
        self.status |= 0b0000_0100;
    }
    fn clv(&mut self) {
        self.status &= 0b1011_1111;
    }
    fn cld(&mut self) {
        self.status &= 0b1111_0111;
    }
    fn sed(&mut self) {
        self.status |= 0b0000_1000;
    }
    fn lda(&mut self, mode: &AddressingMode) {
        self.register_a = self.mem_read(self.get_operand_address(mode));
        self.update_zero_and_negative_flags(self.register_a);
    }
    fn ldx(&mut self, mode: &AddressingMode) {
        self.register_x = self.mem_read(self.get_operand_address(mode));
        self.update_zero_and_negative_flags(self.register_x);
    }
    fn ldy(&mut self, mode: &AddressingMode) {
        self.register_y = self.mem_read(self.get_operand_address(mode));
        self.update_zero_and_negative_flags(self.register_y);
    }
    fn sta(&mut self, mode: &AddressingMode) {
        self.mem_write(self.get_operand_address(mode), self.register_a);
    }
    fn stx(&mut self, mode: &AddressingMode) {
        self.mem_write(self.get_operand_address(mode), self.register_x);
    }
    fn sty(&mut self, mode: &AddressingMode) {
        self.mem_write(self.get_operand_address(mode), self.register_y);
    }
    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }
    fn txa(&mut self) {
        self.register_a = self.register_x;
        self.update_zero_and_negative_flags(self.register_a);
    }
    fn tay(&mut self) {
        self.register_y = self.register_a;
        self.update_zero_and_negative_flags(self.register_y);
    }
    fn tya(&mut self) {
        self.register_a = self.register_y;
        self.update_zero_and_negative_flags(self.register_a);
    }
    fn tsx(&mut self) {
        self.register_x = self.stack_pointer;
        self.update_zero_and_negative_flags(self.register_x);
    }
    fn txs(&mut self) {
        self.stack_pointer = self.register_x;
    }
    fn pha(&mut self) {
        self.stack_push(self.register_a);
    }
    fn pla(&mut self) {
        self.register_a = self.stack_pop();
        self.update_zero_and_negative_flags(self.register_a);
    }
    fn php(&mut self) {
        self.stack_push(self.status | 0b0011_0000);
    }
    fn plp(&mut self) {
        self.status = (self.stack_pop() & 0b1110_1111) | 0b0010_0000;
    }
    fn and(&mut self, mode: &AddressingMode) {
        self.register_a &= self.mem_read(self.get_operand_address(mode));
        self.update_zero_and_negative_flags(self.register_a);
    }
    fn ora(&mut self, mode: &AddressingMode) {
        self.register_a |= self.mem_read(self.get_operand_address(mode));
        self.update_zero_and_negative_flags(self.register_a);
    }
    fn eor(&mut self, mode: &AddressingMode) {
        self.register_a ^= self.mem_read(self.get_operand_address(mode));
        self.update_zero_and_negative_flags(self.register_a);
    }
    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }
    fn iny(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_y);
    }
    fn dex(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_x);
    }
    fn dey(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_y);
    }
    fn inc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let val = self.mem_read(addr).wrapping_add(1);
        self.mem_write(addr, val);
        self.update_zero_and_negative_flags(val);
    }
    fn dec(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let val = self.mem_read(addr).wrapping_sub(1);
        self.mem_write(addr, val);
        self.update_zero_and_negative_flags(val);
    }

    fn adc(&mut self, mode: &AddressingMode) {
        let val = self.mem_read(self.get_operand_address(mode));
        let carry = self.status & 1;
        let sum = self.register_a as u16 + val as u16 + carry as u16;
        if sum > 0xFF {
            self.set_carry();
        } else {
            self.clear_carry();
        }
        let res = sum as u8;
        if (val ^ res) & (self.register_a ^ res) & 0x80 != 0 {
            self.status |= 0b0100_0000;
        } else {
            self.status &= 0b1011_1111;
        }
        self.register_a = res;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn sbc(&mut self, mode: &AddressingMode) {
        let val = self.mem_read(self.get_operand_address(mode));
        let carry = self.status & 1;
        let negated = !val;
        let sum = self.register_a as u16 + negated as u16 + carry as u16;
        if sum > 0xFF {
            self.set_carry();
        } else {
            self.clear_carry();
        }
        let res = sum as u8;
        if (negated ^ res) & (self.register_a ^ res) & 0x80 != 0 {
            self.status |= 0b0100_0000;
        } else {
            self.status &= 0b1011_1111;
        }
        self.register_a = res;
        self.update_zero_and_negative_flags(self.register_a);
    }
    fn cmp(&mut self, mode: &AddressingMode) {
        let val = self.mem_read(self.get_operand_address(mode));
        if self.register_a >= val {
            self.set_carry();
        } else {
            self.clear_carry();
        }
        self.update_zero_and_negative_flags(self.register_a.wrapping_sub(val));
    }
    fn cpx(&mut self, mode: &AddressingMode) {
        let val = self.mem_read(self.get_operand_address(mode));
        if self.register_x >= val {
            self.set_carry();
        } else {
            self.clear_carry();
        }
        self.update_zero_and_negative_flags(self.register_x.wrapping_sub(val));
    }
    fn cpy(&mut self, mode: &AddressingMode) {
        let val = self.mem_read(self.get_operand_address(mode));
        if self.register_y >= val {
            self.set_carry();
        } else {
            self.clear_carry();
        }
        self.update_zero_and_negative_flags(self.register_y.wrapping_sub(val));
    }
    fn bit(&mut self, mode: &AddressingMode) {
        let val = self.mem_read(self.get_operand_address(mode));
        if self.register_a & val == 0 {
            self.status |= 0b0000_0010;
        } else {
            self.status &= 0b1111_1101;
        }
        self.status = (self.status & 0b0011_1111) | (val & 0b1100_0000);
    }
    fn asl(&mut self, mode: &AddressingMode) {
        if let AddressingMode::NoneAddressing = mode {
            if self.register_a & 0x80 != 0 {
                self.set_carry();
            } else {
                self.clear_carry();
            }
            self.register_a <<= 1;
            self.update_zero_and_negative_flags(self.register_a);
        } else {
            let addr = self.get_operand_address(mode);
            let mut val = self.mem_read(addr);
            if val & 0x80 != 0 {
                self.set_carry();
            } else {
                self.clear_carry();
            }
            val <<= 1;
            self.mem_write(addr, val);
            self.update_zero_and_negative_flags(val);
        }
    }
    fn lsr(&mut self, mode: &AddressingMode) {
        if let AddressingMode::NoneAddressing = mode {
            if self.register_a & 1 != 0 {
                self.set_carry();
            } else {
                self.clear_carry();
            }
            self.register_a >>= 1;
            self.update_zero_and_negative_flags(self.register_a);
        } else {
            let addr = self.get_operand_address(mode);
            let mut val = self.mem_read(addr);
            if val & 1 != 0 {
                self.set_carry();
            } else {
                self.clear_carry();
            }
            val >>= 1;
            self.mem_write(addr, val);
            self.update_zero_and_negative_flags(val);
        }
    }
    fn rol(&mut self, mode: &AddressingMode) {
        let c = self.status & 1;
        if let AddressingMode::NoneAddressing = mode {
            let old = (self.register_a & 0x80) >> 7;
            self.register_a = (self.register_a << 1) | c;
            if old != 0 {
                self.set_carry();
            } else {
                self.clear_carry();
            }
            self.update_zero_and_negative_flags(self.register_a);
        } else {
            let addr = self.get_operand_address(mode);
            let mut val = self.mem_read(addr);
            let old = (val & 0x80) >> 7;
            val = (val << 1) | c;
            self.mem_write(addr, val);
            if old != 0 {
                self.set_carry();
            } else {
                self.clear_carry();
            }
            self.update_zero_and_negative_flags(val);
        }
    }
    fn ror(&mut self, mode: &AddressingMode) {
        let c = self.status & 1;
        if let AddressingMode::NoneAddressing = mode {
            let old = self.register_a & 1;
            self.register_a = (self.register_a >> 1) | (c << 7);
            if old != 0 {
                self.set_carry();
            } else {
                self.clear_carry();
            }
            self.update_zero_and_negative_flags(self.register_a);
        } else {
            let addr = self.get_operand_address(mode);
            let mut val = self.mem_read(addr);
            let old = val & 1;
            val = (val >> 1) | (c << 7);
            self.mem_write(addr, val);
            if old != 0 {
                self.set_carry();
            } else {
                self.clear_carry();
            }
            self.update_zero_and_negative_flags(val);
        }
    }
    fn branch(&mut self, condition: bool) {
        let offset = self.mem_read(self.program_counter) as i8;
        self.program_counter = self.program_counter.wrapping_add(1);
        if condition {
            self.program_counter = self.program_counter.wrapping_add(offset as u16);
        }
    }
    fn rti(&mut self) {
        self.status = (self.stack_pop() & 0b1110_1111) | 0b0010_0000;
        self.program_counter = self.stack_pop_u16();
    }
    pub fn run(&mut self) {
        let ref opcode_table = OpCodeTable::new();

        loop {
            let code = self.mem_read(self.program_counter);
            self.program_counter += 1;
            let opcode = opcode_table
                .opcodes
                .get(&code)
                .unwrap_or_else(|| panic!("Opcode {:02X} is not recognized", code));

            match opcode.name {
                "BRK" => return,
                "NOP" => {}
                "LDA" => self.lda(&opcode.mode),
                "LDX" => self.ldx(&opcode.mode),
                "LDY" => self.ldy(&opcode.mode),
                "STA" => self.sta(&opcode.mode),
                "STX" => self.stx(&opcode.mode),
                "STY" => self.sty(&opcode.mode),
                "TAX" => self.tax(),
                "TXA" => self.txa(),
                "TAY" => self.tay(),
                "TYA" => self.tya(),
                "TSX" => self.tsx(),
                "TXS" => self.txs(),
                "CLC" => self.clc(),
                "SEC" => self.sec(),
                "CLI" => self.cli(),
                "SEI" => self.sei(),
                "CLV" => self.clv(),
                "CLD" => self.cld(),
                "SED" => self.sed(),
                "PHA" => self.pha(),
                "PLA" => self.pla(),
                "PHP" => self.php(),
                "PLP" => self.plp(),
                "ADC" => self.adc(&opcode.mode),
                "SBC" => self.sbc(&opcode.mode),
                "AND" => self.and(&opcode.mode),
                "ORA" => self.ora(&opcode.mode),
                "EOR" => self.eor(&opcode.mode),
                "INX" => self.inx(),
                "INY" => self.iny(),
                "DEX" => self.dex(),
                "DEY" => self.dey(),
                "INC" => self.inc(&opcode.mode),
                "DEC" => self.dec(&opcode.mode),
                "CMP" => self.cmp(&opcode.mode),
                "CPX" => self.cpx(&opcode.mode),
                "CPY" => self.cpy(&opcode.mode),
                "BIT" => self.bit(&opcode.mode),
                "ASL" => self.asl(&opcode.mode),
                "LSR" => self.lsr(&opcode.mode),
                "ROL" => self.rol(&opcode.mode),
                "ROR" => self.ror(&opcode.mode),
                "JMP" => {
                    if let AddressingMode::Absolute = opcode.mode {
                        self.program_counter = self.mem_read_u16(self.program_counter);
                    } else {
                        let mem_address = self.mem_read_u16(self.program_counter);
                        let indirect_ref = if mem_address & 0x00FF == 0x00FF {
                            let lo = self.mem_read(mem_address) as u16;
                            let hi = self.mem_read(mem_address & 0xFF00) as u16;
                            (hi << 8) | lo
                        } else {
                            self.mem_read_u16(mem_address)
                        };
                        self.program_counter = indirect_ref;
                    }
                }
                "JSR" => {
                    let target_address = self.mem_read_u16(self.program_counter);
                    self.stack_push_u16(self.program_counter + 2 - 1);
                    self.program_counter = target_address;
                }
                "RTS" => {
                    self.program_counter = self.stack_pop_u16() + 1;
                }
                "RTI" => self.rti(),

                "BCC" => self.branch((self.status & 1) == 0),
                "BCS" => self.branch((self.status & 1) != 0),
                "BEQ" => self.branch((self.status & 2) != 0),
                "BMI" => self.branch((self.status & 0x80) != 0),
                "BNE" => self.branch((self.status & 2) == 0),
                "BPL" => self.branch((self.status & 0x80) == 0),
                "BVC" => self.branch((self.status & 0x40) == 0),
                "BVS" => self.branch((self.status & 0x40) != 0),

                _ => panic!("Instruction {} is unhandled", opcode.name),
            }

            if opcode.name != "JMP"
                && opcode.name != "JSR"
                && opcode.name != "RTS"
                && opcode.name != "RTI"
                && opcode.name != "BCC"
                && opcode.name != "BCS"
                && opcode.name != "BEQ"
                && opcode.name != "BMI"
                && opcode.name != "BNE"
                && opcode.name != "BPL"
                && opcode.name != "BVC"
                && opcode.name != "BVS"
            {
                self.program_counter += (opcode.len - 1) as u16;
            }
        }
    }
}
fn main() {
    let mut cpu = CPU::new();
    let game_code: Vec<u8> = vec![0xA9, 0x10, 0xAA, 0x00];
    cpu.load_and_run(game_code);
    println!("no errors found!");
}
