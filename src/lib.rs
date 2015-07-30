//! Experimental Rust library for implementing Conway's Game of Life
//!
//! See also [gol-tcod] for example code consuming this library.
//!
//! [gol-tcod]: https://github.com/deadalusai/gol-tcod

extern crate rand;

pub mod plaintext;
pub mod rules;
pub mod world;
pub mod grid;