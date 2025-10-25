use wasm_bindgen::{Clamped, prelude::*};

use rand::SeedableRng;
use rand_pcg::Pcg32;

pub mod helpers;
pub use helpers::{hsl_to_rgb, idx};

pub mod params;
pub mod stimuli;

use stimuli::{
    gen_colored_noise, gen_horizontal_palette, gen_noise, gen_palette, gen_perlin_noise,
};

use crate::params::{Mode, Params, params_from_seed};

#[wasm_bindgen]
pub fn generate(mode: u32, seed: u32, width: u32, height: u32) -> Clamped<Vec<u8>> {
    let mut rng = Pcg32::seed_from_u64(seed as u64);
    let mut data = vec![0u8; (width * height * 4) as usize];
    let requested_mode = Some(Mode::from_u32(mode));
    let (_chosen_mode, params) = params_from_seed(seed as u64, requested_mode);

    match params {
        Params::Noise(noise_params) => gen_noise(&mut data, width, height, &mut rng, &noise_params),
        Params::ColoredNoise(colored_noise_params) => {
            gen_colored_noise(&mut data, width, height, &mut rng, &colored_noise_params)
        }
        Params::Perlin(perlin_params) => {
            gen_perlin_noise(&mut data, width, height, &mut rng, &perlin_params)
        }
        Params::Palette(_palette_params) => gen_palette(&mut data, width, height, &mut rng),
        Params::HorizontalPalette(_horizontal_palette_params) => {
            gen_horizontal_palette(&mut data, width, height, &mut rng)
        }
    }

    Clamped(data)
}
