use std::time::Instant;

const WIDTH: usize = 8192;
const HEIGHT: usize = 8192;

#[derive(Clone)]
struct Pixel {
    red: u16,
    green: u32,
    blue: u32,
    // red: u16,
    // _0: u16,
    // _1: u32,
    // _2: u64,
    // green: u16,
    // _3: u16,
    // _4: u32,
    // _5: u64,
    // blue: u16,
    // _6: u16,
    // _7: u32,
    // _8: u64,
}

impl Pixel {
    /// example function that modifies a pixel
    pub fn modify(&mut self) {
        self.red = ((self.red * self.green * self.blue) / 3).pow(2);
        self.green = ((self.red * self.green * self.blue) / 3).pow(2);
        self.blue = ((self.red * self.green * self.blue) / 3).pow(2);
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            red: 5,
            green: 4,
            blue: 3,
        }
    }
}

fn main() {
    let mut image = vec![Pixel::default(); WIDTH * HEIGHT];

    // loop over it
    let start = Instant::now();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            image[y * HEIGHT + x].modify()
        }
    }
    println!("{} us", start.elapsed().as_micros());
}
