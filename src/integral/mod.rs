//! Arbitrary-precision integer arithmetic.
//!
//! This module provides `ApInt`, an arbitrary-precision integer implementation
//! that supports integers of any bit width. Numbers are stored internally as
//! a vector of 64-bit limbs in little-endian order.
//!
//! # Examples
//!
//! ```
//! use ap::integral::ApInt;
//!
//! let a = ApInt::new(32, 100);
//! let b = ApInt::new(32, 50);
//! let sum = a.add(&b);
//! assert_eq!(sum.get_limbs()[0], 150);
//! ```

mod detail;

pub mod ops;
pub mod parse;

#[cfg(feature = "smallvec")]
use smallvec::SmallVec;
use std::cmp::Ordering;
use std::fmt;

use crate::integral::detail::limbs;

/// Type alias for a single limb (64-bit word).
pub type Limb = u64;

/// Number of bits in a limb.
pub const LIMB_BITS: usize = 64;

/// Container type for limbs storage.
///
/// This type alias provides flexible storage for limbs, allowing optimization
/// for small numbers while maintaining compatibility with larger ones.
///
/// # Storage Strategies
///
/// ## With `"smallvec"` feature (optimized)
///
/// Uses `SmallVec<[Limb; 1]>` which stores the first limb inline on the stack,
/// avoiding heap allocation for numbers up to 64 bits. This significantly
/// improves performance for common small integer operations.
///
/// ## Without `"smallvec"` feature (default)
///
/// Uses standard `Vec<Limb>` which always allocates on the heap.
/// This is simpler and has no additional dependencies.
///
/// # Performance Impact
///
/// The `SmallVec` optimization can reduce heap allocations by up to 90%
/// for workloads that primarily use small integers (<= 64 bits).
///
/// # Examples
///
/// ```
/// use ap::integral::Limbs;
///
/// // Create a new limbs container
/// let mut limbs = Limbs::new();
/// limbs.push(42);
/// limbs.push(0x123456789abcdef0);
///
/// // Limbs can be iterated over
/// for limb in &limbs {
///     println!("{:x}", limb);
/// }
/// ```
#[cfg(feature = "smallvec")]
pub type Limbs = SmallVec<[Limb; 1]>;

#[cfg(not(feature = "smallvec"))]
pub type Limbs = Vec<Limb>;

/// Arbitrary-precision integer with fixed bit width.
///
/// `ApInt` represents integers of a fixed width, supporting both signed and
/// unsigned interpretations. The internal representation uses a vector of
/// 64-bit limbs in little-endian order (least significant limb first).
///
/// # Examples
///
/// ```
/// use ap::integral::ApInt;
///
/// // Create 8-bit integers
/// let a = ApInt::new(8, 42);
/// let b = ApInt::new(8, 100);
///
/// // Arithmetic operations
/// let sum = a.add(&b);
/// assert_eq!(sum.get_limbs()[0], 142);
///
/// // Signed comparisons
/// let neg = ApInt::new(8, 0xff); // -1 in 8-bit two's complement
/// assert!(neg.is_negative());
/// assert!(neg.slt(&ApInt::new(8, 0)));
/// ```
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApInt {
    /// Bit width of the integer.
    width: usize,

    /// Little-endian limbs (least significant first).
    limbs: Limbs,
}

impl ApInt {
    /// Creates a new `ApInt` with the specified width and initial value.
    ///
    /// The value is truncated to fit within the specified width.
    ///
    /// # Panics
    ///
    /// Panics if `width == 0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let x = ApInt::new(32, 123);
    /// assert_eq!(x.width(), 32);
    /// assert_eq!(x.get_limbs()[0], 123);
    ///
    /// // Value is truncated to width
    /// let y = ApInt::new(8, 0x1234);
    /// assert_eq!(y.get_limbs()[0], 0x34);
    /// ```
    pub fn new(width: usize, value: u128) -> Self {
        assert!(width > 0, "width must be > 0");

        let num_limbs = Self::num_limbs(width);

        let mut result = Self {
            width,
            limbs: (0..num_limbs)
                .map(|i| match i {
                    0 => value as u64,
                    1 => (value >> 64) as u64,
                    _ => 0,
                })
                .collect(),
        };
        result.clear_unused_bits();
        result
    }

    /// Creates a new `ApInt` with all bits set to zero.
    ///
    /// # Panics
    ///
    /// Panics if `width == 0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let x = ApInt::zero(64);
    /// assert!(x.is_zero());
    /// assert_eq!(x.width(), 64);
    /// ```
    pub fn zero(width: usize) -> Self {
        assert!(width > 0, "width must be > 0");
        Self {
            width,
            limbs: std::iter::repeat_n(0, Self::num_limbs(width)).collect(),
        }
    }

    /// Creates a new `ApInt` with value 1.
    ///
    /// # Panics
    ///
    /// Panics if `width == 0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let x = ApInt::one(32);
    /// assert_eq!(x.get_limbs()[0], 1);
    /// ```
    pub fn one(width: usize) -> Self {
        Self::new(width, 1)
    }

    /// Creates a new `ApInt` from raw limbs.
    ///
    /// The limb vector is resized to match the required number of limbs for
    /// the specified width. Excess limbs are truncated, and missing limbs
    /// are zero-padded. Unused bits in the most significant limb are cleared.
    ///
    /// # Panics
    ///
    /// Panics if `width == 0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let x = ApInt::from_limbs(64, vec![0x123456789abcdef0]);
    /// assert_eq!(x.get_limbs()[0], 0x123456789abcdef0);
    ///
    /// // Extra limbs are truncated
    /// let y = ApInt::from_limbs(64, vec![0x123, 0x456, 0x789]);
    /// assert_eq!(y.get_limbs().len(), 1);
    /// assert_eq!(y.get_limbs()[0], 0x123);
    /// ```
    pub fn from_limbs<T>(width: usize, limbs: T) -> Self
    where
        T: Into<Limbs>,
    {
        assert!(width > 0, "width must be > 0");

        let needed = Self::num_limbs(width);
        let mut limbs = limbs.into();

        limbs.resize(needed, 0);
        limbs.truncate(needed);

        let mut result = Self { width, limbs };
        result.clear_unused_bits();
        result
    }

    /// Returns the number of limbs required to store the given bit width.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// assert_eq!(ApInt::num_limbs(1), 1);
    /// assert_eq!(ApInt::num_limbs(64), 1);
    /// assert_eq!(ApInt::num_limbs(65), 2);
    /// assert_eq!(ApInt::num_limbs(128), 2);
    /// ```
    pub fn num_limbs(width: usize) -> usize {
        width.div_ceil(LIMB_BITS)
    }

    /// Returns a reference to the internal limb vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let x = ApInt::from_limbs(128, vec![0x12345678, 0x9abcdef0]);
    /// let limbs = x.get_limbs();
    /// assert_eq!(limbs.len(), 2);
    /// assert_eq!(limbs[0], 0x12345678);
    /// assert_eq!(limbs[1], 0x9abcdef0);
    /// ```
    pub fn get_limbs(&self) -> &Limbs {
        &self.limbs
    }

    /// Returns the bit width of this integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let x = ApInt::new(64, 123);
    /// assert_eq!(x.width(), 64);
    /// ```
    pub fn width(&self) -> usize {
        self.width
    }

    /// Creates a mask with the specified number of low bits set.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// assert_eq!(ApInt::mask(0), 0);
    /// assert_eq!(ApInt::mask(8), 0xff);
    /// assert_eq!(ApInt::mask(16), 0xffff);
    /// assert_eq!(ApInt::mask(64), u64::MAX);
    /// ```
    pub fn mask(bits: usize) -> Limb {
        match bits {
            0 => 0,
            1..64 => (1u64 << bits) - 1,
            _ => u64::MAX,
        }
    }

    /// Returns `true` if the limbs are stored inline (no heap allocation).
    ///
    /// This is only meaningful when the `"smallvec"` feature is enabled.
    /// Without `"smallvec"`, this always returns `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let x = ApInt::new(8, 42);
    /// // With smallvec: true (1 limb fits inline)
    /// // Without smallvec: false (always heap-allocated)
    /// println!("Stored inline: {}", x.is_inline());
    /// ```
    pub fn is_inline(&self) -> bool {
        #[cfg(feature = "smallvec")]
        {
            self.limbs.len() <= 1
        }
        #[cfg(not(feature = "smallvec"))]
        {
            false
        }
    }

    /// Clears unused bits in this integer.
    fn clear_unused_bits(&mut self) {
        limbs::clear_high_bits(&mut self.limbs, self.width);
    }

    /// Returns `true` if this integer is negative (signed interpretation).
    ///
    /// Uses the most significant bit to determine sign.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let pos = ApInt::new(8, 127);
    /// assert!(!pos.is_negative());
    ///
    /// let neg = ApInt::new(8, 0xff); // -1
    /// assert!(neg.is_negative());
    /// ```
    pub fn is_negative(&self) -> bool {
        let bit = self.width - 1;
        let limb = bit / LIMB_BITS;
        let offset = bit % LIMB_BITS;

        ((self.limbs[limb] >> offset) & 1) != 0
    }

    /// Returns `true` if all bits are zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let x = ApInt::zero(64);
    /// assert!(x.is_zero());
    ///
    /// let y = ApInt::new(64, 1);
    /// assert!(!y.is_zero());
    /// ```
    pub fn is_zero(&self) -> bool {
        self.limbs.iter().all(|&x| x == 0)
    }

    /// Adds two integers of the same width.
    ///
    /// # Panics
    ///
    /// Panics if widths differ.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let a = ApInt::new(8, 100);
    /// let b = ApInt::new(8, 50);
    /// let c = a.add(&b);
    /// assert_eq!(c.get_limbs()[0], 150);
    /// ```
    pub fn add(&self, rhs: &Self) -> Self {
        assert_eq!(self.width, rhs.width);

        let mut result = self.clone();
        result.add_assign(rhs);
        result
    }

    /// Adds another integer to this one (in-place).
    ///
    /// # Panics
    ///
    /// Panics if widths differ.
    pub fn add_assign(&mut self, rhs: &Self) {
        assert_eq!(self.width, rhs.width);

        let mut carry = 0u64;

        for i in 0..self.limbs.len() {
            let (a, c1) = self.limbs[i].overflowing_add(rhs.limbs[i]);
            let (b, c2) = a.overflowing_add(carry);

            self.limbs[i] = b;
            carry = (c1 || c2) as u64;
        }

        self.clear_unused_bits();
    }

    /// Subtracts two integers of the same width.
    ///
    /// # Panics
    ///
    /// Panics if widths differ.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let a = ApInt::new(8, 100);
    /// let b = ApInt::new(8, 50);
    /// let c = a.sub(&b);
    /// assert_eq!(c.get_limbs()[0], 50);
    /// ```
    pub fn sub(&self, rhs: &Self) -> Self {
        assert_eq!(self.width, rhs.width);

        let mut result = self.clone();
        result.sub_assign(rhs);
        result
    }

    /// Subtracts another integer from this one (in-place).
    ///
    /// # Panics
    ///
    /// Panics if widths differ.
    pub fn sub_assign(&mut self, rhs: &Self) {
        assert_eq!(self.width, rhs.width);

        let mut borrow = 0u64;

        for i in 0..self.limbs.len() {
            let (a, b1) = self.limbs[i].overflowing_sub(rhs.limbs[i]);
            let (b, b2) = a.overflowing_sub(borrow);

            self.limbs[i] = b;
            borrow = (b1 || b2) as u64;
        }

        self.clear_unused_bits();
    }

    /// Multiplies two integers of the same width.
    ///
    /// Uses schoolbook multiplication with 64-bit limbs. For small numbers
    /// (single limb), this is a simple 64-bit multiplication. For larger
    /// numbers, it uses O(n²) multiplication where n is the number of limbs.
    ///
    /// # Panics
    ///
    /// Panics if widths differ.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let a = ApInt::new(32, 25);
    /// let b = ApInt::new(32, 4);
    /// let c = a.mul(&b);
    /// assert_eq!(c.get_limbs()[0], 100);
    /// ```
    pub fn mul(&self, rhs: &Self) -> Self {
        assert_eq!(self.width, rhs.width);

        self._mul_impl(rhs)
    }

    /// Multiplies this integer by another (in-place).
    ///
    /// # Panics
    ///
    /// Panics if widths differ.
    pub fn mul_assign(&mut self, rhs: &Self) {
        *self = self.mul(rhs);
    }

    /// Unsigned division: returns (quotient, remainder).
    ///
    /// Uses Knuth's Algorithm D for multi-limb division, with fast paths for
    /// single-limb divisors. This is the most efficient division algorithm
    /// for arbitrary-precision integers.
    ///
    /// # Algorithm
    ///
    /// 1. **Fast path**: If divisor is zero → panic
    /// 2. **Fast path**: If dividend is zero → returns (0, 0)
    /// 3. **Fast path**: If dividend < divisor → returns (0, dividend)
    /// 4. **Fast path**: If divisor fits in one limb → use base 2^64 division
    /// 5. **General case**: Knuth's Algorithm D with base 2^64 limbs
    ///
    /// # Complexity
    ///
    /// - Single-limb divisor: O(n) where n is the number of limbs
    /// - Multi-limb divisor: O(n²) in the worst case
    ///
    /// # Panics
    ///
    /// Panics if widths differ or if `rhs` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let a = ApInt::new(64, 100);
    /// let b = ApInt::new(64, 3);
    /// let (q, r) = a.udivrem(&b);
    /// assert_eq!(q.get_limbs()[0], 33);
    /// assert_eq!(r.get_limbs()[0], 1);
    /// ```
    pub fn udivrem(&self, rhs: &Self) -> (Self, Self) {
        assert_eq!(self.width, rhs.width);
        assert!(!rhs.is_zero(), "division by zero");

        self._udivrem_impl(rhs)
    }

    /// Signed division: returns (quotient, remainder).
    ///
    /// Uses truncated division semantics (like C and LLVM):
    /// - Quotient is rounded toward zero
    /// - Remainder has the same sign as the dividend
    ///
    /// # Panics
    ///
    /// Panics if widths differ or if `rhs` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let a = ApInt::new(8, 0xff); // -1
    /// let b = ApInt::new(8, 2);
    /// let (q, r) = a.sdivrem(&b);
    /// assert_eq!(q.get_limbs()[0], 0); // 0
    /// assert_eq!(r.get_limbs()[0], 0xff); // -1
    /// ```
    pub fn sdivrem(&self, rhs: &Self) -> (Self, Self) {
        assert_eq!(self.width, rhs.width);
        assert!(!rhs.is_zero(), "division by zero");

        if self.is_zero() {
            return (Self::zero(self.width), Self::zero(self.width));
        }

        let lhs_negative = self.is_negative();
        let rhs_negative = rhs.is_negative();

        // Taking the two's-complement negation is exactly what is needed
        // for the absolute value in fixed-width arithmetic, including
        // the minimum signed value.
        let lhs_abs = if lhs_negative {
            self.neg()
        } else {
            self.clone()
        };

        let rhs_abs = if rhs_negative { rhs.neg() } else { rhs.clone() };

        let (mut quotient, mut remainder) = lhs_abs.udivrem(&rhs_abs);

        if lhs_negative ^ rhs_negative {
            quotient = quotient.neg();
        }

        if lhs_negative {
            remainder = remainder.neg();
        }

        (quotient, remainder)
    }

    /// Unsigned division (quotient only).
    ///
    /// # Panics
    ///
    /// Panics if widths differ or if `rhs` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let a = ApInt::new(64, 100);
    /// let b = ApInt::new(64, 3);
    /// let q = a.udiv(&b);
    /// assert_eq!(q.get_limbs()[0], 33);
    /// ```
    pub fn udiv(&self, rhs: &Self) -> Self {
        assert_eq!(self.width, rhs.width);
        assert!(!rhs.is_zero(), "division by zero");

        self._udiv_impl(rhs)
    }

    /// Unsigned division this integer by another (in-place).
    ///
    /// Replaces `self` with the quotient of `self / rhs` using unsigned division.
    ///
    /// # Panics
    ///
    /// Panics if widths differ or if `rhs` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let mut a = ApInt::new(32, 100);
    /// let b = ApInt::new(32, 3);
    /// a.udiv_assign(&b);
    /// assert_eq!(a.get_limbs()[0], 33);
    /// ```
    pub fn udiv_assign(&mut self, rhs: &Self) {
        *self = self.udiv(rhs);
    }

    /// Unsigned remainder.
    ///
    /// # Panics
    ///
    /// Panics if widths differ or if `rhs` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let a = ApInt::new(64, 100);
    /// let b = ApInt::new(64, 3);
    /// let r = a.urem(&b);
    /// assert_eq!(r.get_limbs()[0], 1);
    /// ```
    pub fn urem(&self, rhs: &Self) -> Self {
        assert_eq!(self.width, rhs.width);
        assert!(!rhs.is_zero(), "division by zero");

        self._urem_impl(rhs)
    }

    /// Signed division (quotient only).
    ///
    /// # Panics
    ///
    /// Panics if widths differ or if `rhs` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let a = ApInt::new(8, 0x80); // -128
    /// let b = ApInt::new(8, 2);
    /// let q = a.sdiv(&b);
    /// assert_eq!(q.get_limbs()[0], 0xc0); // -64
    /// ```
    pub fn sdiv(&self, rhs: &Self) -> Self {
        self.sdivrem(rhs).0
    }

    /// Signed division this integer by another (in-place).
    ///
    /// Replaces `self` with the quotient of `self / rhs` using signed division.
    ///
    /// # Panics
    ///
    /// Panics if widths differ or if `rhs` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let mut a = ApInt::new(32, 100);
    /// let b = ApInt::new(32, 3);
    /// a.udiv_assign(&b);
    /// assert_eq!(a.get_limbs()[0], 33);
    /// ```
    pub fn sdiv_assign(&mut self, rhs: &Self) {
        *self = self.sdiv(rhs);
    }

    /// Signed remainder.
    ///
    /// # Panics
    ///
    /// Panics if widths differ or if `rhs` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let a = ApInt::new(8, 0xff); // -1
    /// let b = ApInt::new(8, 2);
    /// let r = a.srem(&b);
    /// assert_eq!(r.get_limbs()[0], 0xff); // -1
    /// ```
    pub fn srem(&self, rhs: &Self) -> Self {
        self.sdivrem(rhs).1
    }

    /// Negates this integer (two's complement).
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let x = ApInt::new(8, 5);
    /// let y = x.neg();
    /// assert_eq!(y.get_limbs()[0], 251); // -5 in 8-bit two's complement
    /// ```
    pub fn neg(&self) -> Self {
        let mut result = self.not();
        result.add_assign(&Self::one(self.width));
        result
    }

    /// Bitwise AND of two integers.
    ///
    /// # Panics
    ///
    /// Panics if widths differ.
    pub fn bitand(&self, rhs: &Self) -> Self {
        assert_eq!(self.width, rhs.width);

        let limbs = self
            .limbs
            .iter()
            .zip(&rhs.limbs)
            .map(|(&a, &b)| a & b)
            .collect();

        Self {
            width: self.width,
            limbs,
        }
    }

    /// Bitwise OR of two integers.
    ///
    /// # Panics
    ///
    /// Panics if widths differ.
    pub fn bitor(&self, rhs: &Self) -> Self {
        assert_eq!(self.width, rhs.width);

        let limbs = self
            .limbs
            .iter()
            .zip(&rhs.limbs)
            .map(|(&a, &b)| a | b)
            .collect();

        Self {
            width: self.width,
            limbs,
        }
    }

    /// Bitwise XOR of two integers.
    ///
    /// # Panics
    ///
    /// Panics if widths differ.
    pub fn bitxor(&self, rhs: &Self) -> Self {
        assert_eq!(self.width, rhs.width);

        let limbs = self
            .limbs
            .iter()
            .zip(&rhs.limbs)
            .map(|(&a, &b)| a ^ b)
            .collect();

        Self {
            width: self.width,
            limbs,
        }
    }

    /// Bitwise NOT (complement) of this integer.
    ///
    /// Unused bits are cleared after the operation.
    pub fn not(&self) -> Self {
        let limbs = self.limbs.iter().map(|&x| !x).collect();

        let mut result = Self {
            width: self.width,
            limbs,
        };

        result.clear_unused_bits();
        result
    }

    /// Logical left shift.
    ///
    /// Shifts bits to the left, filling with zeros on the right.
    /// If `amount >= width`, returns zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let x = ApInt::new(8, 0b1101);
    /// let y = x.shl(2);
    /// assert_eq!(y.get_limbs()[0], 0b110100);
    /// ```
    pub fn shl(&self, amount: usize) -> Self {
        if amount >= self.width {
            return Self::zero(self.width);
        }

        let limb_shift = amount / LIMB_BITS;
        let bit_shift = amount % LIMB_BITS;

        let mut result = Self::zero(self.width);

        for i in 0..self.limbs.len() {
            let dst = i + limb_shift;

            if dst >= result.limbs.len() {
                break;
            }

            result.limbs[dst] |= self.limbs[i] << bit_shift;

            if bit_shift != 0 && dst + 1 < result.limbs.len() {
                result.limbs[dst + 1] |= self.limbs[i] >> (LIMB_BITS - bit_shift);
            }
        }

        result.clear_unused_bits();
        result
    }

    /// Logical right shift.
    ///
    /// Shifts bits to the right, filling with zeros on the left.
    /// If `amount >= width`, returns zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let x = ApInt::new(8, 0b1101);
    /// let y = x.lshr(2);
    /// assert_eq!(y.get_limbs()[0], 0b11);
    /// ```
    pub fn lshr(&self, amount: usize) -> Self {
        if amount >= self.width {
            return Self::zero(self.width);
        }

        let limb_shift = amount / LIMB_BITS;
        let bit_shift = amount % LIMB_BITS;

        let mut result = Self::zero(self.width);

        for i in limb_shift..self.limbs.len() {
            let dst = i - limb_shift;

            result.limbs[dst] |= self.limbs[i] >> bit_shift;

            if bit_shift != 0 && dst > 0 {
                result.limbs[dst - 1] |= self.limbs[i] << (LIMB_BITS - bit_shift);
            }
        }

        result.clear_unused_bits();
        result
    }

    /// Arithmetic right shift (sign-extending).
    ///
    /// Shifts bits to the right, preserving the sign bit.
    /// If `amount >= width`, returns zero for positive numbers
    /// or all ones for negative numbers.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// // Positive number
    /// let x = ApInt::new(8, 0b10000000); // -128
    /// let y = x.ashr(1);
    /// assert_eq!(y.get_limbs()[0], 0b11000000); // -64
    ///
    /// // Negative number
    /// let x = ApInt::new(8, 0xff); // -1
    /// let y = x.ashr(1);
    /// assert_eq!(y.get_limbs()[0], 0xff); // -1
    /// ```
    pub fn ashr(&self, amount: usize) -> Self {
        if amount >= self.width {
            return if self.is_negative() {
                let mut result = Self::zero(self.width);

                for limb in &mut result.limbs {
                    *limb = u64::MAX;
                }

                result.clear_unused_bits();
                result
            } else {
                Self::zero(self.width)
            };
        }

        let mut result = self.lshr(amount);

        if self.is_negative() {
            let fill_from = self.width - amount;

            for bit in fill_from..self.width {
                let limb = bit / LIMB_BITS;
                let offset = bit % LIMB_BITS;

                result.limbs[limb] |= 1u64 << offset;
            }
        }

        result
    }

    /// Unsigned less-than comparison.
    ///
    /// # Panics
    ///
    /// Panics if widths differ.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let a = ApInt::new(8, 100);
    /// let b = ApInt::new(8, 200);
    /// assert!(a.ult(&b));
    /// ```
    pub fn ult(&self, rhs: &Self) -> bool {
        assert_eq!(self.width, rhs.width);

        for i in (0..self.limbs.len()).rev() {
            match self.limbs[i].cmp(&rhs.limbs[i]) {
                Ordering::Less => return true,
                Ordering::Greater => return false,
                Ordering::Equal => {}
            }
        }

        false
    }

    /// Unsigned less-than-or-equal comparison.
    ///
    /// # Panics
    ///
    /// Panics if widths differ.
    pub fn ule(&self, rhs: &Self) -> bool {
        self == rhs || self.ult(rhs)
    }

    /// Unsigned greater-than comparison.
    ///
    /// # Panics
    ///
    /// Panics if widths differ.
    pub fn ugt(&self, rhs: &Self) -> bool {
        !self.ule(rhs)
    }

    /// Unsigned greater-than-or-equal comparison.
    ///
    /// # Panics
    ///
    /// Panics if widths differ.
    pub fn uge(&self, rhs: &Self) -> bool {
        !self.ult(rhs)
    }

    /// Signed less-than comparison.
    ///
    /// # Panics
    ///
    /// Panics if widths differ.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let a = ApInt::new(8, 0xff); // -1
    /// let b = ApInt::new(8, 0);    // 0
    /// assert!(a.slt(&b));
    /// ```
    pub fn slt(&self, rhs: &Self) -> bool {
        assert_eq!(self.width, rhs.width);

        match (self.is_negative(), rhs.is_negative()) {
            (true, false) => true,
            (false, true) => false,
            _ => self.ult(rhs),
        }
    }

    /// Signed less-than-or-equal comparison.
    ///
    /// # Panics
    ///
    /// Panics if widths differ.
    pub fn sle(&self, rhs: &Self) -> bool {
        self == rhs || self.slt(rhs)
    }

    /// Signed greater-than comparison.
    ///
    /// # Panics
    ///
    /// Panics if widths differ.
    pub fn sgt(&self, rhs: &Self) -> bool {
        !self.sle(rhs)
    }

    /// Signed greater-than-or-equal comparison.
    ///
    /// # Panics
    ///
    /// Panics if widths differ.
    pub fn sge(&self, rhs: &Self) -> bool {
        !self.slt(rhs)
    }

    /// Zero-extends to a new width.
    ///
    /// The value is preserved by adding zero bits in the most significant positions.
    ///
    /// # Panics
    ///
    /// Panics if `new_width < self.width`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let x = ApInt::new(8, 0xff);
    /// let y = x.zext(16);
    /// assert_eq!(y.get_limbs()[0], 0xff);
    /// assert_eq!(y.width(), 16);
    /// ```
    pub fn zext(&self, new_width: usize) -> Self {
        assert!(new_width >= self.width);

        Self::from_limbs(new_width, self.limbs.clone())
    }

    /// Truncates to a new width.
    ///
    /// The value is reduced by discarding the most significant bits.
    ///
    /// # Panics
    ///
    /// Panics if `new_width == 0` or `new_width > self.width`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let x = ApInt::new(128, 0xffff);
    /// let y = x.trunc(8);
    /// assert_eq!(y.get_limbs()[0], 0xff);
    /// assert_eq!(y.width(), 8);
    /// ```
    pub fn trunc(&self, new_width: usize) -> Self {
        assert!(new_width > 0);
        assert!(new_width <= self.width);

        let mut limbs = self.limbs.clone();
        limbs.truncate(Self::num_limbs(new_width));

        Self::from_limbs(new_width, limbs)
    }

    /// Sign-extends to a new width.
    ///
    /// The value is preserved by copying the sign bit into the new
    /// most significant positions.
    ///
    /// # Panics
    ///
    /// Panics if `new_width < self.width`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let x = ApInt::new(8, 0x80); // -128
    /// let y = x.sext(16);
    /// assert_eq!(y.get_limbs()[0], 0xff80);
    /// assert_eq!(y.width(), 16);
    /// ```
    pub fn sext(&self, new_width: usize) -> Self {
        assert!(new_width >= self.width);

        if new_width == self.width {
            return self.clone();
        }

        let mut result = self.zext(new_width);

        if self.is_negative() {
            let old_width = self.width;

            for bit in old_width..new_width {
                let limb = bit / LIMB_BITS;
                let offset = bit % LIMB_BITS;

                result.limbs[limb] |= 1u64 << offset;
            }
        }

        result
    }

    /// Converts this integer to `u8`, truncating any bits that don't fit.
    ///
    /// This is a lossy conversion that takes the least significant 8 bits
    /// of the integer. No overflow checking is performed.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let x = ApInt::new(8, 0xff);
    /// assert_eq!(x.to_u8_lossy(), 255);
    ///
    /// // Value is truncated to 8 bits
    /// let y = ApInt::new(16, 0x1234);
    /// assert_eq!(y.to_u8_lossy(), 0x34);
    ///
    /// // Negative values are interpreted as two's complement
    /// let z = ApInt::new(8, 0xff); // -1
    /// assert_eq!(z.to_u8_lossy(), 255);
    /// ```
    pub fn to_u8_lossy(&self) -> u8 {
        (self.limbs[0] & Self::mask(8)) as u8
    }

    /// Converts this integer to `u16`, truncating any bits that don't fit.
    ///
    /// This is a lossy conversion that takes the least significant 16 bits
    /// of the integer. No overflow checking is performed.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let x = ApInt::new(16, 0xffff);
    /// assert_eq!(x.to_u16_lossy(), 65535);
    ///
    /// // Value is truncated to 16 bits
    /// let y = ApInt::new(32, 0x12345678);
    /// assert_eq!(y.to_u16_lossy(), 0x5678);
    /// ```
    pub fn to_u16_lossy(&self) -> u16 {
        (self.limbs[0] & Self::mask(16)) as u16
    }

    /// Converts this integer to `u32`, truncating any bits that don't fit.
    ///
    /// This is a lossy conversion that takes the least significant 32 bits
    /// of the integer. No overflow checking is performed.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let x = ApInt::new(32, 0xffffffff);
    /// assert_eq!(x.to_u32_lossy(), 4294967295);
    ///
    /// // Value is truncated to 32 bits
    /// let y = ApInt::new(64, 0x123456789abcdef0);
    /// assert_eq!(y.to_u32_lossy(), 0x9abcdef0);
    /// ```
    pub fn to_u32_lossy(&self) -> u32 {
        (self.limbs[0] & Self::mask(32)) as u32
    }

    /// Converts this integer to `u64`, truncating any bits that don't fit.
    ///
    /// This is a lossy conversion that takes the least significant 64 bits
    /// of the integer. No overflow checking is performed.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let x = ApInt::new(64, u64::MAX as u128);
    /// assert_eq!(x.to_u64_lossy(), u64::MAX);
    ///
    /// // Value is truncated to 64 bits
    /// let y = ApInt::new(128, 0x123456789abcdef0123456789abcdef0);
    /// assert_eq!(y.to_u64_lossy(), 0x123456789abcdef0);
    /// ```
    pub fn to_u64_lossy(&self) -> u64 {
        self.limbs[0] & Self::mask(64)
    }

    /// Converts this integer to `u128`, truncating any bits that don't fit.
    ///
    /// This is a lossy conversion that takes the least significant 128 bits
    /// of the integer. No overflow checking is performed.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap::integral::ApInt;
    ///
    /// let x = ApInt::new(128, u128::MAX);
    /// assert_eq!(x.to_u128_lossy(), u128::MAX);
    ///
    /// // Value is truncated to 128 bits
    /// let y = ApInt::new(256, 0x123456789abcdef0123456789abcdef0);
    /// // Only the lower 128 bits are returned
    /// assert_eq!(y.to_u128_lossy(), 0x123456789abcdef0123456789abcdef0);
    /// ```
    pub fn to_u128_lossy(&self) -> u128 {
        let mut result = self.limbs[0] as u128;
        if self.limbs.len() > 1 {
            result |= (self.limbs[1] as u128) << 64;
        }
        result
    }
}

impl fmt::Debug for ApInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ApInt({} bits: ", self.width)?;

        for (i, limb) in self.limbs.iter().enumerate().rev() {
            if i == self.limbs.len() - 1 {
                write!(f, "{:x}", limb)?;
            } else {
                write!(f, "_{:016x}", limb)?;
            }
        }

        write!(f, ")")
    }
}

impl fmt::Display for ApInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x")?;

        for (i, limb) in self.limbs.iter().enumerate().rev() {
            if i == self.limbs.len() - 1 {
                write!(f, "{:x}", limb)?;
            } else {
                write!(f, "{:016x}", limb)?;
            }
        }

        Ok(())
    }
}

impl From<&ApInt> for u8 {
    fn from(value: &ApInt) -> Self {
        value.to_u8_lossy()
    }
}

impl From<&ApInt> for u16 {
    fn from(value: &ApInt) -> Self {
        value.to_u16_lossy()
    }
}

impl From<&ApInt> for u32 {
    fn from(value: &ApInt) -> Self {
        value.to_u32_lossy()
    }
}

impl From<&ApInt> for u64 {
    fn from(value: &ApInt) -> Self {
        value.to_u64_lossy()
    }
}

impl From<&ApInt> for u128 {
    fn from(value: &ApInt) -> Self {
        value.to_u128_lossy()
    }
}
