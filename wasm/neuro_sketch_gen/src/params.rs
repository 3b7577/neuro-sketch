// create params that depends on seed for generating images with more exclusive features
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Noise = 0,
    ColoredNoise = 1,
    Perlin = 2,
    Palette = 3,
    HorizontalPalette = 4,
}

impl Mode {
    pub fn from_u32(v: u32) -> Mode {
        match v {
            1 => Mode::ColoredNoise,
            2 => Mode::Perlin,
            3 => Mode::Palette,
            4 => Mode::HorizontalPalette,
            _ => Mode::Noise,
        }
    }
}

#[derive(Clone, Debug)]
pub enum ColorScheme {
    Mono,
    Analogous,
    Complementary,
    Triadic,
}

#[derive(Clone, Debug)]
pub enum SortBy {
    Hue,
    Lightness,
    Saturation,
}

#[derive(Clone, Debug)]
pub enum Params {
    Noise(NoiseParams),
    ColoredNoise(ColoredNoiseParams),
    Perlin(PerlinParams),
    Palette(PaletteParams),
    HorizontalPalette(PaletteParams),
}

#[derive(Clone, Debug)]
pub struct NoiseParams {
    pub dither: f32, // 0..1
    pub gamma: f32,  // 0.7..2.2
}

#[derive(Clone, Debug)]

pub struct ColoredNoiseParams {
    pub hue_base: f32,   // 0..360
    pub hue_jitter: f32, // 0..60
    pub sat_min: f32,    // 0..1
    pub sat_max: f32,    // 0..1
    pub light_min: f32,  // 0..1
    pub light_max: f32,  // 0..1
    pub gamma: f32,      // 0.7..2.2
}

#[derive(Clone, Debug)]
pub struct PerlinParams {
    pub scale: f64,         // 0.004..0.05 "noise" crate expects f64
    pub octaves: u8,        // 1..6
    pub persistence: f32,   // 0.3..0.9
    pub lacunarity: f32,    // 1.8..2.6
    pub contrast: f32,      // 0..1
    pub tint: (u8, u8, u8), // dominant
}

#[derive(Clone, Debug)]
pub struct PaletteParams {
    pub swatches: u32, // 3..12
    pub hue_base: f32, // 0..360
    pub scheme: ColorScheme,
    pub sat: f32,       // 0.2..0.95
    pub light_min: f32, // 0.15..0.9
    pub light_max: f32, // 0.2..0.97
    pub sort_by: SortBy,
}

pub fn params_from_seed(seed: u64, mode: Option<Mode>) -> (Mode, Params) {
    let mut rng = Pcg32::seed_from_u64(seed);

    let chosen_mode = match mode {
        Some(m) => m,
        None => {
            let pick = rng.random_range(0..100);

            if pick < 15 {
                Mode::Noise
            } else if pick < 35 {
                Mode::ColoredNoise
            } else if pick < 65 {
                Mode::Perlin
            } else if pick < 85 {
                Mode::Palette
            } else {
                Mode::HorizontalPalette
            }
        }
    };

    let params = match chosen_mode {
        Mode::Noise => {
            let p = NoiseParams {
                dither: fr(&mut rng, 0.0, 1.0),
                gamma: fr(&mut rng, 0.7, 2.2),
            };

            Params::Noise(p)
        }

        Mode::ColoredNoise => {
            let hue_base = fr(&mut rng, 0.0, 360.0);
            let hue_jitter = fr(&mut rng, 0.0, 60.0);
            let (sat_min, sat_max) = sample_min_max_with_gap(&mut rng, 0.15, 0.98, 0.05);
            let (light_min, light_max) = sample_min_max_with_gap(&mut rng, 0.12, 0.98, 0.05);
            let gamma = fr(&mut rng, 0.8, 2.0);

            Params::ColoredNoise(ColoredNoiseParams {
                hue_base,
                hue_jitter,
                sat_min,
                sat_max,
                light_min,
                light_max,
                gamma,
            })
        }

        Mode::Perlin => {
            let scale = fr(&mut rng, 0.004, 0.05) as f64;
            let octaves = ir(&mut rng, 1, 6) as u8;
            let persistence = fr(&mut rng, 0.3, 0.9);
            let lacunarity = fr(&mut rng, 1.8, 2.6);
            let contrast = fr(&mut rng, 0.0, 1.0);
            let tint = (
                ir(&mut rng, 200, 255) as u8,
                ir(&mut rng, 200, 255) as u8,
                ir(&mut rng, 200, 255) as u8,
            );

            Params::Perlin(PerlinParams {
                scale,
                octaves,
                persistence,
                lacunarity,
                contrast,
                tint,
            })
        }

        Mode::Palette | Mode::HorizontalPalette => {
            let swatches = ir(&mut rng, 3, 12);
            let hue_base = fr(&mut rng, 0.0, 360.0);
            let scheme = {
                match rng.random_range(0..4) {
                    0 => ColorScheme::Mono,
                    1 => ColorScheme::Analogous,
                    2 => ColorScheme::Complementary,
                    _ => ColorScheme::Triadic,
                }
            };

            let sat = fr(&mut rng, 0.2, 0.95);

            let (light_min, light_max) = sample_min_max_with_gap(&mut rng, 0.15, 0.9, 0.05);

            let sort_by = {
                match rng.random_range(0..3) {
                    0 => SortBy::Hue,
                    1 => SortBy::Lightness,
                    _ => SortBy::Saturation,
                }
            };

            let p = PaletteParams {
                swatches,
                hue_base,
                scheme,
                sat,
                light_min,
                light_max,
                sort_by,
            };
            if chosen_mode == Mode::Palette {
                Params::Palette(p)
            } else {
                Params::HorizontalPalette(p)
            }
        }
    };

    (chosen_mode, params)
}

#[inline]
fn fr(rng: &mut Pcg32, low: f32, high: f32) -> f32 {
    debug_assert!(high >= low);
    low + rng.random::<f32>() * (high - low)
}

#[inline]
fn ir(rng: &mut Pcg32, low: u32, high: u32) -> u32 {
    debug_assert!(high >= low);
    rng.random_range(low..=high)
}

#[inline]
fn sample_min_max_with_gap(rng: &mut Pcg32, low: f32, high: f32, gap: f32) -> (f32, f32) {
    let low = low.min(high);
    let high = high.max(low + f32::EPSILON);
    let gap = gap.clamp(0.0, high - low);

    let min_high = (high - gap).max(low);
    let min_v = fr(rng, low, min_high);

    let max_low = (min_v + gap).min(high);
    let max_v = fr(rng, max_low, high);

    (min_v, max_v)
}
