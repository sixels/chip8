# chip8

CHIP-8 interpreter with native (SDL2) and web (webassembly) front-ends.

## Testing the emulator

This library provides a basic CHIP-8 implementation, meant to be used as the core for a front-end. Although, this repository ships a demo using a SDL2 front-end.
[Check out the web front-end also](https://github.com/protoshark/chip8-wasm)

### Dependencies

- [SDL2](https://www.libsdl.org/download-2.0.php) (required for the SDL2 front-end)

### Running

One can test it by simply running the following command in a terminal (assuming that `cargo` and `SDL2` are installed)

```
cargo run --release --features sdl -- path/to/chip8/rom
```
