use wasm_bindgen::{Clamped, prelude::*};

use rand::SeedableRng;
use rand_pcg::Pcg32;

pub mod helpers;
pub use helpers::{hsl_to_rgb, idx};

pub mod stimuli;

use stimuli::{gen_colored_noise, gen_noise, gen_palette, gen_perlin_noise};

#[wasm_bindgen]
pub fn generate(mode: u32, seed: u32, width: u32, height: u32) -> Clamped<Vec<u8>> {
    let mut rng = Pcg32::seed_from_u64(seed as u64);
    let mut data = vec![0u8; (width * height * 4) as usize];

    match mode {
        1 => gen_colored_noise(&mut data, width, height, &mut rng),
        2 => gen_perlin_noise(&mut data, width, height, &mut rng),
        3 => gen_palette(&mut data, width, height, &mut rng),
        _ => gen_noise(&mut data, width, height, &mut rng),
    }

    Clamped(data)
}
