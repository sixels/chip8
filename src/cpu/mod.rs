use std::cell::RefCell;

use crate::mmu::MMU;

#[allow(dead_code)]
mod instruction;

use instruction::{Instruction, Opcode};

#[derive(Copy, Clone)]
pub enum Status {
    Running,
    Halt,
}

pub struct CPU {
    // general purpose registers (usually called Vx)
    v: [u8; 16],

    // Program Counter and Stack Pointer
    pc: u16,
    sp: u16,

    // generally used to store memory addresses
    i: u16,

    // Delay and Sound Timers
    delay: u8,
    sound: u8,

    stack: [u16; 16],

    // Memory Bus
    pub(crate) bus: RefCell<MMU>,

    // cpu status
    status: Status,
}

impl CPU {
    // create a new cpu
    pub fn new(bus: MMU) -> Self {
        // rom should already be locked
        assert!(bus.locked_rom());

        Self {
            v: [0; 16],

            pc: 0x200,
            sp: 0,

            i: 0,

            delay: 0,
            sound: 0,

            stack: [0; 16],

            bus: RefCell::new(bus),
            status: Status::Running,
        }
    }

    pub fn status(&self) -> Status {
        self.status
    }

    // perform a cpu cycle
    pub fn cycle(&mut self) {
        let instruction = self.fetch();
        self.execute(instruction);
        // TODO: decrement the timers
    }

    // fetch and decode an opcode, returning the respective instruction
    pub fn fetch(&mut self) -> Instruction {
        let opcode = self.bus.borrow_mut().rw(self.pc as usize);
        println!("{:04X}", opcode);
        let instruction: Instruction = opcode.into();

        self.pc += 2;

        instruction
    }

    // push a word into stack
    fn push_stack(&mut self, word: u16) {
        self.stack[usize::from(self.sp)] = word;
        self.sp += 1;
    }
    // pop a word from stack
    fn pop_stack(&mut self) -> u16 {
        self.sp -= 1;
        let sp = usize::from(self.sp);

        let word = self.stack[sp];
        self.stack[sp] = 0;

        word
    }

    // execute a given instruction
    pub fn execute(&mut self, instruction: Instruction) {
        let Instruction(opcode, addressing_mode) = instruction;
        println!("Running {:06x?}", instruction);

        match opcode {
            Opcode::CLS => self.exec_cls(),

            Opcode::JP => self.exec_jp(addressing_mode),
            Opcode::LD => self.exec_ld(addressing_mode),

            Opcode::DRW => self.exec_drw(addressing_mode),

            Opcode::ADD => self.exec_add(addressing_mode),
            Opcode::SUB => self.exec_sub(addressing_mode),
            Opcode::OR => self.exec_or(addressing_mode),
            Opcode::AND => self.exec_and(addressing_mode),
            Opcode::XOR => self.exec_xor(addressing_mode),
            Opcode::SHL => self.exec_shl(addressing_mode),
            Opcode::SHR => self.exec_shr(addressing_mode),

            Opcode::SE => self.exec_se(addressing_mode),
            Opcode::SNE => self.exec_sne(addressing_mode),

            Opcode::CALL => self.exec_call(addressing_mode),
            Opcode::RET => self.exec_ret(addressing_mode),

            Opcode::RND => self.exec_rnd(addressing_mode),

            _ => panic!("Instruction not implemented: {:x?}", instruction),
        }
    }
}
