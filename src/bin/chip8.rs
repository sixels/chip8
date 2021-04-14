extern crate chip8;

use chip8::cpu::CPU;
use chip8::mmu::MMU;
use std::env;

fn main() {
    let mut mmu: MMU = Default::default();

    // get the rom path from the first argument
    if let Some(rom_path) = env::args().nth(1) {
        println!("{}", rom_path);
        mmu.load_game(rom_path).unwrap();
    } else {
        panic!("You should pass the rom path as argument")
    }

    let mut cpu: CPU = CPU::new(mmu);

    #[cfg(feature = "sdl")]
    chip8::frontend::SDL::new(&mut cpu).run();

    #[cfg(not(feature = "sdl"))]
    loop {
        cpu.cycle()
    }
}
