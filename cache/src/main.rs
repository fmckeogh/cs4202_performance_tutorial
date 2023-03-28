use std::time::Instant;

const WIDTH: usize = 8192;
const HEIGHT: usize = 8192;

/// example function that modifies a pixel
fn modify_pixel((r, g, b): &mut (u8, u8, u8)) {
    *r = r.pow(3) / 4;
    *g = g.pow(3) / 4;
    *b = b.pow(3) / 4;
}

fn main() {
    let mut image = vec![vec![(3u8, 4u8, 5u8); WIDTH]; HEIGHT];

    // loop row-first
    let start = Instant::now();
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            modify_pixel(&mut image[y][x]);
        }
    }
    println!("row-first:\t\t{} us", start.elapsed().as_micros());

    // // loop column-first
    // let start = Instant::now();
    // for y in 0..HEIGHT {
    //     for x in 0..WIDTH {
    //         modify_pixel(&mut image[y][x]);
    //     }
    // }
    // println!("col-first:\t\t{} us", start.elapsed().as_micros());

    // // ...now let's make it a flat vector
    // let mut image = vec![(3u8, 4u8, 5u8); WIDTH * HEIGHT];

    // // loop flat
    // let start = Instant::now();
    // for y in 0..HEIGHT {
    //     for x in 0..WIDTH {
    //         modify_pixel(&mut image[y * HEIGHT + x]);
    //     }
    // }
    // println!("flat:\t\t\t{} us", start.elapsed().as_micros());
}
