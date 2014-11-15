
extern crate gol;

use std::rand;
use std::os;
use std::io::timer;
use std::io::stdio;
use std::time::Duration;
use gol::{ World, Live, Dead };

#[cfg(not(test))]
fn main() {

    let w = 50u;
    let h = 50u;
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

    loop {
        //Print world
        for row in w.iter_rows() {
            for cell in row.iter() {
                match *cell {
                    Live => stdio::print("O"),
                    Dead => stdio::print(" ")
                };
            }
            stdio::print("\n");
        }
        stdio::println("---------");
        stdio::flush();

        //Step world
        w.step_mut();

        //Sleep for a moment
        timer::sleep(Duration::milliseconds(200));
    }
}