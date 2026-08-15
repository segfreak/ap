use crate::integral::{ApInt, LIMB_BITS, Limbs, detail::limbs};

pub(super) fn schoolbook_mul(a: &ApInt, b: &ApInt) -> ApInt {
    assert_eq!(a.width(), b.width());

    if a.is_zero() || b.is_zero() {
        return ApInt::zero(a.width());
    }

    let limbs = schoolbook_mul_limbs(a.get_limbs(), b.get_limbs());

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

pub(super) fn schoolbook_mul_limbs(a: &[u64], b: &[u64]) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }

    let mut result = vec![0u64; a.len() + b.len()];

    for i in 0..a.len() {
        let mut carry = 0u64;

        for j in 0..b.len() {
            let index = i + j;

            let product = a[i] as u128 * b[j] as u128;

            let product_low = product as u64;
            let product_high = (product >> 64) as u64;

            let (x, carry_low) = result[index].overflowing_add(product_low);

            let (x, carry_carry) = x.overflowing_add(carry);

            result[index] = x;

            let mut next_carry = product_high.wrapping_add(carry_low as u64);

            next_carry = next_carry.wrapping_add(carry_carry as u64);

            carry = next_carry;
        }

        let mut index = i + b.len();

        while carry != 0 {
            let (x, overflow) = result[index].overflowing_add(carry);

            result[index] = x;
            carry = overflow as u64;

            index += 1;
        }
    }

    limbs::trim_limbs(&mut result);

    result
}

impl ApInt {
    // pub(super) fn _mul_schoolbook_impl(&self, rhs: &Self) -> Self {
    //     let n = self.limbs.len();
    //     let mut result = vec![0u64; n];

    //     for i in 0..n {
    //         let mut carry = 0u64;

    //         for j in 0..n.saturating_sub(i) {
    //             let k = i + j;

    //             let product = self.limbs[i] as u128 * rhs.limbs[j] as u128;
    //             let addend = result[k] as u128 + product + carry as u128;

    //             result[k] = addend as u64;
    //             carry = (addend >> 64) as u64;
    //         }

    //         let k = i + (n - i).min(n.saturating_sub(i));
    //         if k < n {
    //             let (value, _) = result[k].overflowing_add(carry);
    //             result[k] = value;
    //         }
    //     }

    //     Self::from_limbs(self.width, result)
    // }

    pub(super) fn _mul_schoolbook_impl(&self, rhs: &Self) -> Self {
        schoolbook_mul(self, rhs)
    }
}
