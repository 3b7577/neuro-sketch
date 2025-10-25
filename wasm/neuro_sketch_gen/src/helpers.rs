#[inline]
pub fn idx(x: u32, y: u32, w: u32) -> usize {
    ((y * w + x) * 4) as usize
}
