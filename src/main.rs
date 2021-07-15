// original paper: https://www.cs.waikato.ac.nz/~ihw/papers/94-HWT-SI-IHW-SIRDS-paper.pdf

use image::{GenericImageView, ImageBuffer, RgbImage};
use rand::Rng;

const MAX_X: usize = 512; // image X size
const MAX_Y: usize = 512; // image Y size

fn separation(mu: f64, e: f64, z: f64) -> f64 {
    // computes stereo separation
    let s = (1. - mu * z) * e / (2. - mu * z);
    s
}

fn generate_autostereogram() -> [[u8; MAX_X]; MAX_X] {
    // process the depth map Z to generate autostereogram
    let mut rng = rand::thread_rng();

    const DPI: f64 = 72.; // Dots Per Inch
    const E: f64 = 2.5 * DPI; // Eye to eye distance
    const MU: f64 = 1. / 5.; // near plane ratio (between far and image planes)

    let mut z: [[f64; MAX_X]; MAX_Y] = [[0.0; MAX_X]; MAX_Y];

    let img = image::open("rust.png").unwrap();
    for y in 0..MAX_Y {
        for x in 0..MAX_X {
            z[x][y] = img.get_pixel(x as u32, y as u32)[0] as f64 / 255.;
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
            let s = separation(MU, E, z[x][y]);
            let left = x as f64 - s / 2.; // left ray image position
            let right = left + s; // right ray image position

            if (left >= 0.) & (right < MAX_Y as f64) {
                let mut left = left as u32; // left ray image index
                let mut right = right as u32; // left ray image index

                let mut l = same[left as usize]; // same as left ray image index
                // process rightwards until no constraint
                while (l != left) & (l != right) { 
                    if l < right {
                        left = l;
                        l = same[left as usize];
                    } else {
                        same[left as usize] = right;
                        left = right;
                        l = same[left as usize];
                        right = l;
                    }
                }
                // set left and right as same
                same[left as usize] = right;
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
    let mut img: RgbImage = ImageBuffer::new(
        MAX_X as u32, MAX_Y as u32);
        
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let x = x as usize;
        let y = y as usize;
        let m_pix: u8 = 255 * m[x][y] as u8;
        *pixel = image::Rgb([m_pix, m_pix, m_pix]);
    }
    // save image as png
    img.save("autostereogram.png").unwrap();
}
