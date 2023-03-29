use {
    rand::{thread_rng, Rng},
    std::time::Instant,
};

fn main() {
    let mut data = vec![0u32; 10_000_000];
    thread_rng().fill(data.as_mut_slice());

    let mut hi = 0;
    let mut lo = 0;

    let start = Instant::now();
    for element in &data {
        if *element > (u32::MAX / 2) {
            hi += 1;
        } else {
            lo += 1;
        }
    }
    println!("random: {} us, {} {}", start.elapsed().as_micros(), hi, lo);

    // sort the data
    data.sort();
    hi = 0;
    lo = 0;

    let start = Instant::now();
    for element in &data {
        if *element > (u32::MAX / 2) {
            hi += 1;
        } else {
            lo += 1;
        }
    }
    println!("sorted: {} us, {} {}", start.elapsed().as_micros(), hi, lo);
}
