use std::thread::spawn;

fn main() {
    let handle = spawn(|| {
        generate_large_matrix(20_000, 20_000);
    });

    // generate_large_matrix(20_000, 20_000);
    do_some_other_work();

    handle.join().expect("The thread being joined has panicked");
}

fn do_some_other_work() {
    println!("Doing some other work...");
}

fn generate_large_matrix(rows: usize, cols: usize) -> Vec<Vec<usize>> {
    println!("Generating a large matrix...");

    let mut matrix: Vec<Vec<usize>> = Vec::with_capacity(rows * cols);
    let mut count = 1;

    for _ in 0..rows {
        let mut row = Vec::with_capacity(cols);
        for _ in 0..cols {
            row.push(count);
            count += 1;
        }
        matrix.push(row);
    }

    matrix
}
