#![allow(unused_imports)]

extern crate gol;
extern crate rand;
extern crate time;

use std::os;
use std::thread;
use std::process::{ exit };

use rand::{ Rng, thread_rng };

use time::{ Duration };

use gol::{ Grid, World, Cell };
use gol::Cell::{ Live, Dead };

#[cfg(not(test))]
fn main() {

    let (width, height) = (50, 20);

    let mut rng = thread_rng();
    let grid = Grid::create_random(&mut rng, width, height);

    let mut w = World::new(grid);

    // A single frame of output
    let mut frame = String::with_capacity((width + 1) * height);

    loop {
        //Print world
        for row in w.iter_rows() {
            for cell in row.iter() {
                match *cell {
                    Live => frame.push('O'),
                    Dead => frame.push(' ')
                };
            }
            frame.push('\n');
        }

        println!("{}Generation: {}", &frame, &w.generation());

        frame.clear();

        //Step world
        w.step_mut();

        //Sleep for a moment
        thread::sleep_ms(50);
    }
}