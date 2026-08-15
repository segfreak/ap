pub fn mul_u64(a: u64, b: u64) -> (u64, u64) {
    let a0 = a as u32 as u64;
    let a1 = a >> 32;

    let b0 = b as u32 as u64;
    let b1 = b >> 32;

    let p00 = a0 * b0;
    let p01 = a0 * b1;
    let p10 = a1 * b0;
    let p11 = a1 * b1;

    let middle = (p00 >> 32)
        .wrapping_add(p01 as u32 as u64)
        .wrapping_add(p10 as u32 as u64);

    let lo = (p00 & 0xffff_ffff) | (middle << 32);

    let hi = p11
        .wrapping_add(p01 >> 32)
        .wrapping_add(p10 >> 32)
        .wrapping_add(middle >> 32);

    (lo, hi)
}
