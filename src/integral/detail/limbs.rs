use crate::integral::{ApInt, LIMB_BITS, Limb};

    /// Clears unused high bits in the most significant limb.
    pub(crate)fn clear_high_bits(limbs: &mut [Limb], width: usize) {
        let used_bits = width % LIMB_BITS;

        if used_bits != 0
            && let Some(last) = limbs.last_mut()
        {
            *last &= ApInt::mask(used_bits);
        }
    }


pub(super) fn add_limbs(a: &[Limb], b: &[Limb]) -> Vec<Limb> {
    let n = a.len().max(b.len());

    let mut result = Vec::with_capacity(n + 1);

    let mut carry = 0u64;

    for i in 0..n {
        let x = a.get(i).copied().unwrap_or(0);
        let y = b.get(i).copied().unwrap_or(0);

        let (sum, carry_a) = x.overflowing_add(y);

        let (sum, carry_b) = sum.overflowing_add(carry);

        result.push(sum);

        carry = (carry_a || carry_b) as Limb;
    }

    if carry != 0 {
        result.push(carry);
    }

    result
}

pub(super) fn sub_limbs(a: &mut Vec<Limb>, b: &[Limb]) {
    debug_assert!(compare_limbs(a, b) != std::cmp::Ordering::Less);

    let mut borrow = 0u64;

    for i in 0..a.len() {
        let x = a[i];
        let y = b.get(i).copied().unwrap_or(0);

        let (value, borrow_a) = x.overflowing_sub(y);

        let (value, borrow_b) = value.overflowing_sub(borrow);

        a[i] = value;

        borrow = (borrow_a || borrow_b) as Limb;
    }

    debug_assert_eq!(borrow, 0);

    trim_limbs(a);
}

pub(super) fn add_shifted(out: &mut [Limb], value: &[Limb], shift: usize) {
    if value.is_empty() {
        return;
    }

    debug_assert!(shift + value.len() <= out.len());

    let mut carry = 0u64;

    for i in 0..value.len() {
        let index = i + shift;

        let (x, carry_a) = out[index].overflowing_add(value[i]);

        let (x, carry_b) = x.overflowing_add(carry);

        out[index] = x;

        carry = (carry_a || carry_b) as Limb;
    }

    let mut index = shift + value.len();

    while carry != 0 {
        debug_assert!(index < out.len());

        let (x, overflow) = out[index].overflowing_add(carry);

        out[index] = x;

        carry = overflow as Limb;
        index += 1;
    }
}

pub(super) fn compare_limbs(a: &[Limb], b: &[Limb]) -> std::cmp::Ordering {
    let mut a_len = a.len();
    let mut b_len = b.len();

    while a_len != 0 && a[a_len - 1] == 0 {
        a_len -= 1;
    }

    while b_len != 0 && b[b_len - 1] == 0 {
        b_len -= 1;
    }

    if a_len != b_len {
        return a_len.cmp(&b_len);
    }

    for i in (0..a_len).rev() {
        match a[i].cmp(&b[i]) {
            std::cmp::Ordering::Equal => {}
            ordering => return ordering,
        }
    }

    std::cmp::Ordering::Equal
}

pub(super) fn trim_limbs(limbs: &mut Vec<Limb>) {
    while limbs.last().copied() == Some(0) {
        limbs.pop();
    }
}
