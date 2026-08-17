use crate::integral::ApInt;

impl ApInt {
    pub(super) fn _udiv_knuthd_impl(&self, rhs: &Self) -> Self {
        let n = self.limbs.len();

        if self.is_zero() {
            return Self::zero(self.width);
        }

        if self.ult(rhs) {
            return Self::zero(self.width);
        }

        if n == 1 {
            return Self::from_limbs(self.width, vec![self.limbs[0] / rhs.limbs[0]]);
        }

        let divisor_len = rhs.limbs.iter().rposition(|&x| x != 0).map_or(0, |i| i + 1);

        if divisor_len == 1 {
            let divisor = rhs.limbs[0];

            let mut quotient = vec![0u64; n];
            let mut remainder = 0u64;

            for i in (0..n).rev() {
                let dividend = ((remainder as u128) << 64) | self.limbs[i] as u128;

                quotient[i] = (dividend / divisor as u128) as u64;

                remainder = (dividend % divisor as u128) as u64;
            }

            return Self::from_limbs(self.width, quotient);
        }

        let u_len = self.limbs.len();
        let v_len = divisor_len;

        let shift = rhs.limbs[v_len - 1].leading_zeros();

        let mut v = vec![0u64; v_len];
        let mut carry = 0u64;

        if shift == 0 {
            v.copy_from_slice(&rhs.limbs[..v_len]);
        } else {
            for (i, &x) in rhs.limbs.iter().enumerate().take(v_len) {
                v[i] = (x << shift) | carry;
                carry = x >> (64 - shift);
            }
        }

        let mut u = vec![0u64; u_len + 1];

        if shift == 0 {
            u[..u_len].copy_from_slice(&self.limbs);
        } else {
            carry = 0;

            for (i, &x) in self.limbs.iter().enumerate().take(u_len) {
                u[i] = (x << shift) | carry;
                carry = x >> (64 - shift);
            }

            u[u_len] = carry;
        }

        let m = u_len - v_len;
        let mut q = vec![0u64; m + 1];

        let v_most = v[v_len - 1];
        let v_next = v[v_len - 2];

        for j in (0..=m).rev() {
            let numerator = ((u[j + v_len] as u128) << 64) | u[j + v_len - 1] as u128;

            let mut qhat = numerator / v_most as u128;
            let mut rhat = numerator % v_most as u128;

            if qhat == (1u128 << 64) {
                qhat -= 1;
                rhat += v_most as u128;
            }

            while qhat * v_next as u128 > (rhat << 64) + u[j + v_len - 2] as u128 {
                qhat -= 1;
                rhat += v_most as u128;

                if rhat >= (1u128 << 64) {
                    break;
                }
            }

            let qhat64 = qhat as u64;

            let mut borrow = 0u64;
            let mut carry_mul = 0u64;

            for i in 0..v_len {
                let product = v[i] as u128 * qhat64 as u128 + carry_mul as u128;

                let product_lo = product as u64;
                carry_mul = (product >> 64) as u64;

                let (x, b1) = u[j + i].overflowing_sub(product_lo);

                let (x, b2) = x.overflowing_sub(borrow);

                u[j + i] = x;
                borrow = (b1 || b2) as u64;
            }

            let (x, b1) = u[j + v_len].overflowing_sub(carry_mul);

            let (x, b2) = x.overflowing_sub(borrow);

            u[j + v_len] = x;

            if b1 || b2 {
                q[j] = qhat64.wrapping_sub(1);

                let mut carry_add = 0u64;

                for i in 0..v_len {
                    let (x, c1) = u[j + i].overflowing_add(v[i]);

                    let (x, c2) = x.overflowing_add(carry_add);

                    u[j + i] = x;
                    carry_add = (c1 || c2) as u64;
                }

                let (x, _) = u[j + v_len].overflowing_add(carry_add);

                u[j + v_len] = x;
            } else {
                q[j] = qhat64;
            }
        }

        Self::from_limbs(self.width, q)
    }

    pub(super) fn _urem_knuthd_impl(&self, rhs: &Self) -> Self {
        let n = self.limbs.len();

        if self.is_zero() {
            return Self::zero(self.width);
        }

        if self.ult(rhs) {
            return self.clone();
        }

        if n == 1 {
            return Self::from_limbs(self.width, vec![self.limbs[0] % rhs.limbs[0]]);
        }

        let divisor_len = rhs.limbs.iter().rposition(|&x| x != 0).map_or(0, |i| i + 1);

        if divisor_len == 1 {
            let divisor = rhs.limbs[0];

            let mut remainder = 0u64;

            for i in (0..n).rev() {
                let dividend = ((remainder as u128) << 64) | self.limbs[i] as u128;

                remainder = (dividend % divisor as u128) as u64;
            }

            return Self::from_limbs(self.width, vec![remainder]);
        }

        let u_len = self.limbs.len();
        let v_len = divisor_len;

        let shift = rhs.limbs[v_len - 1].leading_zeros();

        let mut v = vec![0u64; v_len];
        let mut carry = 0u64;

        if shift == 0 {
            v.copy_from_slice(&rhs.limbs[..v_len]);
        } else {
            for (i, &x) in rhs.limbs.iter().enumerate().take(v_len) {
                v[i] = (x << shift) | carry;
                carry = x >> (64 - shift);
            }
        }

        let mut u = vec![0u64; u_len + 1];

        if shift == 0 {
            u[..u_len].copy_from_slice(&self.limbs);
        } else {
            carry = 0;

            for (i, &x) in self.limbs.iter().enumerate().take(u_len) {
                u[i] = (x << shift) | carry;
                carry = x >> (64 - shift);
            }

            u[u_len] = carry;
        }

        let m = u_len - v_len;

        let v_most = v[v_len - 1];
        let v_next = v[v_len - 2];

        for j in (0..=m).rev() {
            let numerator = ((u[j + v_len] as u128) << 64) | u[j + v_len - 1] as u128;

            let mut qhat = numerator / v_most as u128;
            let mut rhat = numerator % v_most as u128;

            if qhat == (1u128 << 64) {
                qhat -= 1;
                rhat += v_most as u128;
            }

            while qhat * v_next as u128 > (rhat << 64) + u[j + v_len - 2] as u128 {
                qhat -= 1;
                rhat += v_most as u128;

                if rhat >= (1u128 << 64) {
                    break;
                }
            }

            let qhat64 = qhat as u64;

            let mut borrow = 0u64;
            let mut carry_mul = 0u64;

            for i in 0..v_len {
                let product = v[i] as u128 * qhat64 as u128 + carry_mul as u128;

                let product_lo = product as u64;
                carry_mul = (product >> 64) as u64;

                let (x, b1) = u[j + i].overflowing_sub(product_lo);

                let (x, b2) = x.overflowing_sub(borrow);

                u[j + i] = x;
                borrow = (b1 || b2) as u64;
            }

            let (x, b1) = u[j + v_len].overflowing_sub(carry_mul);

            let (x, b2) = x.overflowing_sub(borrow);

            u[j + v_len] = x;

            if b1 || b2 {
                let mut carry_add = 0u64;

                for i in 0..v_len {
                    let (x, c1) = u[j + i].overflowing_add(v[i]);

                    let (x, c2) = x.overflowing_add(carry_add);

                    u[j + i] = x;
                    carry_add = (c1 || c2) as u64;
                }

                let (x, _) = u[j + v_len].overflowing_add(carry_add);

                u[j + v_len] = x;
            }
        }

        let mut r = vec![0u64; v_len];

        if shift == 0 {
            r.copy_from_slice(&u[..v_len]);
        } else {
            for i in 0..v_len {
                let low = u[i] >> shift;

                let high = if i + 1 < v_len {
                    u[i + 1] << (64 - shift)
                } else {
                    0
                };

                r[i] = low | high;
            }
        }

        Self::from_limbs(self.width, r)
    }

    pub(crate) fn _udivrem_knuthd_impl(&self, rhs: &Self) -> (Self, Self) {
        let n = self.limbs.len();

        // Fast path for zero.
        if self.is_zero() {
            return (Self::zero(self.width), Self::zero(self.width));
        }

        // Fast path: dividend < divisor.
        if self.ult(rhs) {
            return (Self::zero(self.width), self.clone());
        }

        // Fast path for single-limb division.
        if n == 1 {
            let q = self.limbs[0] / rhs.limbs[0];
            let r = self.limbs[0] % rhs.limbs[0];

            return (
                Self::from_limbs(self.width, vec![q]),
                Self::from_limbs(self.width, vec![r]),
            );
        }

        let divisor_len = rhs.limbs.iter().rposition(|&x| x != 0).map_or(0, |i| i + 1);

        if divisor_len == 1 {
            let divisor = rhs.limbs[0];
            let mut quotient = vec![0u64; n];
            let mut remainder = 0u64;

            for i in (0..n).rev() {
                let dividend = ((remainder as u128) << 64) | self.limbs[i] as u128;
                quotient[i] = (dividend / divisor as u128) as u64;
                remainder = (dividend % divisor as u128) as u64;
            }

            return (
                Self::from_limbs(self.width, quotient),
                Self::from_limbs(self.width, vec![remainder]),
            );
        }

        // Knuth Algorithm D.
        // B = 2^64.
        // u = dividend
        // v = divisor
        let u_len = self.limbs.len();
        let v_len = divisor_len;

        let shift = rhs.limbs[v_len - 1].leading_zeros();

        let mut v = vec![0u64; v_len];
        let mut carry = 0u64;

        if shift == 0 {
            v.copy_from_slice(&rhs.limbs[..v_len]);
        } else {
            for (i, out) in v.iter_mut().enumerate().take(v_len) {
                let x = rhs.limbs[i];
                *out = (x << shift) | carry;
                carry = x >> (64 - shift);
            }
        }

        let mut u = vec![0u64; u_len + 1];

        if shift == 0 {
            u[..u_len].copy_from_slice(&self.limbs);
        } else {
            carry = 0;

            for (i, out) in u.iter_mut().enumerate().take(u_len) {
                let x = self.limbs[i];
                *out = (x << shift) | carry;
                carry = x >> (64 - shift);
            }

            u[u_len] = carry;
        }

        let m = u_len - v_len;
        let mut q = vec![0u64; m + 1];

        let v_most = v[v_len - 1];
        let v_next = v[v_len - 2];

        for j in (0..=m).rev() {
            let numerator = ((u[j + v_len] as u128) << 64) | u[j + v_len - 1] as u128;

            let mut qhat = numerator / v_most as u128;
            let mut rhat = numerator % v_most as u128;

            if qhat == (1u128 << 64) {
                qhat -= 1;
                rhat += v_most as u128;
            }

            if v_len >= 2 {
                while qhat * v_next as u128 > (rhat << 64) + u[j + v_len - 2] as u128 {
                    qhat -= 1;
                    rhat += v_most as u128;

                    if rhat >= (1u128 << 64) {
                        break;
                    }
                }
            }

            let qhat64 = qhat as u64;

            let mut borrow = 0u64;
            let mut carry_mul = 0u64;

            for i in 0..v_len {
                let product = v[i] as u128 * qhat64 as u128 + carry_mul as u128;

                let product_lo = product as u64;
                carry_mul = (product >> 64) as u64;

                let (x, b1) = u[j + i].overflowing_sub(product_lo);
                let (x, b2) = x.overflowing_sub(borrow);

                u[j + i] = x;
                borrow = (b1 || b2) as u64;
            }

            let (x, b1) = u[j + v_len].overflowing_sub(carry_mul);
            let (x, b2) = x.overflowing_sub(borrow);

            u[j + v_len] = x;

            let negative = b1 || b2;

            if negative {
                q[j] = qhat64.wrapping_sub(1);

                let mut carry_add = 0u64;

                for i in 0..v_len {
                    let (x, c1) = u[j + i].overflowing_add(v[i]);
                    let (x, c2) = x.overflowing_add(carry_add);

                    u[j + i] = x;
                    carry_add = (c1 || c2) as u64;
                }

                let (x, _) = u[j + v_len].overflowing_add(carry_add);
                u[j + v_len] = x;
            } else {
                q[j] = qhat64;
            }
        }

        let mut r = vec![0u64; v_len];

        if shift == 0 {
            r.copy_from_slice(&u[..v_len]);
        } else {
            for i in 0..v_len {
                let low = u[i] >> shift;

                let high = if i + 1 < v_len {
                    u[i + 1] << (64 - shift)
                } else {
                    0
                };

                r[i] = low | high;
            }
        }

        (
            Self::from_limbs(self.width, q),
            Self::from_limbs(self.width, r),
        )
    }
}
