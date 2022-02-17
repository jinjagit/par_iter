use rand::Rng;
use std::time::Instant;
use rayon::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let mut  vec: Vec<f64> = vec![];

    for _i in 0..99999999 {
        vec.push(rng.gen::<f64>());
    }

    let now = Instant::now();
    let a: Vec<f64> = vec.iter().map(|x| x.sin().sqrt().atan()).collect();
    let new_now = Instant::now();
    println!("{:?}", new_now.duration_since(now));

    let now = Instant::now();
    let a: Vec<f64> = vec.par_iter().map(|x| x.sin().sqrt().atan()).collect();
    let new_now = Instant::now();
    println!("{:?}", new_now.duration_since(now));

    //println!("array: {:?}", a);
}
