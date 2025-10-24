use crate::helpers::idx;
use rand::Rng;
use rand_pcg::Pcg32;

pub fn gen_noise(data: &mut Vec<u8>, w: u32, h: u32, rng: &mut Pcg32) {
    for y in 0..h {
        for x in 0..w {
            let gray = rng.random();
            let i = idx(x, y, w);
            data[i..i + 4].copy_from_slice(&[gray, gray, gray, 255]);
        }
    }
}

pub fn gen_colored_noise(data: &mut Vec<u8>, w: u32, h: u32, rng: &mut Pcg32) {
    for y in 0..h {
        for x in 0..w {
            let (r, g, b) = (rng.random(), rng.random(), rng.random());
            let i = idx(x, y, w);
            data[i..i + 4].copy_from_slice(&[r, g, b, 255]);
        }
    }
}
