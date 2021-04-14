use std::path::Path;
use std::{fs, io};

const ROM_SIZE: usize = 0x200;
const UPPER_ROM_SIZE: usize = 0x400;
const RAM_SIZE: usize = 0xA00;
const VRAM_SIZE: usize = 64 * 32;

const CH8_FONT: [u8; 0x50] = [
    0x60, 0x90, 0x90, 0x90, 0x60, // 0
    0x20, 0x60, 0x20, 0x20, 0xF0, // 1
    0xE0, 0x10, 0x70, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0x60, 0x90, 0x60, 0x90, 0x60, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0x60, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct MMU {
    pub rom: [u8; ROM_SIZE],
    pub upper_rom: [u8; UPPER_ROM_SIZE],
    ram: [u8; RAM_SIZE],

    pub vram: Box<[u8; VRAM_SIZE]>,

    locked_rom: bool,
}

impl Default for MMU {
    fn default() -> Self {
        let mut rom = [0u8; ROM_SIZE];
        for (i, &byte) in CH8_FONT.iter().enumerate() {
            rom[i] = byte
        }

        Self {
            rom,
            upper_rom: [0; UPPER_ROM_SIZE],
            ram: [0; RAM_SIZE],

            vram: Box::new([0; VRAM_SIZE]),

            locked_rom: false,
        }
    }
}

#[allow(dead_code)]
impl MMU {
    pub fn rb(&self, offset: usize) -> u8 {
        match offset {
            0x000..=0x1FF => self.rom[offset],
            0x200..=0x5FF => self.upper_rom[offset - 0x200],
            0x600..=0xFFF => self.ram[offset - 0x600],
            _ => panic!("Attempt to read an invalid offset: 0x{:04x}", offset),
        }
    }
    pub fn wb(&mut self, offset: usize, byte: u8) {
        match offset {
            0x200..=0x5FF if !self.locked_rom => self.upper_rom[offset - 0x200] = byte,
            0x600..=0xFFF => self.ram[offset - 0x600] = byte,
            0x200..=0x5FF => self.upper_rom[offset - 0x200] = byte,
            0x000..=0x1FF => self.rom[offset] = byte,
            _ => panic!("Invalid offset"),
        }
    }

    pub fn rw(&self, offset: usize) -> u16 {
        u16::from(self.rb(offset)) << 8 | u16::from(self.rb(offset + 1))
    }
    pub fn ww(&mut self, offset: usize, word: u16) {
        let high = word >> 8;
        let low = word & 0xFF;

        self.wb(offset, high as u8);
        self.wb(offset + 1, low as u8);
    }

    pub fn wb_vram(&mut self, x: usize, y: usize, byte: u8) -> bool {
        let offset = x + y * 64;

        let old_value = self.vram[offset];
        self.vram[offset] ^= byte;

        byte == old_value
    }
    pub fn rb_vram(&self, x: usize, y: usize) -> u8 {
        let offset = x + y * 64;

        self.vram[offset]
    }

    pub fn load_game<P: AsRef<Path>>(&mut self, game_path: P) -> io::Result<()> {
        let game_content = fs::read(game_path)?;

        for (i, &byte) in game_content.iter().enumerate() {
            self.upper_rom[i] = byte
        }

        // lock the rom when the game is loaded
        self.lock_rom();
        Ok(())
    }

    pub fn lock_rom(&mut self) {
        self.locked_rom = true;
    }
    pub fn locked_rom(&self) -> bool {
        self.locked_rom
    }
}
