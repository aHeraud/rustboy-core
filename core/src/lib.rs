#![feature(nll)]

extern crate time;
extern crate flate2;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate bincode;

pub mod gameboy;

pub use gameboy::Gameboy;
pub use gameboy::joypad::Key;

pub const WIDTH: usize = 160;
pub const HEIGHT: usize = 144;
pub const FPS: f64 = 59.7_f64;
