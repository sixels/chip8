pub mod cpu;
pub mod mmu;

#[cfg(feature = "sdl")]
mod frontend;

#[cfg(feature = "sdl")]
fn main() {
    use std::env;

    use cpu::CPU;
    use mmu::MMU;

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

#[cfg(feature = "default")]
fn main(){}