use noise::{NoiseFn, Perlin};
use rand::Rng;
use rand_pcg::Pcg32;

use crate::{helpers::idx, params::PerlinParams};

#[inline]
// S-curve
fn apply_contrast_sigmoid(v: f64, contrast: f32) -> f64 {
    let k = 10.0; // keep it less or eq than 10.0
    let slope = k * contrast as f64 + 1.0;
    let centered = v - 0.5;
    let f_v = 1.0 / (1.0 + (-slope * centered).exp());
    f_v.clamp(0.0, 1.0)
}

#[inline]
// (1 - α)*a + α*b
fn lerp_color(a_greyscale: f64, tint: u8, brightness_weight: f64) -> u8 {
    ((1.0 - brightness_weight) * a_greyscale + brightness_weight * (tint as f64))
        .round()
        .clamp(0.0, 255.0) as u8
}

#[inline]
fn tint_lerp(v: f64, tint: (u8, u8, u8)) -> [u8; 4] {
    let greyscale = (v * 255.0).round();

    let r = lerp_color(greyscale, tint.0, v);
    let g = lerp_color(greyscale, tint.1, v);
    let b = lerp_color(greyscale, tint.2, v);

    [r, g, b, 255]
}

pub fn gen_perlin_noise(
    data: &mut Vec<u8>,
    w: u32,
    h: u32,
    rng: &mut Pcg32,
    params: &PerlinParams,
) {
    let perlin = Perlin::new(rng.random::<u32>());
    let &PerlinParams {
        scale,
        octaves,
        persistence,
        lacunarity,
        contrast,
        tint,
    } = params;

    for y in 0..h {
        for x in 0..w {
            let nx = x as f64 * scale;
            let ny = y as f64 * scale;

            let mut value = 0.0;
            let mut amp = 1.0;
            let mut freq = 1.0;
            let mut amp_sum = 0.0;

            for _ in 0..octaves {
                value += perlin.get([nx * freq, ny * freq]) * amp;
                amp_sum += amp;
                amp *= persistence as f64;
                freq *= lacunarity as f64;
            }

            let v = ((value / amp_sum).clamp(-1.0, 1.0) + 1.0) * 0.5;

            let vc = apply_contrast_sigmoid(v, contrast);
            let px = tint_lerp(vc, tint);

            let i = idx(x, y, w);
            data[i..i + 4].copy_from_slice(&px);
        }
    }
}
