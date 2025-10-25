use crate::{
    helpers::idx,
    params::{ColoredNoiseParams, NoiseParams},
};
use palette::FromColor;
use palette::{Hsl, Srgb};
use rand::Rng;
use rand_pcg::Pcg32;

pub fn gen_noise(data: &mut Vec<u8>, w: u32, h: u32, rng: &mut Pcg32, params: &NoiseParams) {
    for y in 0..h {
        for x in 0..w {
            let mut gray = rng.random::<u8>() as f32 / 255.0;

            let jitter = (rng.random::<f32>() - 0.5) * 2.0 * params.dither;
            gray = (gray + jitter).clamp(0.0, 1.0);

            gray = gray.powf(1.0 / params.gamma);

            let i = idx(x, y, w);
            let v = (gray * 255.0) as u8;
            data[i..i + 4].copy_from_slice(&[v, v, v, 255]);
        }
    }
}

pub fn gen_colored_noise(
    data: &mut Vec<u8>,
    w: u32,
    h: u32,
    rng: &mut Pcg32,
    params: &ColoredNoiseParams,
) {
    let &ColoredNoiseParams {
        hue_base,
        hue_jitter,
        sat_min,
        sat_max,
        light_min,
        light_max,
        gamma,
    } = params;

    for y in 0..h {
        for x in 0..w {
            let hue = (hue_base + rng.random_range(-hue_jitter..hue_jitter)).rem_euclid(360.0);
            let sat = rng.random_range(sat_min..sat_max);
            let light = (rng.random_range(light_min..light_max)).powf(1.0 / gamma);

            let hsl = Hsl::new(hue, sat, light);
            let rgb: Srgb<u8> = Srgb::from_color(hsl).into_format();

            let i = idx(x, y, w);
            data[i..i + 4].copy_from_slice(&[rgb.red, rgb.green, rgb.blue, 255]);
        }
    }
}
