#![allow(unused_imports)]

extern crate gol;
extern crate rand;

use std::os;
use std::thread;
use std::process::{ exit };
use rand::{ random };
use gol::{ World, Cell };
use gol::Cell::{ Live, Dead };

#[cfg(not(test))]
fn main() {

    let (rows, cells) = (20, 50);
    let count = rows * cells;
    let state = (0..count).map(|_| if random::<bool>() { Live } else { Dead }).collect();

    let mut w = match World::try_create(rows, cells, state) {
        Ok(w) => w,
        Err(err) => { 
            println!("Error creating world: {:?}", err);
            exit(1);
        }
    };

    // A single frame of output
    let mut frame = String::with_capacity((w.cells() + 1) * w.rows());

    loop {
        //Print world
        for row in w.iter_rows() {
            for cell in row.iter() {
                match *cell {
                    Live => frame.push('@'),
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
        thread::sleep_ms(30);
    }
}
