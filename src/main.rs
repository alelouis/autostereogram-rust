// original paper: https://www.cs.waikato.ac.nz/~ihw/papers/94-HWT-SI-IHW-SIRDS-paper.pdf

use image::{GenericImageView, ImageBuffer, RgbImage};
use rand::Rng;

fn separation(mu: f32, e: f32, z: f32) -> f32 {
    // computes stereo separation
    let s = (1. - mu * z) * e / (2. - mu * z);
    s as f32
}

fn generate_autostereogram() -> [[u8; 512]; 512] {
    // process the depth map Z to generate autostereogram
    let mut rng = rand::thread_rng();

    const MAX_X: usize = 512; // image X size
    const MAX_Y: usize = 512; // image Y size
    const DPI: f32 = 72.; // Dots Per Inch
    const E: f32 = 2.5 * DPI; // Eye to eye distance
    const MU: f32 = 1. / 5.; // near plane ratio (between far and image planes)

    let mut z: [[f32; MAX_X]; MAX_Y] = [[0.0; MAX_X]; MAX_Y];

    let img = image::open("rust.png").unwrap();
    for y in 0..MAX_Y {
        for x in 0..MAX_X {
            z[x][y] = img.get_pixel(x as u32, y as u32)[0] as f32 / 255.;
        }
    }

    let mut same: [u32; MAX_X] = [0; MAX_X];
    let mut pix: [u8; MAX_X] = [0; MAX_X];
    let mut image: [[u8; MAX_X]; MAX_Y] = [[0; MAX_X]; MAX_Y];

    // for every column
    for y in 0..MAX_Y {
        // reinit row
        for x in 0..MAX_X {
            same[x] = x as u32;
        }
        // first row process, left-right
        for x in 0..MAX_X {
            let s: f32 = separation(MU, E, z[x][y]);
            let left: f32 = x as f32 - s / 2.; // left ray image position
            let right: f32 = left + s; // right ray image position

            if (left >= 0.) & (right < MAX_Y as f32) {
                let mut left_i: u32 = left as u32; // left ray image index
                let mut right_i: u32 = right as u32; // left ray image index

                let mut l: u32 = same[left_i as usize]; // same as left ray image index
                // process rightwards until no constraint
                while (l != left_i) & (l != right_i) { 
                    if l < right_i {
                        left_i = l;
                        l = same[left_i as usize];
                    } else {
                        same[left_i as usize] = right_i;
                        left_i = right_i;
                        l = same[left_i as usize];
                        right_i = l;
                    }
                }
                // set left and right as same
                same[left_i as usize] = right_i;
            }
        }
        // second row process, right-left
        for x in (0..MAX_X).rev() {
            if same[x] == x as u32 {
                pix[x] = rng.gen_range(0..2) // if unconstrained, sample value
            } else {
                pix[x] = pix[same[x] as usize]; // else set value as right value
            }
            image[x][y] = pix[x]; // set image u8 pixel value
        }
    }
    image
}

fn main() {
    // main call
    let m = generate_autostereogram();
    // construct image buffer
    let mut img: RgbImage = ImageBuffer::new(512, 512);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let m_pix: u8 = 255 * m[x as usize][y as usize] as u8;
        *pixel = image::Rgb([m_pix, m_pix, m_pix]);
    }
    // save image as png
    img.save("autostereogram.png").unwrap();
}
