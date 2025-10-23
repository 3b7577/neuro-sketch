use wasm_bindgen::{Clamped, prelude::*};

use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let h = h.rem_euclid(360.0);

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let hp = h / 60.0;
    let x = c * (1.0 - ((hp % 2.0) - 1.0).abs());

    let (r1, g1, b1) = match hp.floor() as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        5 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };

    let m = l - c / 2.0;

    let to_u8 = |v: f32| ((v * 255.0).round()).clamp(0.0, 255.0) as u8;

    (to_u8(r1 + m), to_u8(g1 + m), to_u8(b1 + m))
}

#[inline]
fn idx(x: u32, y: u32, w: u32) -> usize {
    ((y * w + x) * 4) as usize
}

fn gen_noise(data: &mut Vec<u8>, w: u32, h: u32, rng: &mut Pcg32) {
    for y in 0..h {
        for x in 0..w {
            let gray = rng.random();
            let i = idx(x, y, w);
            data[i..i + 4].copy_from_slice(&[gray, gray, gray, 255]);
        }
    }
}

fn gen_palette(data: &mut Vec<u8>, w: u32, h: u32, rng: &mut Pcg32) {
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

#[wasm_bindgen]
pub fn generate(mode: u32, seed: u32, width: u32, height: u32) -> Clamped<Vec<u8>> {
    let mut rng = Pcg32::seed_from_u64(seed as u64);
    let mut data = vec![0u8; (width * height * 4) as usize];

    match mode {
        1 => gen_palette(&mut data, width, height, &mut rng),
        _ => gen_noise(&mut data, width, height, &mut rng),
    }

    Clamped(data)
}
