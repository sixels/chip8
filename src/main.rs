use std::env;

#[allow(dead_code)]
mod cpu;
#[allow(dead_code)]
mod mmu;
#[allow(dead_code)]
mod frontend;

use mmu::MMU;
use cpu::CPU;

fn main() {
    let mut mmu: MMU = Default::default();

    // get the rom path from the first argument
    if let Some(rom_path) = env::args().nth(1) {
        println!("{}", rom_path);
        mmu.load_game(rom_path).unwrap();
    } else {
        panic!("You should pass the rom path as argument")
    }

    let mut cpu: CPU = CPU::new(&mut mmu);
    frontend::SDL::new(&mut cpu).run();
}
