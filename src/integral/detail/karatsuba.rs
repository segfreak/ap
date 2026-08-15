use crate::integral::{
    ApInt, LIMB_BITS, Limbs,
    detail::{limbs, schoolbook},
};

const KARATSUBA_THRESHOLD: usize = 8;

pub(super) fn karatsuba_mul(a: &ApInt, b: &ApInt) -> ApInt {
    assert_eq!(a.width(), b.width());

    if a.is_zero() || b.is_zero() {
        return ApInt::zero(a.width());
    }

    let limbs = karatsuba_mul_limbs(a.get_limbs(), b.get_limbs());

    let mut result = Limbs::from(limbs);

    let limb_count = ApInt::num_limbs(a.width());
    result.truncate(limb_count);

    if !a.width().is_multiple_of(LIMB_BITS)
        && let Some(last) = result.last_mut()
    {
        let bits = a.width() % LIMB_BITS;
        *last &= (1u64 << bits) - 1;
    }

    ApInt::from_limbs(a.width(), result)
}

pub(super) fn karatsuba_mul_limbs(a: &[u64], b: &[u64]) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }

    if a.len().min(b.len()) <= KARATSUBA_THRESHOLD {
        return schoolbook::schoolbook_mul_limbs(a, b);
    }

    let n = a.len().max(b.len());
    let m = n / 2;

    let a_low = if a.len() > m { &a[..m] } else { a };

    let a_high = if a.len() > m { &a[m..] } else { &[] };

    let b_low = if b.len() > m { &b[..m] } else { b };

    let b_high = if b.len() > m { &b[m..] } else { &[] };

    let z0 = karatsuba_mul_limbs(a_low, b_low);
    let z2 = karatsuba_mul_limbs(a_high, b_high);

    let a_sum = limbs::add_limbs(a_low, a_high);
    let b_sum = limbs::add_limbs(b_low, b_high);

    let mut z1 = karatsuba_mul_limbs(&a_sum, &b_sum);

    limbs::sub_limbs(&mut z1, &z0);
    limbs::sub_limbs(&mut z1, &z2);

    let mut result = vec![0u64; a.len() + b.len()];

    limbs::add_shifted(&mut result, &z0, 0);
    limbs::add_shifted(&mut result, &z1, m);
    limbs::add_shifted(&mut result, &z2, 2 * m);

    limbs::trim_limbs(&mut result);

    result
}

impl ApInt {
    pub(super) fn _mul_karatsuba_impl(&self, rhs: &Self) -> Self {
        karatsuba_mul(self, rhs)
    }
}
