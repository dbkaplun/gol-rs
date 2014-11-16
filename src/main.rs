#![allow(unused_imports)]

extern crate gol;

use std::rand;
use std::os;
use std::io::timer;
use std::time::Duration;
use gol::{ World, Live, Dead };

#[cfg(not(test))]
fn main() {

    let w = 50u;
    let h = 30u;
    let state = Vec::from_fn(w * h, |_| {
        match rand::random::<bool>() { true => Live, false => Dead }
    });

    let mut w = match World::try_create(w, h, state) {
        Ok(w) => w,
        Err(err) => { 
            println!("Error creating world: {}", err);
            os::set_exit_status(1);
            return;
        }
    };

    // A single frame of output
    let mut frame = String::with_capacity((w.width() + 1) * w.height());

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
        timer::sleep(Duration::milliseconds(200));
    }
}
