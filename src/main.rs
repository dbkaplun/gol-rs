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

    bench_find_neighbours();

    return;

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

fn bench_find_neighbours() {
    use rand::{ StdRng, SeedableRng };

    fn make_rng() -> StdRng {
        let seed: &[_] = &[1, 2, 3, 4];
        SeedableRng::from_seed(seed)
    }

    fn find_neighbours_1(w: &World) {
        
        w.find_neighbours(0, 0);
        w.find_neighbours(700, 300);
        w.find_neighbours(500, 500);
        w.find_neighbours(300, 700);
        w.find_neighbours(999, 999);
    }

    fn find_neighbours_2(w: &World) {
 
        w.find_neighbours_2(0, 0);
        w.find_neighbours_2(700, 300);
        w.find_neighbours_2(500, 500);
        w.find_neighbours_2(300, 700);
        w.find_neighbours_2(999, 999);
    }

    let mut rng = make_rng();
    let w = World::new(Grid::create_random(&mut rng, 1000, 1000));

    let iterations = 100_000;

    println!("find_neighbours:   {}ns", bench(iterations, || find_neighbours_1(&w)).num_nanoseconds().unwrap());
    println!("find_neighbours_2: {}ns", bench(iterations, || find_neighbours_2(&w)).num_nanoseconds().unwrap());
}

fn bench<F>(iterations: u32, f: F) -> Duration
    where F: Fn() -> ()
{
    let _sum = (0..iterations)
        .map(|_| Duration::span(|| f()).num_nanoseconds().unwrap())
        .fold(0, |s, e| s + e);

    let _avg: f64 = _sum as f64 / iterations as f64;

    Duration::nanoseconds(_avg as i64)
}