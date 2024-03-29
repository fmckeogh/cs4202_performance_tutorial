use std::time::Instant;

const WIDTH: usize = 8192;
const HEIGHT: usize = 8192;

/// example function that modifies a pixel
fn modify_pixel((r, g, b): &mut (u8, u8, u8)) {
    *r = r.wrapping_pow(3) / 4;
    *g = g.wrapping_pow(3) / 4;
    *b = b.wrapping_pow(3) / 4;
}

fn main() {
    let mut image = vec![vec![(3u8, 4u8, 5u8); WIDTH]; HEIGHT];

    // loop column-first
    let start = Instant::now();
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            modify_pixel(&mut image[y][x]);
        }
    }
    println!("column-first:\t\t{} us", start.elapsed().as_micros());

    // loop row-first
    let start = Instant::now();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            modify_pixel(&mut image[y][x]);
        }
    }
    println!("row-first:\t\t{} us", start.elapsed().as_micros());

    // ...now let's make it a linear vector
    let mut image = vec![(3u8, 4u8, 5u8); WIDTH * HEIGHT];

    // loop over it
    let start = Instant::now();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            modify_pixel(&mut image[y * HEIGHT + x]);
        }
    }
    println!("linear:\t\t\t{} us", start.elapsed().as_micros());
}
