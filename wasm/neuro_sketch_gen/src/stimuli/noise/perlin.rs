use noise::{NoiseFn, Perlin};
use rand::Rng;
use rand_pcg::Pcg32;

use crate::idx;

pub fn gen_perlin_noise(data: &mut Vec<u8>, w: u32, h: u32, rng: &mut Pcg32) {
    let perlin = Perlin::new(rng.random());

    let scale = (w.min(h) as f64 / 3.0).max(1.0);
    let octaves = 5;
    let persistence = 0.35;
    let lacunarity = 2.0;

    for y in 0..h {
        for x in 0..w {
            let nx = x as f64 / scale;
            let ny = y as f64 / scale;

            let mut value = 0.0;
            let mut amp = 1.0;
            let mut freq = 1.0;
            let mut max_amp = 0.0;

            for _ in 0..octaves {
                value += perlin.get([nx * freq, ny * freq]) * amp;
                max_amp += amp;
                amp *= persistence;
                freq *= lacunarity;
            }

            value /= max_amp;

            let v = ((value * 0.5 + 0.5) * 255.0).round().clamp(0.0, 255.0) as u8;

            let i = idx(x, y, w);
            data[i..i + 4].copy_from_slice(&[v, v, v, 255]);
        }
    }
}
