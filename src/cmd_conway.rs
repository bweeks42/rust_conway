use std::{thread, time};
use graphics::Viewport;

// Print
fn print_matrix(matrix: &ConwayMatrix) {
    print!("\x1B[1;1H");
    for row in matrix {
        for cell in row {
            print!("{} ", cell.0);
        }
        print!("\n");
    }
}

fn print_matrix_neighbors(matrix: &ConwayMatrix) {
    print!("\x1B[1;1H");
    for row in matrix {
        for cell in row {
            print!("{} ", cell.2);
        }
        print!("\n");
    }
}


// Draw


// Main
fn main() {
    let mut matrix = matrix_of_size(30);
    put_glider_in_matrix(&mut matrix);

            // 60fps draw
    let sleep_time = time::Duration::from_millis(16);

    println!("\x1B[2J"); // clear screen
    loop {

        print_matrix(&matrix);
        thread::sleep(sleep_time);
        //print_matrix_neighbors(&matrix);
        update_matrix(&mut matrix);
        //thread::sleep(sleep_time);
    }
}
