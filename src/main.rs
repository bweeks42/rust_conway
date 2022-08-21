mod conway_core;

use std::{thread, time};
use conway_core::{ConwayMatrix, put_glider_in_matrix};
use rand::Rng;

// Print
fn print_matrix(matrix: &ConwayMatrix) {
    print!("\x1B[1;1H");
    print!("{}", matrix.to_string());
}

fn print_matrix_neighbors(matrix: &ConwayMatrix) {
    // print!("\x1B[1;1H");
    // print!("{}", matrix.to_string());
}


// Main
fn main() {
    let matrix = &mut ConwayMatrix::new(70);
    put_glider_in_matrix(matrix);

    // 60fps draw
    let sleep_time = time::Duration::from_millis(20);
    let mut glider_countdown = rand::thread_rng().gen_range(1..10);

    println!("\x1B[2J"); // clear screen

    loop {
        print_matrix(&matrix);
        matrix.tick();
        thread::sleep(sleep_time);
        glider_countdown -= 1;
        if glider_countdown == 0 {
            put_glider_in_matrix(matrix);
            glider_countdown = rand::thread_rng().gen_range(1..10);
        }
    }
}
