use wasm_bindgen::{Clamped, prelude::*};

use rand::{RngCore, SeedableRng};
use rand_pcg::Pcg32;

pub mod helpers;
pub mod params;
pub mod stimuli;

use crate::params::{Mode, Params, params_from_rng};
use stimuli::{gen_colored_noise, gen_noise, gen_palette, gen_perlin_noise};

fn spawn_rngs(seed: u64) -> (Pcg32, Pcg32) {
    let mut main = Pcg32::seed_from_u64(seed);
    let s_params = main.next_u64();
    let s_pixels = main.next_u64();
    (
        Pcg32::seed_from_u64(s_params),
        Pcg32::seed_from_u64(s_pixels),
    )
}

#[wasm_bindgen]
pub fn generate(mode: u32, seed: u32, width: u32, height: u32) -> Clamped<Vec<u8>> {
    let (mut rng_params, mut rng_pixels) = spawn_rngs(seed as u64);
    let mut data = vec![0u8; (width * height * 4) as usize];
    let requested_mode = Mode::try_from(mode).ok();
    let (_chosen_mode, params) = params_from_rng(&mut rng_params, requested_mode);

    match params {
        Params::Noise(noise_params) => {
            gen_noise(&mut data, width, height, &mut rng_pixels, &noise_params)
        }
        Params::ColoredNoise(colored_noise_params) => gen_colored_noise(
            &mut data,
            width,
            height,
            &mut rng_pixels,
            &colored_noise_params,
        ),
        Params::Perlin(perlin_params) => {
            gen_perlin_noise(&mut data, width, height, &mut rng_pixels, &perlin_params)
        }
        Params::Palette(palette_params) => {
            gen_palette(&mut data, width, height, &mut rng_pixels, &palette_params)
        }
    }

    Clamped(data)
}
