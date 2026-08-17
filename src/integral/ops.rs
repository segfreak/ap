use crate::integral::ApInt;
use std::ops::{
    Add, AddAssign, BitAnd, BitOr, BitXor, Div, DivAssign, Mul, MulAssign, Neg, Not, Shl, Shr, Sub,
    SubAssign,
};

impl Add for ApInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        ApInt::add(&self, &rhs)
    }
}

impl Add<&ApInt> for &ApInt {
    type Output = ApInt;

    fn add(self, rhs: &ApInt) -> ApInt {
        ApInt::add(self, rhs)
    }
}

impl AddAssign for ApInt {
    fn add_assign(&mut self, rhs: Self) {
        ApInt::add_assign(self, &rhs);
    }
}

impl Sub for ApInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        ApInt::sub(&self, &rhs)
    }
}

impl Sub<&ApInt> for &ApInt {
    type Output = ApInt;

    fn sub(self, rhs: &ApInt) -> ApInt {
        ApInt::sub(self, rhs)
    }
}

impl SubAssign for ApInt {
    fn sub_assign(&mut self, rhs: Self) {
        ApInt::sub_assign(self, &rhs);
    }
}

impl Mul for ApInt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        ApInt::mul(&self, &rhs)
    }
}

impl Mul<&ApInt> for &ApInt {
    type Output = ApInt;

    fn mul(self, rhs: &ApInt) -> ApInt {
        ApInt::mul(self, rhs)
    }
}

impl MulAssign for ApInt {
    fn mul_assign(&mut self, rhs: Self) {
        ApInt::mul_assign(self, &rhs);
    }
}

/// Implements signed division for `ApInt` using the `/` operator.
///
/// # Panics
///
/// Panics if the divisor is zero or widths differ.
///
/// # Examples
///
/// ```
/// use ap::integral::ApInt;
///
/// let a = ApInt::new(8, 0xff); // -1
/// let b = ApInt::new(8, 2);
/// let c = a / b;
/// assert_eq!(c.get_limbs()[0], 0); // 0
/// ```
impl Div for ApInt {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        ApInt::sdiv(&self, &rhs)
    }
}

/// Implements signed division for `ApInt` using the `/` operator.
///
/// # Panics
///
/// Panics if the divisor is zero or widths differ.
///
/// # Examples
///
/// ```
/// use ap::integral::ApInt;
///
/// let a = ApInt::new(8, 0xff); // -1
/// let b = ApInt::new(8, 2);
/// let c = &a / &b;
/// assert_eq!(c.get_limbs()[0], 0); // 0
/// ```
impl Div<&ApInt> for &ApInt {
    type Output = ApInt;

    fn div(self, rhs: &ApInt) -> ApInt {
        ApInt::sdiv(self, rhs)
    }
}

impl DivAssign for ApInt {
    fn div_assign(&mut self, rhs: Self) {
        ApInt::sdiv_assign(self, &rhs);
    }
}

impl Neg for ApInt {
    type Output = Self;

    fn neg(self) -> Self {
        ApInt::neg(&self)
    }
}

impl BitAnd for ApInt {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        ApInt::bitand(&self, &rhs)
    }
}

impl BitOr for ApInt {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        ApInt::bitor(&self, &rhs)
    }
}

impl BitXor for ApInt {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        ApInt::bitxor(&self, &rhs)
    }
}

impl Not for ApInt {
    type Output = Self;

    fn not(self) -> Self {
        ApInt::not(&self)
    }
}

impl Shl<usize> for ApInt {
    type Output = Self;

    fn shl(self, amount: usize) -> Self {
        ApInt::shl(&self, amount)
    }
}

impl Shr<usize> for ApInt {
    type Output = Self;

    fn shr(self, amount: usize) -> Self {
        ApInt::lshr(&self, amount)
    }
}
