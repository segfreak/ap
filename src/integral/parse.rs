use crate::integral::*;

impl ApInt {
    /// Creates an `ApInt` from a string in the given radix.
    ///
    /// This method parses a string representation of an integer with the
    /// specified radix (base). It supports both positive and negative
    /// numbers, with optional `+` or `-` signs.
    ///
    /// # Supported Radices
    ///
    /// - **2**: Binary (digits `0-1`)
    /// - **8**: Octal (digits `0-7`)
    /// - **10**: Decimal (digits `0-9`)
    /// - **16**: Hexadecimal (digits `0-9`, `a-f`, `A-F`)
    /// - Other radices between 2 and 36 are supported with digits `0-9`, `a-z`, `A-Z`
    ///
    /// # Format
    ///
    /// The string can optionally start with:
    /// - `+` or `-` for sign
    /// - `0x`/`0X` for hexadecimal (overrides radix parameter)
    /// - `0o`/`0O` for octal (overrides radix parameter)
    /// - `0b`/`0B` for binary (overrides radix parameter)
    ///
    /// # Panics
    ///
    /// Panics if `width == 0` or `radix < 2` or `radix > 36`.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The string is empty
    /// - The string contains invalid digits for the given radix
    /// - The value is too large for the specified bit width
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ap::integral::ApInt;
    ///
    /// // Decimal
    /// let x = ApInt::from_str_radix(8, "255", 10).unwrap();
    /// assert_eq!(x.get_limbs()[0], 255);
    ///
    /// // Hexadecimal
    /// let x = ApInt::from_str_radix(8, "ff", 16).unwrap();
    /// assert_eq!(x.get_limbs()[0], 255);
    ///
    /// // Binary
    /// let x = ApInt::from_str_radix(8, "11111111", 2).unwrap();
    /// assert_eq!(x.get_limbs()[0], 255);
    ///
    /// // Octal
    /// let x = ApInt::from_str_radix(8, "377", 8).unwrap();
    /// assert_eq!(x.get_limbs()[0], 255);
    ///
    /// // Negative numbers
    /// let x = ApInt::from_str_radix(8, "-1", 10).unwrap();
    /// assert_eq!(x.get_limbs()[0], 0xff);
    ///
    /// // Auto-detected hex prefix overrides radix
    /// let x = ApInt::from_str_radix(16, "0xff", 10).unwrap();
    /// assert_eq!(x.get_limbs()[0], 255);
    /// ```
    pub fn from_str_radix(width: usize, s: &str, radix: u32) -> Result<Self, String> {
        assert!(width > 0, "width must be > 0");
        assert!((2..=36).contains(&radix), "radix must be between 2 and 36");

        let s = s.trim();

        if s.is_empty() {
            return Err("empty integer".into());
        }

        // Parse sign
        let (negative, s) = match s.as_bytes()[0] {
            b'-' => (true, &s[1..]),
            b'+' => (false, &s[1..]),
            _ => (false, s),
        };

        if s.is_empty() {
            return Err("expected digits after sign".into());
        }

        // Detect common prefixes
        let (actual_radix, digits) = if s.len() >= 2 && s.as_bytes()[0] == b'0' {
            match s.as_bytes()[1] {
                b'x' | b'X' => (16, &s[2..]),
                b'o' | b'O' => (8, &s[2..]),
                b'b' | b'B' => (2, &s[2..]),
                _ => (radix, s),
            }
        } else {
            (radix, s)
        };

        if digits.is_empty() {
            return Err("expected digits after prefix".into());
        }

        // Check for underscores and other separators (optional)
        let digits = Self::strip_underscores(digits);

        // Parse the number
        let limbs = Self::parse_limbs_from_digits(width, &digits, actual_radix)?;

        // Apply sign if negative
        let result = if negative {
            Self::negate_limbs(width, limbs)
        } else {
            Self::from_limbs(width, limbs)
        };

        Ok(result)
    }

    /// Helper: strip underscores from digit string (optional feature)
    fn strip_underscores(s: &str) -> String {
        s.chars().filter(|&c| c != '_').collect()
    }

    /// Helper: parse digits into limbs
    fn parse_limbs_from_digits(width: usize, digits: &str, radix: u32) -> Result<Limbs, String> {
        let num_limbs = Self::num_limbs(width);
        let mut limbs: Limbs = (0..num_limbs).map(|_| 0).collect();
        let radix_limb = radix as u64;

        // Optimization for power-of-two radices
        if radix == 2 {
            return Self::parse_binary_limbs(width, digits);
        } else if radix == 8 {
            return Self::parse_octal_limbs(width, digits);
        } else if radix == 16 {
            return Self::parse_hex_limbs(width, digits);
        }

        // General case
        for byte in digits.bytes() {
            let digit = match byte {
                b'0'..=b'9' => (byte - b'0') as u64,
                b'a'..=b'z' => (byte - b'a' + 10) as u64,
                b'A'..=b'Z' => (byte - b'A' + 10) as u64,
                _ => {
                    return Err(format!("invalid digit `{}`", byte as char));
                }
            };

            if digit >= radix_limb {
                return Err(format!(
                    "digit `{}` is invalid for base {}",
                    byte as char, radix
                ));
            }

            let mut carry = digit;
            for limb in &mut limbs {
                let result = (*limb as u128) * (radix_limb as u128) + (carry as u128);
                *limb = result as u64;
                carry = (result >> 64) as u64;
            }

            if carry > 0 {
                return Err(format!("number too large for {} bits", width));
            }
        }

        limbs::clear_high_bits(&mut limbs, width);
        Ok(limbs)
    }

    /// Optimized parser for binary numbers
    fn parse_binary_limbs(width: usize, digits: &str) -> Result<Limbs, String> {
        let num_limbs = Self::num_limbs(width);
        let mut limbs: Limbs = (0..num_limbs).map(|_| 0).collect();

        for (i, byte) in digits.bytes().enumerate() {
            match byte {
                b'0' => continue,
                b'1' => {
                    let bit_pos = digits.len() - 1 - i;
                    if bit_pos >= width {
                        return Err(format!("number too large for {} bits", width));
                    }
                    let limb_idx = bit_pos / LIMB_BITS;
                    let bit_in_limb = bit_pos % LIMB_BITS;
                    limbs[limb_idx] |= 1u64 << bit_in_limb;
                }
                _ => {
                    return Err(format!("invalid binary digit `{}`", byte as char));
                }
            }
        }

        limbs::clear_high_bits(&mut limbs, width);
        Ok(limbs)
    }

    /// Optimized parser for octal numbers
    fn parse_octal_limbs(width: usize, digits: &str) -> Result<Limbs, String> {
        let num_limbs = Self::num_limbs(width);
        let mut limbs: Limbs = (0..num_limbs).map(|_| 0).collect();

        for byte in digits.bytes() {
            let digit = match byte {
                b'0'..=b'7' => (byte - b'0') as u64,
                _ => {
                    return Err(format!("invalid octal digit `{}`", byte as char));
                }
            };

            let mut carry = digit;
            for limb in &mut limbs {
                let result = (*limb as u128) * 8 + (carry as u128);
                *limb = result as u64;
                carry = (result >> 64) as u64;
            }

            if carry > 0 {
                return Err(format!("number too large for {} bits", width));
            }
        }

        limbs::clear_high_bits(&mut limbs, width);
        Ok(limbs)
    }

    /// Optimized parser for hexadecimal numbers
    fn parse_hex_limbs(width: usize, digits: &str) -> Result<Limbs, String> {
        let num_limbs = Self::num_limbs(width);
        let mut limbs: Limbs = (0..num_limbs).map(|_| 0).collect();

        let digits_len = digits.len();
        let bits_used = digits_len * 4;

        if bits_used > width {
            // Check if the value actually exceeds the width
            let leading_hex_digits = width.div_ceil(4);
            let to_skip = digits_len.saturating_sub(leading_hex_digits);

            // Check if the skipped digits are all zero
            let has_nonzero = digits[..to_skip].bytes().any(|b| b != b'0');
            if has_nonzero {
                return Err(format!("number too large for {} bits", width));
            }
        }

        // Process hex digits from right to left for efficiency
        let start_idx = if digits_len > 0 && digits_len * 4 > width {
            let leading_digits = width.div_ceil(4);
            digits_len - leading_digits
        } else {
            0
        };

        let mut bit_pos = 0;
        for byte in digits[start_idx..].bytes() {
            let digit = match byte {
                b'0'..=b'9' => (byte - b'0') as u64,
                b'a'..=b'f' => (byte - b'a' + 10) as u64,
                b'A'..=b'F' => (byte - b'A' + 10) as u64,
                _ => {
                    return Err(format!("invalid hex digit `{}`", byte as char));
                }
            };

            let limb_idx = bit_pos / LIMB_BITS;
            let bit_offset = bit_pos % LIMB_BITS;

            if limb_idx < num_limbs {
                limbs[limb_idx] |= digit << bit_offset;
                // Handle overflow into next limb
                if bit_offset + 4 > LIMB_BITS && limb_idx + 1 < num_limbs {
                    limbs[limb_idx + 1] |= digit >> (LIMB_BITS - bit_offset);
                }
            }

            bit_pos += 4;
        }

        limbs::clear_high_bits(&mut limbs, width);
        Ok(limbs)
    }

    /// Helper: negate limbs (two's complement)
    fn negate_limbs(width: usize, mut limbs: Limbs) -> Self {
        for limb in &mut limbs {
            *limb = !*limb;
        }

        let mut carry = 1u64;
        for limb in &mut limbs {
            let (value, overflow) = limb.overflowing_add(carry);
            *limb = value;
            if !overflow {
                break;
            }
            carry = 1;
        }

        limbs::clear_high_bits(&mut limbs, width);
        Self::from_limbs(width, limbs)
    }

    /// Parses a string with auto-detected radix (supports decimal, hex, octal, binary).
    ///
    /// This is a convenience wrapper around `from_str_radix` that auto-detects
    /// the radix based on common prefixes:
    /// - `0x`/`0X` → hexadecimal (16)
    /// - `0o`/`0O` → octal (8)
    /// - `0b`/`0B` → binary (2)
    /// - Otherwise → decimal (10)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ap::integral::ApInt;
    ///
    /// let x = ApInt::parse(8, "255").unwrap();
    /// assert_eq!(x.to_u8_lossy(), 255);
    ///
    /// let x = ApInt::parse(8, "0xff").unwrap();
    /// assert_eq!(x.to_u8_lossy(), 255);
    ///
    /// let x = ApInt::parse(8, "0b11111111").unwrap();
    /// assert_eq!(x.to_u8_lossy(), 255);
    ///
    /// let x = ApInt::parse(8, "0o377").unwrap();
    /// assert_eq!(x.to_u8_lossy(), 255);
    ///
    /// let x = ApInt::parse(8, "260").unwrap();
    /// assert_eq!(x.to_u8_lossy(), 4);
    /// ```
    pub fn parse(width: usize, s: &str) -> Result<Self, String> {
        let s = s.trim();
        if s.is_empty() {
            return Err("empty integer".into());
        }

        let (negative, s) = match s.as_bytes()[0] {
            b'-' => (true, &s[1..]),
            b'+' => (false, &s[1..]),
            _ => (false, s),
        };

        if s.is_empty() {
            return Err("expected digits after sign".into());
        }

        // Detect radix from prefix
        let (radix, digits) = if s.len() >= 2 && s.as_bytes()[0] == b'0' {
            match s.as_bytes()[1] {
                b'x' | b'X' => (16, &s[2..]),
                b'o' | b'O' => (8, &s[2..]),
                b'b' | b'B' => (2, &s[2..]),
                _ => (10, s),
            }
        } else {
            (10, s)
        };

        if digits.is_empty() {
            return Err("expected digits after prefix".into());
        }

        let mut result = Self::from_str_radix(width, digits, radix)?;

        if negative {
            result = result.neg();
        }

        Ok(result)
    }
}
