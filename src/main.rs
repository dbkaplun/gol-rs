#![allow(unused_imports)]

extern crate gol;

use std::rand;
use std::os;
use std::io::timer;
use std::time::Duration;
use gol::{ World, Cell };
use gol::Cell::Live as O;
use gol::Cell::Dead as X;

#[cfg(not(test))]
#[allow(unstable)]
fn main() {

    let (rows, cells) = (20, 50);
    let count = rows * cells;
    let state = (0..count).map(|_| match rand::random::<bool>() { true => O, false => X }).collect();

    let mut w = match World::try_create(rows, cells, state) {
        Ok(w) => w,
        Err(err) => { 
            println!("Error creating world: {:?}", err);
            os::set_exit_status(1);
            return;
        }
    };

    // A single frame of output
    let mut frame = String::with_capacity((w.cells() + 1) * w.rows());

    loop {
        //Print world
        for row in w.iter_rows() {
            for cell in row.iter() {
                match *cell {
                    O => frame.push('@'),
                    X => frame.push(' ')
                };
            }
            frame.push('\n');
        }

        println!("{}Generation: {}", &frame, &w.generation());

        frame.clear();

        //Step world
        w.step_mut();

        //Sleep for a moment
        timer::sleep(Duration::milliseconds(30));
    }
}
