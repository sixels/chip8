use super::Status;

#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    CLS,
    RET,
    SYS,
    JP,
    CALL,
    SE,
    LD,
    ADD,
    OR,
    AND,
    XOR,
    SUB,
    SHR,
    SUBN,
    SHL,
    SNE,
    RND,
    DRW,
    SKP,
    SKNP,
}

#[derive(Debug, Clone, Copy)]
pub enum AddressingMode {
    Implicit,
    Addr(u16),

    Vx(usize),
    VxImediate(usize, u8),
    VxDT(usize),
    VxKey(usize),
    VxMem(usize),

    VxVy(usize, usize),
    VxVyImediate(usize, usize, u8),

    V0Addr(u16),

    IAddr(u16),
    IVx(usize),
    FVx(usize),
    BVx(usize),
    MemVx(usize),

    DTVx(usize),
    STVx(usize),
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction(pub Opcode, pub AddressingMode);

impl From<u16> for Instruction {
    fn from(opcode: u16) -> Self {
        let opcode_nibbles = (
            ((opcode >> 0xC) & 0xF) as u8,
            ((opcode >> 0x8) & 0xF) as usize,
            ((opcode >> 0x4) & 0xF) as usize,
            (opcode & 0xF) as u8,
        );

        match opcode_nibbles {
            (0x0, 0x0, 0xE, 0x0) => Instruction(Opcode::CLS, AddressingMode::Implicit),

            (0x0, 0x0, 0xE, 0xE) => Instruction(Opcode::RET, AddressingMode::Implicit),
            (0x0, _, _, _) => Instruction(Opcode::SYS, AddressingMode::Addr(opcode & 0xFFF)),
            (0x1, _, _, _) => Instruction(Opcode::JP, AddressingMode::Addr(opcode & 0xFFF)),
            (0x2, _, _, _) => Instruction(Opcode::CALL, AddressingMode::Addr(opcode & 0xFFF)),

            (0x3, x, _, _) => Instruction(
                Opcode::SE,
                AddressingMode::VxImediate(x, (opcode & 0xFF) as u8),
            ),
            (0x4, x, _, _) => Instruction(
                Opcode::SNE,
                AddressingMode::VxImediate(x, (opcode & 0xFF) as u8),
            ),
            (0x5, x, y, 0) => Instruction(Opcode::SE, AddressingMode::VxVy(x, y)),
            (0x6, x, _, _) => Instruction(
                Opcode::LD,
                AddressingMode::VxImediate(x, (opcode & 0xFF) as u8),
            ),
            (0x7, x, _, _) => Instruction(
                Opcode::ADD,
                AddressingMode::VxImediate(x, (opcode & 0xFF) as u8),
            ),

            (0x8, x, y, 0x0) => Instruction(Opcode::LD, AddressingMode::VxVy(x, y)),
            (0x8, x, y, 0x1) => Instruction(Opcode::OR, AddressingMode::VxVy(x, y)),
            (0x8, x, y, 0x2) => Instruction(Opcode::AND, AddressingMode::VxVy(x, y)),
            (0x8, x, y, 0x3) => Instruction(Opcode::XOR, AddressingMode::VxVy(x, y)),
            (0x8, x, y, 0x4) => Instruction(Opcode::ADD, AddressingMode::VxVy(x, y)),
            (0x8, x, y, 0x5) => Instruction(Opcode::SUB, AddressingMode::VxVy(x, y)),
            (0x8, x, y, 0x6) => Instruction(Opcode::SHR, AddressingMode::VxVy(x, y)),
            (0x8, x, y, 0x7) => Instruction(Opcode::SUBN, AddressingMode::VxVy(x, y)),
            (0x8, x, y, 0xE) => Instruction(Opcode::SHL, AddressingMode::VxVy(x, y)),

            (0x9, x, y, 0) => Instruction(Opcode::SNE, AddressingMode::VxVy(x, y)),
            (0xA, _, _, _) => Instruction(Opcode::LD, AddressingMode::IAddr(opcode & 0xFFF)),
            (0xB, _, _, _) => Instruction(Opcode::JP, AddressingMode::V0Addr(opcode & 0xFFF)),
            (0xC, x, _, _) => Instruction(
                Opcode::RND,
                AddressingMode::VxImediate(x, (opcode & 0xFF) as u8),
            ),
            (0xD, x, y, _) => Instruction(
                Opcode::DRW,
                AddressingMode::VxVyImediate(x, y, (opcode & 0xF) as u8),
            ),

            (0xE, x, 0x9, 0xE) => Instruction(Opcode::SKP, AddressingMode::Vx(x)),
            (0xE, x, 0xA, 0x1) => Instruction(Opcode::SKNP, AddressingMode::Vx(x)),

            (0xF, x, 0x0, 0x7) => Instruction(Opcode::LD, AddressingMode::VxDT(x)),
            (0xF, x, 0x0, 0xA) => Instruction(Opcode::LD, AddressingMode::VxKey(x)),
            (0xF, x, 0x1, 0x5) => Instruction(Opcode::LD, AddressingMode::DTVx(x)),
            (0xF, x, 0x1, 0x8) => Instruction(Opcode::LD, AddressingMode::STVx(x)),
            (0xF, x, 0x1, 0xE) => Instruction(Opcode::ADD, AddressingMode::IVx(x)),
            (0xF, x, 0x2, 0x9) => Instruction(Opcode::LD, AddressingMode::FVx(x)),
            (0xF, x, 0x3, 0x3) => Instruction(Opcode::LD, AddressingMode::BVx(x)),
            (0xF, x, 0x5, 0x5) => Instruction(Opcode::LD, AddressingMode::MemVx(x)),
            (0xF, x, 0x6, 0x5) => Instruction(Opcode::LD, AddressingMode::VxMem(x)),

            _ => unreachable!(),
        }
    }
}

// TODO: debug support for instructions
// impl Display for Instruction {  }

impl super::CPU {
    /// CLS: clear the screen
    pub fn exec_cls(&mut self) {
        for b in self.bus.borrow_mut().vram.iter_mut() {
            *b = 0;
        }
    }

    /// JP: jump to the given address
    pub fn exec_jp(&mut self, addressing_mode: AddressingMode) {
        match addressing_mode {
            AddressingMode::Addr(addr) => {
                self.pc = addr;
            }
            AddressingMode::V0Addr(addr) => {
                self.pc = u16::from(self.v[0]) + addr;
            }
            _ => unreachable!(),
        }
    }

    /// LD: Load the content of a register/memory location into another
    pub fn exec_ld(&mut self, addressing_mode: AddressingMode) {
        match addressing_mode {
            AddressingMode::VxImediate(x, kk) => {
                self.v[x] = kk;
            }
            AddressingMode::VxVy(x, y) => {
                self.v[x] = self.v[y];
            }
            AddressingMode::IAddr(addr) => {
                self.i = addr;
            }
            AddressingMode::VxDT(x) => {
                self.v[x] = self.delay;
            }
            AddressingMode::VxKey(x) => self.status = Status::WaitingKeypress(x),
            AddressingMode::DTVx(x) => {
                self.delay = self.v[x];
            }
            AddressingMode::STVx(x) => {
                self.sound = self.v[x];
            }
            AddressingMode::FVx(x) => {
                self.i = u16::from(self.v[x]) * 5;
            }
            AddressingMode::BVx(x) => {
                let i = usize::from(self.i);
                let value = self.v[x];

                self.bus.borrow_mut().wb(i, value / 100);
                self.bus.borrow_mut().wb(i + 1, (value % 100) / 10);
                self.bus.borrow_mut().wb(i + 2, value % 10);
            }
            AddressingMode::MemVx(x) => {
                for rx in 0..=x {
                    let offset = usize::from(self.i) + rx;
                    self.bus.borrow_mut().wb(offset, self.v[rx]);
                }
            }
            AddressingMode::VxMem(x) => {
                for rx in 0..=x {
                    let offset = usize::from(self.i) + rx;
                    self.v[rx] = self.bus.borrow_mut().rb(offset);
                }
            }
            _ => unreachable!(),
        }
    }

    /// DRW: update the VRAM (i.e. draw the screen)
    pub fn exec_drw(&mut self, addressing_mode: AddressingMode) {
        if let AddressingMode::VxVyImediate(x, y, n) = addressing_mode {
            let (x, y) = (usize::from(self.v[x]), usize::from(self.v[y]));

            self.v[0xF] = 0;

            for row in 0..usize::from(n) {
                let sprite_data = self.bus.borrow_mut().rb(usize::from(self.i) + row);

                for bit in 0..8 {
                    if (sprite_data & (0x80 >> bit)) > 0 {
                        if self
                            .bus
                            .borrow_mut()
                            .wb_vram((x + bit) % 64, (y + row) % 32, 1)
                        {
                            self.v[0xF] = 1;
                        }
                    }
                }
            }
        } else {unreachable!()}
    }

    /// ADD: add the content of a register with a value
    pub fn exec_add(&mut self, addressing_mode: AddressingMode) {
        match addressing_mode {
            AddressingMode::VxImediate(x, kk) => {
                self.v[x] = self.v[x].wrapping_add(kk);
            }
            AddressingMode::VxVy(x, y) => {
                let (result, did_overflow) = self.v[x].overflowing_add(self.v[y]);

                self.v[x] = result;
                self.v[0xf] = u8::from(did_overflow);
            }
            AddressingMode::IVx(x) => {
                self.i = self.i.wrapping_add(u16::from(self.v[x]));
            }
            _ => unreachable!(),
        }
    }
    /// SUB: subract the content of a register with a value
    pub fn exec_sub(&mut self, addressing_mode: AddressingMode) {
        if let AddressingMode::VxVy(x, y) = addressing_mode {
            let (result, did_overflow) = self.v[x].overflowing_sub(self.v[y]);

            self.v[x] = result;
            self.v[0xF] = u8::from(did_overflow);
        } else {
            unreachable!()
        }
    }
    /// OR: bitwise-or the content of a register with a value
    pub fn exec_or(&mut self, addressing_mode: AddressingMode) {
        if let AddressingMode::VxVy(x, y) = addressing_mode {
            self.v[x] = self.v[x] | self.v[y];
        } else {
            unreachable!()
        }
    }
    /// AND: bitwise-and the content of a register with a value
    pub fn exec_and(&mut self, addressing_mode: AddressingMode) {
        if let AddressingMode::VxVy(x, y) = addressing_mode {
            self.v[x] = self.v[x] & self.v[y];
        } else {
            unreachable!()
        }
    }
    /// XOR: bitwise-xor the content of a register with a value
    pub fn exec_xor(&mut self, addressing_mode: AddressingMode) {
        if let AddressingMode::VxVy(x, y) = addressing_mode {
            self.v[x] = self.v[x] ^ self.v[y];
        } else {
            unreachable!()
        }
    }
    /// SHL: shift left the bits of a register
    pub fn exec_shl(&mut self, addressing_mode: AddressingMode) {
        if let AddressingMode::VxVy(x, _) = addressing_mode {
            self.v[0xf] = self.v[x] & 0x80;
            self.v[x] <<= 1;
        } else {
            unreachable!()
        }
    }
    /// SHR: shift right the bits of a register
    pub fn exec_shr(&mut self, addressing_mode: AddressingMode) {
        if let AddressingMode::VxVy(x, _) = addressing_mode {
            self.v[0xf] = self.v[x] & 0x01;
            self.v[x] >>= 1;
        } else {
            unreachable!()
        }
    }

    /// SE: Skip the next instruction if `vx` is equal value
    pub fn exec_se(&mut self, addressing_mode: AddressingMode) {
        let (x, byte) = match addressing_mode {
            AddressingMode::VxImediate(x, kk) => (x, kk),
            AddressingMode::VxVy(x, y) => (x, self.v[y]),
            _ => unreachable!(),
        };

        if self.v[x] == byte {
            self.pc += 2;
        }
    }
    /// SNE: Skip the next instruction if `vx` is not equal value
    pub fn exec_sne(&mut self, addressing_mode: AddressingMode) {
        let (x, byte) = match addressing_mode {
            AddressingMode::VxImediate(x, kk) => (x, kk),
            AddressingMode::VxVy(x, y) => (x, self.v[y]),
            _ => unreachable!(),
        };

        if self.v[x] != byte {
            self.pc += 2;
        }
    }

    /// CALL: Call the subroutine at `addr`
    pub fn exec_call(&mut self, addressing_mode: AddressingMode) {
        if let AddressingMode::Addr(addr) = addressing_mode {
            self.push_stack(self.pc);
            self.pc = addr;
        } else {
            unreachable!()
        }
    }
    /// RET: Return from a subroutine
    pub fn exec_ret(&mut self, addressing_mode: AddressingMode) {
        if let AddressingMode::Implicit = addressing_mode {
            self.pc = self.pop_stack();
        } else {
            unreachable!()
        }
    }

    /// RND: Set `vx` to a random number masked with NN
    pub fn exec_rnd(&mut self, addressing_mode: AddressingMode) {
        if let AddressingMode::VxImediate(x, nn) = addressing_mode {
            self.v[x] = rand::random::<u8>() & nn;
        } else {
            unreachable!()
        }
    }

    // SKNP: Skip next instruction if key with the value of Vx is not pressed.
    pub fn exec_sknp(&mut self, addressing_mode: AddressingMode) {
        if let AddressingMode::Vx(x) = addressing_mode {
            if !((self.keypad & 1 << u16::from(self.v[x])) == 1 << u16::from(self.v[x])) {
                self.pc += 2;
            }
        } else {
            unreachable!()
        }
    }

    // SKP Skip next instruction if key with the value of Vx is pressed.
    pub fn exec_skp(&mut self, addressing_mode: AddressingMode) {
        if let AddressingMode::Vx(x) = addressing_mode {
            if (self.keypad & 1 << u16::from(self.v[x])) == 1 << u16::from(self.v[x]) {
                self.pc += 2;
            }
        } else {unreachable!()}
    }
}
