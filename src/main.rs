#![allow(unused_imports)]

extern crate gol;
extern crate rand;

use std::os;
use std::thread;
use std::process::{ exit };
use rand::{ Rng, thread_rng };
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


    /*
    #[bench]
    fn find_neighbours_1(b: &mut Bencher) {
        
        let mut rng = make_rng();
        let w = World::new(Grid::create_random(&mut rng, 1000, 1000));

        b.iter(|| { 
            w.find_neighbours(500, 500);
        });
    }

    #[bench]
    fn find_neighbours_2(b: &mut Bencher) {
        
        let mut rng = make_rng();
        let w = World::new(Grid::create_random(&mut rng, 1000, 1000));

        b.iter(|| { 
            w.find_neighbours2(500, 500); 
        });
    }
    */