pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
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
pub fn idx(x: u32, y: u32, w: u32) -> usize {
    ((y * w + x) * 4) as usize
}
