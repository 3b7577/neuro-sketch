use crate::{
    helpers::idx,
    params::{ColorScheme, PaletteParams},
};
use palette::FromColor;
use palette::{Hsl, Srgb};

use rand::Rng;
use rand_pcg::Pcg32;

#[inline]
fn wrap_deg(h: f32) -> f32 {
    h.rem_euclid(360.0)
}

fn scheme_hues(base: f32, scheme: ColorScheme) -> Vec<f32> {
    match scheme {
        ColorScheme::Mono => vec![base],
        ColorScheme::Analogous => vec![base - 30.0, base, base + 30.0],
        ColorScheme::Complementary => vec![base, base + 180.0],
        ColorScheme::Triadic => vec![base, base + 120.0, base + 240.0],
    }
    .into_iter()
    .map(wrap_deg)
    .collect()
}

#[inline]
fn paint_vertical_bar(data: &mut [u8], w: u32, h: u32, x0: u32, x1: u32, rgba: [u8; 4]) {
    for y in 0..h {
        for x in x0..x1 {
            let i = idx(x, y, w);
            data[i..i + 4].copy_from_slice(&rgba);
        }
    }
}

pub fn gen_palette(
    data: &mut Vec<u8>,
    width: u32,
    height: u32,
    rng: &mut Pcg32,
    params: &PaletteParams,
) {
    let &PaletteParams {
        swatches,
        hue_base,
        scheme,
        sat,
        light_min,
        light_max,
    } = params;

    let anchors = scheme_hues(hue_base, scheme);

    for i in 0..swatches {
        let a = anchors[(i as usize) % anchors.len()];
        let h = wrap_deg(a + rng.random_range(-8.0..8.0));
        let s = (sat + rng.random_range(-0.05..0.05)).clamp(0.0, 1.0);
        let l = rng.random_range(light_min..light_max);

        let hsl = Hsl::new(h, s, l);

        let rgb: Srgb<u8> = Srgb::from_color(hsl).into_format();
        let x0 = (width * i as u32) / swatches;
        let x1 = (width * (i as u32 + 1)) / swatches;

        paint_vertical_bar(
            data,
            width,
            height,
            x0,
            x1,
            [rgb.red, rgb.green, rgb.blue, 255],
        );
    }
}
