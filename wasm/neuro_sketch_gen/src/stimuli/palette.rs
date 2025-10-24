use crate::helpers::{hsl_to_rgb, idx};
use rand::Rng;
use rand_pcg::Pcg32;

pub fn gen_palette(data: &mut Vec<u8>, w: u32, h: u32, rng: &mut Pcg32) {
    let bars = 6u32;

    for bar in 0..bars {
        let h_deg = rng.random_range(0.0..360.0);
        let s = rng.random_range(0.45..0.85);
        let l = rng.random_range(0.35..0.70);
        let (r, g, b) = hsl_to_rgb(h_deg, s, l);

        let x0 = (w * bar) / bars;
        let x1 = (w * (bar + 1)) / bars;

        for y in 0..h {
            for x in x0..x1 {
                let i = idx(x, y, w);
                data[i..i + 4].copy_from_slice(&[r, g, b, 255]);
            }
        }
    }
}
