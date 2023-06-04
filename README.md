# PogBoy
<p align="center">
<img width="650px" src="https://imgur.com/YHdXH07.png"/>
</p>

A work in progress Gameboy emulator i made to learn Rust, using [Piston](https://www.piston.rs/) for the rendering.
The emulator is still in early development, most of the basic stuff is yet to be implemented, but it can boot the original BIOS, Tetris and Dr Mario 🎉

## Starting and debugging ROM
Roms should be placed inside `/src/roms`, the emulator must be compiled in release mode to create an optimized build (for performance issues) and ran with `cargo run --release -- rom-name.gb`.

Some debugging commands can be launched during execution: <kbd>l</kbd> will enable CPU execution logging, <kbd>t</kbd> will toggle the currently used tileset, <kbd>d</kbd> will dump interrupts/cpu/lcdc states to screen, tilemaps and screentiles to file and current tileset to image

## Test
All tests are organized in the `tests` directory and can be run with `cargo test`
