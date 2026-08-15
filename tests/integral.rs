use ap::integral::*;

#[test]
fn new_zero() {
    let x = ApInt::new(8, 0);
    assert_eq!(x.width(), 8);
    assert!(x.is_zero());
    assert_eq!(x.get_limbs().len(), 1);
}

#[test]
fn new_max_value() {
    let x = ApInt::new(8, 0xff);
    assert!(!x.is_zero());
    assert_eq!(x.get_limbs()[0], 0xff);
}

#[test]
fn new_with_large_value() {
    let x = ApInt::new(128, u128::MAX);
    assert_eq!(x.get_limbs()[0], u64::MAX);
    assert_eq!(x.get_limbs()[1], u64::MAX);
    assert_eq!(x.get_limbs().len(), 2);
}

#[test]
fn zero_constructor() {
    let x = ApInt::zero(64);
    assert!(x.is_zero());
    assert_eq!(x.width(), 64);
    assert_eq!(x.get_limbs().len(), 1);
}

#[test]
fn one_constructor() {
    let x = ApInt::one(32);
    assert_eq!(x.get_limbs()[0], 1);
    assert_eq!(x.width(), 32);
}

#[test]
fn from_limbs() {
    let x = ApInt::from_limbs(64, vec![0x123456789abcdef0]);
    assert_eq!(x.get_limbs()[0], 0x123456789abcdef0);
}

#[test]
fn from_limbs_truncate() {
    let x = ApInt::from_limbs(32, vec![0xffffffffffffffff]);
    assert_eq!(x.get_limbs()[0], 0xffffffff);
    assert_eq!(x.width(), 32);
}

#[test]
fn from_limbs_resize() {
    let x = ApInt::from_limbs(128, vec![0x123]);
    assert_eq!(x.get_limbs().len(), 2);
    assert_eq!(x.get_limbs()[0], 0x123);
    assert_eq!(x.get_limbs()[1], 0);
}

#[test]
fn is_negative_positive() {
    let x = ApInt::new(8, 127);
    assert!(!x.is_negative());
}

#[test]
fn is_negative_negative() {
    let x = ApInt::new(8, 0xff);
    assert!(x.is_negative());
}

#[test]
fn is_negative_multi_limb() {
    let x = ApInt::from_limbs(128, vec![0, 0]);
    assert!(!x.is_negative());

    let y = ApInt::from_limbs(128, vec![0, u64::MAX]);
    assert!(y.is_negative());
}

#[test]
fn is_zero_true() {
    let x = ApInt::zero(64);
    assert!(x.is_zero());
}

#[test]
fn is_zero_false() {
    let x = ApInt::new(64, 1);
    assert!(!x.is_zero());
}

#[test]
fn add_multi_limb() {
    let a = ApInt::from_limbs(128, vec![u64::MAX, 0]);
    let b = ApInt::from_limbs(128, vec![1, 0]);
    let c = a.add(&b);
    assert_eq!(c.get_limbs()[0], 0);
    assert_eq!(c.get_limbs()[1], 1);
}

#[test]
fn add_with_carry() {
    let a = ApInt::from_limbs(128, vec![u64::MAX, u64::MAX]);
    let b = ApInt::from_limbs(128, vec![1, 0]);
    let c = a.add(&b);
    assert_eq!(c.get_limbs()[0], 0);
    assert_eq!(c.get_limbs()[1], 0);
}

#[test]
fn sub_multi_limb() {
    let a = ApInt::from_limbs(128, vec![0, 1]);
    let b = ApInt::from_limbs(128, vec![1, 0]);
    let c = a.sub(&b);
    assert_eq!(c.get_limbs()[0], u64::MAX);
    assert_eq!(c.get_limbs()[1], 0);
}

#[test]
fn sub_borrow_multi_limb() {
    let a = ApInt::from_limbs(128, vec![0, 0]);
    let b = ApInt::from_limbs(128, vec![1, 0]);
    let c = a.sub(&b);
    assert_eq!(c.get_limbs()[0], u64::MAX);
    assert_eq!(c.get_limbs()[1], u64::MAX);
}

#[test]
fn mul_overflow() {
    let a = ApInt::new(8, 0xff);
    let b = ApInt::new(8, 0xff);
    let c = a.mul(&b);
    assert_eq!(c.get_limbs()[0], 0x01);
}

#[test]
fn mul_zero() {
    let a = ApInt::new(64, 0x123456789abcdef);
    let b = ApInt::zero(64);
    let c = a.mul(&b);
    assert!(c.is_zero());
}

#[test]
fn mul_negative() {
    let a = ApInt::new(8, 0xff); // -1
    let b = ApInt::new(8, 0x02);
    let c = a.mul(&b);
    assert_eq!(c.get_limbs()[0], 0xfe); // -2
}

#[test]
fn neg_zero() {
    let x = ApInt::zero(8);
    let y = x.neg();
    assert!(y.is_zero());
}

#[test]
fn neg_negative() {
    let x = ApInt::new(8, 0xff); // -1
    let y = x.neg();
    assert_eq!(y.get_limbs()[0], 0x01); // 1
}

#[test]
fn neg_multi_limb() {
    let x = ApInt::from_limbs(128, vec![1, 0]);
    let y = x.neg();
    assert_eq!(y.get_limbs()[0], u64::MAX);
    assert_eq!(y.get_limbs()[1], u64::MAX);
}

#[test]
fn bitand_multi_limb() {
    let a = ApInt::from_limbs(128, vec![0x0f, 0x0f]);
    let b = ApInt::from_limbs(128, vec![0xf0, 0xf0]);
    let c = a.bitand(&b);
    assert_eq!(c.get_limbs()[0], 0);
    assert_eq!(c.get_limbs()[1], 0);
}

#[test]
fn bitor_multi_limb() {
    let a = ApInt::from_limbs(128, vec![0x0f, 0x0f]);
    let b = ApInt::from_limbs(128, vec![0xf0, 0xf0]);
    let c = a.bitor(&b);
    assert_eq!(c.get_limbs()[0], 0xff);
    assert_eq!(c.get_limbs()[1], 0xff);
}

#[test]
fn bitxor_multi_limb() {
    let a = ApInt::from_limbs(128, vec![0x0f, 0x0f]);
    let b = ApInt::from_limbs(128, vec![0xff, 0xff]);
    let c = a.bitxor(&b);
    assert_eq!(c.get_limbs()[0], 0xf0);
    assert_eq!(c.get_limbs()[1], 0xf0);
}

#[test]
fn not_multi_limb() {
    let x = ApInt::from_limbs(128, vec![0, 0]);
    let y = x.not();
    assert_eq!(y.get_limbs()[0], u64::MAX);
    assert_eq!(y.get_limbs()[1], u64::MAX);
}

#[test]
fn shl_by_zero() {
    let x = ApInt::new(64, 0x123456789abcdef);
    let y = x.shl(0);
    assert_eq!(x, y);
}

#[test]
fn shl_full() {
    let x = ApInt::new(8, 0xff);
    let y = x.shl(8);
    assert!(y.is_zero());
}

#[test]
fn shl_multi_limb() {
    let x = ApInt::from_limbs(128, vec![1, 0]);
    let y = x.shl(64);
    assert_eq!(y.get_limbs()[0], 0);
    assert_eq!(y.get_limbs()[1], 1);
}

#[test]
fn shl_with_bit_shift() {
    let x = ApInt::from_limbs(128, vec![0xffffffff, 0]);
    let y = x.shl(32);
    assert_eq!(y.get_limbs()[0], 0xffffffff00000000);
    assert_eq!(y.get_limbs()[1], 0);
}

#[test]
fn lshr_by_zero() {
    let x = ApInt::new(64, 0x123456789abcdef);
    let y = x.lshr(0);
    assert_eq!(x, y);
}

#[test]
fn lshr_full() {
    let x = ApInt::new(8, 0xff);
    let y = x.lshr(8);
    assert!(y.is_zero());
}

#[test]
fn lshr_multi_limb() {
    let x = ApInt::from_limbs(128, vec![0, 1]);
    let y = x.lshr(64);
    assert_eq!(y.get_limbs()[0], 1);
    assert_eq!(y.get_limbs()[1], 0);
}

#[test]
fn ashr_positive() {
    let x = ApInt::new(8, 0x80);
    let y = x.ashr(1);
    assert_eq!(y.get_limbs()[0], 0xc0);
}

#[test]
fn ashr_negative() {
    let x = ApInt::new(8, 0xff); // -1
    let y = x.ashr(1);
    assert_eq!(y.get_limbs()[0], 0xff); // -1
}

#[test]
fn ashr_full() {
    let x = ApInt::new(8, 0x80); // -128
    let y = x.ashr(8);
    assert_eq!(y.get_limbs()[0], 0xff); // -1
}

#[test]
fn ashr_full_positive() {
    let x = ApInt::new(8, 0x7f); // 127
    let y = x.ashr(8);
    assert!(y.is_zero());
}

#[test]
fn ult_equal() {
    let a = ApInt::new(32, 100);
    let b = ApInt::new(32, 100);
    assert!(!a.ult(&b));
}

#[test]
fn ult_less() {
    let a = ApInt::new(32, 100);
    let b = ApInt::new(32, 200);
    assert!(a.ult(&b));
}

#[test]
fn ult_greater() {
    let a = ApInt::new(32, 200);
    let b = ApInt::new(32, 100);
    assert!(!a.ult(&b));
}

#[test]
fn ule_equal() {
    let a = ApInt::new(32, 100);
    let b = ApInt::new(32, 100);
    assert!(a.ule(&b));
}

#[test]
fn ule_less() {
    let a = ApInt::new(32, 100);
    let b = ApInt::new(32, 200);
    assert!(a.ule(&b));
}

#[test]
fn ugt_equal() {
    let a = ApInt::new(32, 100);
    let b = ApInt::new(32, 100);
    assert!(!a.ugt(&b));
}

#[test]
fn ugt_greater() {
    let a = ApInt::new(32, 200);
    let b = ApInt::new(32, 100);
    assert!(a.ugt(&b));
}

#[test]
fn uge_equal() {
    let a = ApInt::new(32, 100);
    let b = ApInt::new(32, 100);
    assert!(a.uge(&b));
}

#[test]
fn uge_greater() {
    let a = ApInt::new(32, 200);
    let b = ApInt::new(32, 100);
    assert!(a.uge(&b));
}

#[test]
fn slt_equal() {
    let a = ApInt::new(8, 100);
    let b = ApInt::new(8, 100);
    assert!(!a.slt(&b));
}

#[test]
fn slt_positive_less() {
    let a = ApInt::new(8, 100); // 100
    let b = ApInt::new(8, 120); // 120
    assert!(a.slt(&b));
}

#[test]
fn slt_positive_less_overflow() {
    let a = ApInt::new(8, 100); // 100
    let b = ApInt::new(8, 200); // -56
    assert!(a.sge(&b));
}

#[test]
fn slt_negative_positive() {
    let a = ApInt::new(8, 0xff); // -1
    let b = ApInt::new(8, 0x7f); // 127
    assert!(a.slt(&b));
}

#[test]
fn slt_negative_positive_2() {
    let a = ApInt::new(8, 0x80); // -128
    let b = ApInt::new(8, 0x7f); // 127
    assert!(a.slt(&b));
    assert!(!b.slt(&a));
}

#[test]
fn slt_positive_negative() {
    let a = ApInt::new(8, 0x7f); // 127
    let b = ApInt::new(8, 0xff); // -1
    assert!(!a.slt(&b));
}

#[test]
fn slt_negative_less() {
    let a = ApInt::new(8, 0xff); // -1
    let b = ApInt::new(8, 0x80); // -128
    assert!(!a.slt(&b));
    assert!(b.slt(&a));
}

#[test]
fn slt_boundary_values() {
    let min = ApInt::new(8, 0x80); // -128
    let max = ApInt::new(8, 0x7f); // 127
    let zero = ApInt::zero(8);
    let neg_one = ApInt::new(8, 0xff); // -1

    assert!(min.slt(&max));
    assert!(min.slt(&zero));
    assert!(min.slt(&neg_one));
    assert!(neg_one.slt(&zero));
    assert!(zero.slt(&max));
    assert!(!max.slt(&zero));
    assert!(!zero.slt(&neg_one));
}

#[test]
fn slt_multi_limb() {
    let min = ApInt::from_limbs(128, vec![0, 0x8000000000000000]); // -2^127
    let max = ApInt::from_limbs(128, vec![u64::MAX, 0x7fffffffffffffff]); // 2^127-1
    let zero = ApInt::zero(128);
    let neg_one = ApInt::from_limbs(128, vec![u64::MAX, u64::MAX]); // -1

    assert!(min.slt(&max));
    assert!(min.slt(&zero));
    assert!(neg_one.slt(&zero));
    assert!(zero.slt(&max));
}

#[test]
fn sle_equal() {
    let a = ApInt::new(8, 100);
    let b = ApInt::new(8, 100);
    assert!(a.sle(&b));
}

#[test]
fn sgt_equal() {
    let a = ApInt::new(8, 100);
    let b = ApInt::new(8, 100);
    assert!(!a.sgt(&b));
}

#[test]
fn sge_equal() {
    let a = ApInt::new(8, 100);
    let b = ApInt::new(8, 100);
    assert!(a.sge(&b));
}

#[test]
fn zext_same_width() {
    let x = ApInt::new(8, 0xff);
    let y = x.zext(8);
    assert_eq!(x, y);
}

#[test]
fn zext_larger_width() {
    let x = ApInt::new(8, 0xff);
    let y = x.zext(16);
    assert_eq!(y.get_limbs()[0], 0xff);
    assert_eq!(y.get_limbs().len(), 1);
    assert_eq!(y.width(), 16);
}

#[test]
fn trunc_same_width() {
    let x = ApInt::new(8, 0xff);
    let y = x.trunc(8);
    assert_eq!(x, y);
}

#[test]
fn trunc_smaller_width() {
    let x = ApInt::new(128, 0xffff);
    let y = x.trunc(8);
    assert_eq!(y.get_limbs()[0], 0xff);
    assert_eq!(y.width(), 8);
}

#[test]
fn sext_same_width() {
    let x = ApInt::new(8, 0x80);
    let y = x.sext(8);
    assert_eq!(x, y);
}

#[test]
fn sext_positive() {
    let x = ApInt::new(8, 0x7f);
    let y = x.sext(16);
    assert_eq!(y.get_limbs()[0], 0x7f);
    assert_eq!(y.get_limbs().len(), 1);
}

#[test]
fn sext_negative() {
    let x = ApInt::new(8, 0x80); // -128
    let y = x.sext(16);
    assert_eq!(y.get_limbs()[0], 0xff80);
    assert_eq!(y.width(), 16);
}

#[test]
fn sext_multi_limb() {
    let x = ApInt::new(64, u64::MAX as u128);
    let y = x.sext(128);
    assert_eq!(y.get_limbs()[0], u64::MAX);
    assert_eq!(y.get_limbs()[1], u64::MAX);
    assert_eq!(y.get_limbs().len(), 2);
}

#[test]
fn debug_format() {
    let x = ApInt::from_limbs(128, vec![0x12345678, 0x9abcdef0]);
    let s = format!("{:?}", x);
    println!("{}", s);
    assert!(s.contains("ApInt(128 bits:"));
    assert!(s.contains("9abcdef0"));
    assert!(s.contains("12345678"));
}

#[test]
fn display_format() {
    let x = ApInt::from_limbs(128, vec![0x12345678, 0x9abcdef0]);
    let s = format!("{}", x);
    assert!(s.starts_with("0x"));
    assert!(s.contains("9abcdef0"));
    assert!(s.contains("12345678"));
}

#[test]
fn add_assign_operator() {
    let mut a = ApInt::new(32, 100);
    let b = ApInt::new(32, 50);
    a += b;
    assert_eq!(a.get_limbs()[0], 150);
}

#[test]
fn sub_assign_operator() {
    let mut a = ApInt::new(32, 100);
    let b = ApInt::new(32, 50);
    a -= b;
    assert_eq!(a.get_limbs()[0], 50);
}

#[test]
fn mul_assign_operator() {
    let mut a = ApInt::new(32, 25);
    let b = ApInt::new(32, 4);
    a *= b;
    assert_eq!(a.get_limbs()[0], 100);
}

#[test]
fn add_reference_operators() {
    let a = ApInt::new(32, 100);
    let b = ApInt::new(32, 50);
    let c = &a + &b;
    assert_eq!(c.get_limbs()[0], 150);
}

#[test]
fn sub_reference_operators() {
    let a = ApInt::new(32, 100);
    let b = ApInt::new(32, 50);
    let c = &a - &b;
    assert_eq!(c.get_limbs()[0], 50);
}

#[test]
fn mul_reference_operators() {
    let a = ApInt::new(32, 25);
    let b = ApInt::new(32, 4);
    let c = &a * &b;
    assert_eq!(c.get_limbs()[0], 100);
}

#[test]
fn bitwise_operators() {
    let a = ApInt::new(8, 0x0f);
    let b = ApInt::new(8, 0xf0);
    assert_eq!((a.clone() & b.clone()).get_limbs()[0], 0);
    assert_eq!((a.clone() | b.clone()).get_limbs()[0], 0xff);
    assert_eq!((a.clone() ^ b.clone()).get_limbs()[0], 0xff);
    assert_eq!((!a).get_limbs()[0], 0xf0);
}

#[test]
fn shift_operators() {
    let a = ApInt::new(16, 0b1101);
    assert_eq!((a.clone() << 2).get_limbs()[0], 0b110100);
    assert_eq!((a >> 2).get_limbs()[0], 0b11);
}

#[test]
fn neg_operator() {
    let a = ApInt::new(8, 5);
    let b = -a;
    assert_eq!(b.get_limbs()[0], 251);
}

#[test]
#[should_panic]
fn new_with_zero_width() {
    ApInt::new(0, 0);
}

#[test]
#[should_panic]
fn zero_with_zero_width() {
    ApInt::zero(0);
}

#[test]
#[should_panic]
fn add_different_widths() {
    let a = ApInt::new(32, 100);
    let b = ApInt::new(64, 100);
    a.add(&b);
}

#[test]
#[should_panic]
fn sub_different_widths() {
    let a = ApInt::new(32, 100);
    let b = ApInt::new(64, 100);
    a.sub(&b);
}

#[test]
#[should_panic]
fn mul_different_widths() {
    let a = ApInt::new(32, 100);
    let b = ApInt::new(64, 100);
    a.mul(&b);
}

#[test]
#[should_panic]
fn bitand_different_widths() {
    let a = ApInt::new(32, 100);
    let b = ApInt::new(64, 100);
    a.bitand(&b);
}

#[test]
#[should_panic]
fn zext_smaller_width() {
    let x = ApInt::new(32, 100);
    x.zext(16);
}

#[test]
#[should_panic]
fn trunc_zero_width() {
    let x = ApInt::new(32, 100);
    x.trunc(0);
}

#[test]
#[should_panic]
fn trunc_larger_width() {
    let x = ApInt::new(32, 100);
    x.trunc(64);
}

#[test]
#[should_panic]
fn sext_smaller_width() {
    let x = ApInt::new(32, 100);
    x.sext(16);
}

#[test]
fn sle_tests() {
    let a = ApInt::new(16, 100);
    let b = ApInt::new(16, 100);
    let c = ApInt::new(16, 200);
    let neg = ApInt::new(16, 0xffff); // -1

    assert!(a.sle(&b)); // equal
    assert!(a.sle(&c)); // less
    assert!(!c.sle(&a)); // greater
    assert!(neg.sle(&a)); // negative < positive
    assert!(neg.sle(&neg)); // equal
}

#[test]
fn sgt_tests() {
    let a = ApInt::new(16, 100);
    let b = ApInt::new(16, 100);
    let c = ApInt::new(16, 200);
    let neg = ApInt::new(16, 0xffff); // -1

    assert!(!a.sgt(&b)); // equal
    assert!(c.sgt(&a)); // greater
    assert!(!a.sgt(&c)); // less
    assert!(a.sgt(&neg)); // positive > negative
    assert!(!neg.sgt(&a)); // negative < positive
}

#[test]
fn sge_tests() {
    let a = ApInt::new(16, 100);
    let b = ApInt::new(16, 100);
    let c = ApInt::new(16, 200);
    let neg = ApInt::new(16, 0xffff); // -1

    assert!(a.sge(&b)); // equal
    assert!(c.sge(&a)); // greater
    assert!(!a.sge(&c)); // less
    assert!(a.sge(&neg)); // positive > negative
    assert!(!neg.sge(&a)); // negative < positive
}

#[test]
fn ult_multi_limb() {
    let a = ApInt::from_limbs(128, vec![u64::MAX, 0]);
    let b = ApInt::from_limbs(128, vec![0, 1]);
    let c = ApInt::from_limbs(128, vec![u64::MAX, u64::MAX]);

    assert!(a.ult(&b)); // 2^64-1 < 2^64
    assert!(b.ult(&c)); // 2^64 < 2^128-1
    assert!(a.ult(&c)); // 2^64-1 < 2^128-1
    assert!(!b.ult(&a)); // 2^64 > 2^64-1
}

#[test]
fn ult_boundary() {
    let zero = ApInt::zero(64);
    let one = ApInt::one(64);
    let max = ApInt::from_limbs(64, vec![u64::MAX]);

    assert!(zero.ult(&one));
    assert!(one.ult(&max));
    assert!(!max.ult(&one));
    assert!(!zero.ult(&zero));
}

#[test]
fn ule_tests() {
    let a = ApInt::new(64, 100);
    let b = ApInt::new(64, 100);
    let c = ApInt::new(64, 200);

    assert!(a.ule(&b)); // equal
    assert!(a.ule(&c)); // less
    assert!(!c.ule(&a)); // greater
}

#[test]
fn ugt_tests() {
    let a = ApInt::new(64, 100);
    let b = ApInt::new(64, 100);
    let c = ApInt::new(64, 200);

    assert!(!a.ugt(&b)); // equal
    assert!(c.ugt(&a)); // greater
    assert!(!a.ugt(&c)); // less
}

#[test]
fn uge_tests() {
    let a = ApInt::new(64, 100);
    let b = ApInt::new(64, 100);
    let c = ApInt::new(64, 200);

    assert!(a.uge(&b)); // equal
    assert!(c.uge(&a)); // greater
    assert!(!a.uge(&c)); // less
}

#[test]
fn add_overflow_detection() {
    let a = ApInt::new(8, 200);
    let b = ApInt::new(8, 100);
    let c = a.add(&b);
    assert_eq!(c.get_limbs()[0], 44); // 200 + 100 = 300 mod 256 = 44
    assert!(!c.is_negative());
}

#[test]
fn add_signed_overflow() {
    let a = ApInt::new(8, 0x70); // 112
    let b = ApInt::new(8, 0x20); // 32
    let c = a.add(&b);
    assert_eq!(c.get_limbs()[0], 0x90); // 144, interprets as -112
    assert!(c.is_negative());
}

#[test]
fn sub_underflow_detection() {
    let a = ApInt::new(8, 0);
    let b = ApInt::new(8, 1);
    let c = a.sub(&b);
    assert_eq!(c.get_limbs()[0], 0xff); // -1
    assert!(c.is_negative());
}

#[test]
fn sub_underflow_multi_limb() {
    let a = ApInt::from_limbs(128, vec![0, 0]);
    let b = ApInt::from_limbs(128, vec![1, 0]);
    let c = a.sub(&b);
    assert_eq!(c.get_limbs()[0], u64::MAX);
    assert_eq!(c.get_limbs()[1], u64::MAX);
    assert!(c.is_negative());
}

#[test]
fn mul_by_one() {
    let a = ApInt::new(64, 0x123456789abcdef);
    let one = ApInt::one(64);
    let c = a.mul(&one);
    assert_eq!(a, c);
}

#[test]
fn mul_by_zero() {
    let a = ApInt::new(64, 0x123456789abcdef);
    let zero = ApInt::zero(64);
    let c = a.mul(&zero);
    assert!(c.is_zero());
}

#[test]
fn mul_max_values() {
    let a = ApInt::new(64, u64::MAX.into());
    let b = ApInt::new(64, u64::MAX.into());
    let c = a.mul(&b);
    assert_eq!(c.get_limbs()[0], 1); // (2^64-1)^2 mod 2^64 = 1
}

#[test]
fn shl_boundary() {
    let a = ApInt::new(8, 1);
    let b = a.shl(7);
    assert_eq!(b.get_limbs()[0], 0x80);
    assert!(b.is_negative());

    let c = a.shl(8);
    assert!(c.is_zero());
}

#[test]
fn shl_all_ones() {
    let a = ApInt::new(8, 0xff);
    let b = a.shl(1);
    assert_eq!(b.get_limbs()[0], 0xfe);
    let c = a.shl(8);
    assert!(c.is_zero());
}

#[test]
fn lshr_boundary() {
    let a = ApInt::new(8, 0x80);
    let b = a.lshr(7);
    assert_eq!(b.get_limbs()[0], 1);
    let c = a.lshr(8);
    assert!(c.is_zero());
}

#[test]
fn lshr_all_ones() {
    let a = ApInt::new(8, 0xff);
    let b = a.lshr(1);
    assert_eq!(b.get_limbs()[0], 0x7f);
    let c = a.lshr(8);
    assert!(c.is_zero());
}

#[test]
fn ashr_boundary() {
    let a = ApInt::new(8, 0x80); // -128
    let b = a.ashr(7);
    assert_eq!(b.get_limbs()[0], 0xff); // -1
    let c = a.ashr(8);
    assert_eq!(c.get_limbs()[0], 0xff); // -1
}

#[test]
fn ashr_alternating_pattern() {
    let a = ApInt::new(8, 0xaa); // 10101010
    let b = a.ashr(1);
    assert_eq!(b.get_limbs()[0], 0xd5); // 11010101
    let c = a.ashr(2);
    assert_eq!(c.get_limbs()[0], 0xea); // 11101010
}

#[test]
fn sext_from_different_widths() {
    // Из 8 в 16
    let x8 = ApInt::new(8, 0xff);
    let x16 = x8.sext(16);
    assert_eq!(x16.get_limbs()[0], 0xffff);
    assert_eq!(x16.width(), 16);

    // Из 8 в 32
    let x32 = x8.sext(32);
    assert_eq!(x32.get_limbs()[0], 0xffffffff);
    assert_eq!(x32.width(), 32);

    // Из 16 в 128
    let x16_2 = ApInt::new(16, 0x8000);
    let x128 = x16_2.sext(128);
    assert_eq!(x128.get_limbs()[0], 0xffffffffffff8000);
    assert_eq!(x128.get_limbs()[1], u64::MAX);
}

#[test]
fn zext_from_different_widths() {
    let x8 = ApInt::new(8, 0xff);
    let x16 = x8.zext(16);
    assert_eq!(x16.get_limbs()[0], 0xff);
    assert_eq!(x16.width(), 16);

    let x32 = x8.zext(32);
    assert_eq!(x32.get_limbs()[0], 0xff);
    assert_eq!(x32.width(), 32);
}

#[test]
fn trunc_multiple_times() {
    let x = ApInt::new(128, 0x123456789abcdef);
    let x64 = x.trunc(64);
    assert_eq!(x64.get_limbs()[0], 0x123456789abcdef);
    assert_eq!(x64.width(), 64);

    let x32 = x.trunc(32);
    assert_eq!(x32.get_limbs()[0], 0x89abcdef);
    assert_eq!(x32.width(), 32);

    let x = ApInt::new(128, 0xffffffffffffffff);
    let x32 = x.trunc(32);
    assert_eq!(x32.get_limbs()[0], 0xffffffff);

    let x = ApInt::new(128, 0x123456789abcdef);
    let x8 = x.trunc(8);
    assert_eq!(x8.get_limbs()[0], 0xef);
}

#[test]
fn zext_then_trunc() {
    let x = ApInt::new(8, 0xff);
    let x16 = x.zext(16);
    let x8_2 = x16.trunc(8);
    assert_eq!(x, x8_2);
}

#[test]
fn sext_then_trunc() {
    let x = ApInt::new(8, 0x80);
    let x16 = x.sext(16);
    let x8_2 = x16.trunc(8);
    assert_eq!(x, x8_2);
}

#[test]
fn not_twice() {
    let x = ApInt::new(8, 0x55);
    let y = x.not().not();
    assert_eq!(x, y);
}

#[test]
fn xor_with_self() {
    let x = ApInt::new(8, 0x55);
    let y = x.bitxor(&x);
    assert!(y.is_zero());
}

#[test]
fn and_with_self() {
    let x = ApInt::new(8, 0x55);
    let y = x.bitand(&x);
    assert_eq!(x, y);
}

#[test]
fn or_with_self() {
    let x = ApInt::new(8, 0x55);
    let y = x.bitor(&x);
    assert_eq!(x, y);
}

#[test]
fn de_morgan_laws() {
    let a = ApInt::new(8, 0x55);
    let b = ApInt::new(8, 0xaa);

    // !(a & b) == !a | !b
    let lhs = a.bitand(&b).not();
    let rhs = a.not().bitor(&b.not());
    assert_eq!(lhs, rhs);

    // !(a | b) == !a & !b
    let lhs = a.bitor(&b).not();
    let rhs = a.not().bitand(&b.not());
    assert_eq!(lhs, rhs);
}

#[test]
fn negation_properties() {
    let a = ApInt::new(8, 42);
    let b = a.neg();
    let c = b.neg();
    assert_eq!(a, c); // -(-a) == a

    let zero = ApInt::zero(8);
    let neg_zero = zero.neg();
    assert!(neg_zero.is_zero()); // -0 == 0
}

#[test]
fn add_sub_inverse() {
    let a = ApInt::new(8, 42);
    let b = a.neg();
    let c = a.add(&b);
    assert!(c.is_zero()); // a + (-a) == 0

    let d = a.sub(&a);
    assert!(d.is_zero()); // a - a == 0
}

#[test]
fn distributivity() {
    let a = ApInt::new(8, 3);
    let b = ApInt::new(8, 4);
    let c = ApInt::new(8, 5);

    // a * (b + c) == a * b + a * c
    let lhs = a.mul(&b.add(&c));
    let rhs = a.mul(&b).add(&a.mul(&c));
    assert_eq!(lhs, rhs);
}

#[test]
fn associativity() {
    let a = ApInt::new(8, 3);
    let b = ApInt::new(8, 4);
    let c = ApInt::new(8, 5);

    // (a + b) + c == a + (b + c)
    let lhs = a.add(&b).add(&c);
    let rhs = a.add(&b.add(&c));
    assert_eq!(lhs, rhs);

    // (a * b) * c == a * (b * c)
    let lhs = a.mul(&b).mul(&c);
    let rhs = a.mul(&b.mul(&c));
    assert_eq!(lhs, rhs);
}

#[test]
fn commutativity() {
    let a = ApInt::new(8, 3);
    let b = ApInt::new(8, 4);

    // a + b == b + a
    assert_eq!(a.add(&b), b.add(&a));

    // a * b == b * a
    assert_eq!(a.mul(&b), b.mul(&a));
}

#[test]
fn identity_elements() {
    let a = ApInt::new(8, 42);
    let zero = ApInt::zero(8);
    let one = ApInt::one(8);

    // a + 0 == a
    assert_eq!(a.add(&zero), a);

    // a * 1 == a
    assert_eq!(a.mul(&one), a);

    // a * 0 == 0
    assert_eq!(a.mul(&zero), zero);
}

#[test]
fn clone_equality() {
    let a = ApInt::new(64, 0x123456789abcdef);
    let b = a.clone();
    assert_eq!(a, b);
    assert_eq!(a.width(), b.width());
    assert_eq!(a.get_limbs(), b.get_limbs());
}

#[test]
fn width_calculation() {
    assert_eq!(ApInt::num_limbs(1), 1);
    assert_eq!(ApInt::num_limbs(64), 1);
    assert_eq!(ApInt::num_limbs(65), 2);
    assert_eq!(ApInt::num_limbs(128), 2);
    assert_eq!(ApInt::num_limbs(129), 3);
}

#[test]
fn mask_function() {
    assert_eq!(ApInt::mask(0), 0);
    assert_eq!(ApInt::mask(1), 1);
    assert_eq!(ApInt::mask(8), 0xff);
    assert_eq!(ApInt::mask(16), 0xffff);
    assert_eq!(ApInt::mask(63), 0x7fffffffffffffff);
    assert_eq!(ApInt::mask(64), u64::MAX);
}

#[test]
fn clear_unused_bits() {
    let x = ApInt::from_limbs(10, vec![u64::MAX]);
    assert_eq!(x.get_limbs()[0], 0x3ff); // Только 10 бит должны быть установлены
    assert_eq!(x.width(), 10);
}

#[test]
fn from_limbs_clear_unused() {
    // 10-битное число с установленными лишними битами
    let x = ApInt::from_limbs(10, vec![0xffff]);
    assert_eq!(x.get_limbs()[0], 0x3ff); // Должно быть обрезано до 10 бит
}

#[test]
fn add_assign_reference() {
    let mut a = ApInt::new(32, 100);
    let b = ApInt::new(32, 50);
    a.add_assign(&b);
    assert_eq!(a.get_limbs()[0], 150);
}

#[test]
fn sub_assign_reference() {
    let mut a = ApInt::new(32, 100);
    let b = ApInt::new(32, 50);
    a.sub_assign(&b);
    assert_eq!(a.get_limbs()[0], 50);
}

#[test]
fn mul_assign_reference() {
    let mut a = ApInt::new(32, 25);
    let b = ApInt::new(32, 4);
    a.mul_assign(&b);
    assert_eq!(a.get_limbs()[0], 100);
}

#[test]
fn operator_precedence() {
    let a = ApInt::new(8, 2);
    let b = ApInt::new(8, 3);
    let c = ApInt::new(8, 4);

    // a + b * c
    let result = a.clone() + b.clone() * c.clone();
    let expected = a.add(&b.mul(&c));
    assert_eq!(result, expected);
}

#[test]
fn display_zero() {
    let x = ApInt::zero(64);
    assert_eq!(format!("{}", x), "0x0");
}

#[test]
fn display_multi_limb() {
    let x = ApInt::from_limbs(128, vec![0x12345678, 0x9abcdef0]);
    let s = format!("{}", x);
    assert!(s.contains("9abcdef0"));
    assert!(s.contains("12345678"));
}

#[test]
fn large_width_operations() {
    let width = 1024;
    let a = ApInt::new(width, 0x123456789abcdef);
    let b = ApInt::new(width, 0xfedcba987654321);
    let c = a.add(&b);
    assert_eq!(c.width(), width);
    assert!(c.get_limbs().len() > 1);
}

#[test]
fn small_width_operations() {
    let width = 1;
    let a = ApInt::new(width, 1);
    let b = ApInt::new(width, 1);
    let c = a.add(&b);
    assert_eq!(c.get_limbs()[0], 0); // 1 + 1 = 0 (переполнение в 1 бите)
    assert_eq!(c.width(), 1);
}

#[test]
fn all_bits_set() {
    let x = ApInt::new(8, 0xff);
    let y = ApInt::new(8, 0xff);
    let z = x.bitand(&y);
    assert_eq!(z.get_limbs()[0], 0xff);
}

#[test]
fn no_bits_set() {
    let x = ApInt::zero(8);
    let y = ApInt::new(8, 0xff);
    let z = x.bitand(&y);
    assert!(z.is_zero());
}

#[test]
fn shift_combinations() {
    let x = ApInt::new(8, 0x55);
    let y = x.shl(2);
    let z = y.lshr(2);
    assert_ne!(x, z); // 0x55 != 0x15

    let x2 = ApInt::new(16, 0x5555);
    let y2 = x2.shl(2);
    let z2 = y2.lshr(2);
    assert_ne!(x2, z2); // 0x5555 != 0x1555

    let x3 = ApInt::new(16, 0x1555);
    let y3 = x3.shl(2);
    let z3 = y3.lshr(2);
    assert_eq!(x3, z3); // 0x1555 << 2 >> 2 = 0x1555

    let x4 = ApInt::new(8, 0b00010101); // 0x15
    let y4 = x4.shl(2);
    let z4 = y4.lshr(2);
    assert_eq!(x4, z4); // 0x15 << 2 >> 2 = 0x15

    let x5 = ApInt::new(8, 0b10101010); // 0xAA
    let y5 = x5.lshr(2);
    let z5 = y5.shl(2);
    assert_ne!(x5, z5);
}

#[test]
fn mul_128_edge_cases() {
    let a = ApInt::from_limbs(128, vec![u64::MAX, u64::MAX]);
    let b = ApInt::from_limbs(128, vec![1, 0]);
    let c = a.mul(&b);
    assert_eq!(c.get_limbs()[0], u64::MAX);
    assert_eq!(c.get_limbs()[1], u64::MAX);

    let d = ApInt::from_limbs(128, vec![0, u64::MAX]);
    let e = ApInt::from_limbs(128, vec![2, 0]);
    let f = d.mul(&e);
    assert_eq!(f.get_limbs()[0], 0);
    assert_eq!(f.get_limbs()[1], u64::MAX - 1);
}

#[test]
fn string_representation_roundtrip() {
    let x = ApInt::new(64, 0x123456789abcdef);
    let s = format!("{:?}", x);
    assert!(s.starts_with("ApInt(64 bits: "));
    assert!(s.ends_with(")"));
}

#[test]
fn get_limbs_returns_correct_slice() {
    let x = ApInt::from_limbs(128, vec![0x12345678, 0x9abcdef0]);
    let limbs = x.get_limbs();
    assert_eq!(limbs.len(), 2);
    assert_eq!(limbs[0], 0x12345678);
    assert_eq!(limbs[1], 0x9abcdef0);
}

#[test]
fn get_limbs() {
    let x = ApInt::from_limbs(64, vec![0x123456789abcdef]);
    let limbs = x.get_limbs();
    assert_eq!(limbs[0], 0x123456789abcdef);
}

#[test]
fn from_limbs_with_extra_limbs() {
    let x = ApInt::from_limbs(64, vec![0x123, 0x456, 0x789]);
    assert_eq!(x.get_limbs().len(), 1);
    assert_eq!(x.get_limbs()[0], 0x123);
}

#[test]
fn from_limbs_with_empty_vec() {
    let x = ApInt::from_limbs(64, vec![]);
    assert_eq!(x.get_limbs().len(), 1);
    assert_eq!(x.get_limbs()[0], 0);
    assert!(x.is_zero());
}

#[test]
fn new_non_byte_aligned() {
    let x = ApInt::new(7, 0b1010101);
    assert_eq!(x.get_limbs()[0], 0b1010101);
    assert_eq!(x.width(), 7);
    assert!(x.is_negative());
}

#[test]
fn add_non_byte_aligned() {
    let a = ApInt::new(7, 0b1010101); // 85
    let b = ApInt::new(7, 0b0101010); // 42
    let c = a.add(&b);
    assert_eq!(c.get_limbs()[0], 0b1111111); // 127
}

#[test]
fn add_overflow_non_byte_aligned() {
    let a = ApInt::new(7, 0b1111111); // 127
    let b = ApInt::new(7, 0b0000001); // 1
    let c = a.add(&b);
    assert_eq!(c.get_limbs()[0], 0);
    assert!(c.is_zero());
}

#[test]
fn add_non_byte_aligned_overflow_to_negative() {
    // 7-битное: 63 + 1 = 64, но в 7 битах это -64
    let a = ApInt::new(7, 0b0111111); // 63
    let b = ApInt::new(7, 0b0000001); // 1
    let c = a.add(&b);
    assert_eq!(c.get_limbs()[0], 0b1000000); // -64
    assert!(c.is_negative());
}

#[test]
fn slt_non_byte_aligned() {
    let a = ApInt::new(7, 0b0111111); // 63
    let b = ApInt::new(7, 0b1000000); // -64
    assert!(!a.slt(&b)); // 63 < -64 ? false
    assert!(b.slt(&a)); // -64 < 63 ? true
}

#[test]
fn ult_non_byte_aligned() {
    let a = ApInt::new(7, 0b0111111); // 63
    let b = ApInt::new(7, 0b1000000); // 64 (unsigned)
    assert!(a.ult(&b)); // 63 < 64 ? true
}

#[test]
fn shl_non_byte_aligned() {
    let x = ApInt::new(7, 0b1010101);
    let y = x.shl(3);
    assert_eq!(y.get_limbs()[0], 0b0101000); // Сдвиг и обрезание до 7 бит
    assert_eq!(y.width(), 7);
}

#[test]
fn sext_non_byte_aligned() {
    let x = ApInt::new(7, 0b1010101);
    let y = x.sext(16);
    assert_eq!(y.get_limbs()[0], 0b1111111111010101); // 0xFFD5
    assert_eq!(y.width(), 16);
    assert!(y.is_negative());
}

#[test]
fn equality_different_limbs_same_value() {
    let a = ApInt::from_limbs(64, vec![0x123]);
    let b = ApInt::from_limbs(64, vec![0x123, 0]);
    assert_eq!(a, b);
}

#[test]
fn equality_different_width_same_value() {
    let a = ApInt::new(8, 0xff);
    let b = ApInt::new(16, 0xff);
    assert_ne!(a, b);
}

#[test]
fn add_large_numbers() {
    let a = ApInt::from_limbs(256, vec![u64::MAX, u64::MAX, 0, 0]);
    let b = ApInt::from_limbs(256, vec![1, 0, 0, 0]);
    let c = a.add(&b);
    assert_eq!(c.get_limbs()[0], 0);
    assert_eq!(c.get_limbs()[1], 0);
    assert_eq!(c.get_limbs()[2], 1);
    assert_eq!(c.get_limbs()[3], 0);
}

#[test]
fn mul_large_numbers() {
    let a = ApInt::from_limbs(128, vec![u64::MAX, u64::MAX]);
    let b = ApInt::from_limbs(128, vec![u64::MAX, u64::MAX]);
    let c = a.mul(&b);
    // (2^128-1)^2 mod 2^128 = 1
    assert_eq!(c.get_limbs()[0], 1);
    assert_eq!(c.get_limbs()[1], 0);
}

#[test]
fn shl_exact_limb_boundary() {
    let x = ApInt::from_limbs(128, vec![1, 0]);
    let y = x.shl(64);
    assert_eq!(y.get_limbs()[0], 0);
    assert_eq!(y.get_limbs()[1], 1);
}

#[test]
fn shl_exact_width() {
    let x = ApInt::new(8, 0xff);
    let y = x.shl(8);
    assert!(y.is_zero());
}

#[test]
fn lshr_exact_limb_boundary() {
    let x = ApInt::from_limbs(128, vec![0, 1]);
    let y = x.lshr(64);
    assert_eq!(y.get_limbs()[0], 1);
    assert_eq!(y.get_limbs()[1], 0);
}

#[test]
fn add_then_mul() {
    let a = ApInt::new(8, 2);
    let b = ApInt::new(8, 3);
    let c = ApInt::new(8, 4);
    let result = a.add(&b).mul(&c);
    assert_eq!(result.get_limbs()[0], 20); // (2+3)*4 = 20
}

#[test]
fn sub_then_add() {
    let a = ApInt::new(8, 10);
    let b = ApInt::new(8, 3);
    let c = ApInt::new(8, 5);
    let result = a.sub(&b).add(&c);
    assert_eq!(result.get_limbs()[0], 12); // (10-3)+5 = 12
}

#[test]
fn new_with_max_128() {
    let x = ApInt::new(128, u128::MAX);
    assert_eq!(x.get_limbs()[0], u64::MAX);
    assert_eq!(x.get_limbs()[1], u64::MAX);
}

#[test]
fn new_with_max_65() {
    let x = ApInt::new(65, u128::MAX);
    // 65 бит, значит 2 лимба, но только младшие 65 бит установлены
    assert_eq!(x.get_limbs().len(), 2);
    assert_eq!(x.get_limbs()[0], u64::MAX);
    assert_eq!(x.get_limbs()[1], 0x1); // Только 1 бит в старшем лимбе
}

#[test]
#[should_panic(expected = "width must be > 0")]
fn one_with_zero_width() {
    ApInt::one(0);
}

#[test]
#[should_panic(expected = "width must be > 0")]
fn from_limbs_with_zero_width() {
    ApInt::from_limbs(0, vec![1, 2, 3]);
}

#[test]
#[should_panic]
fn add_assign_different_widths() {
    let mut a = ApInt::new(32, 100);
    let b = ApInt::new(64, 100);
    a.add_assign(&b);
}

#[test]
#[should_panic]
fn sub_assign_different_widths() {
    let mut a = ApInt::new(32, 100);
    let b = ApInt::new(64, 100);
    a.sub_assign(&b);
}

#[test]
fn large_number_of_limbs() {
    let width = 4096; // 64 лимба
    let a = ApInt::new(width, 0x123456789abcdef);
    let b = ApInt::new(width, 0xfedcba987654321);
    let c = a.add(&b);
    assert_eq!(c.width(), width);
    assert_eq!(c.get_limbs().len(), 64);
}

#[test]
fn clone_independence() {
    let a = ApInt::new(8, 0x55);
    let mut b = a.clone();
    b = b.add(&ApInt::new(8, 0x01));
    assert_ne!(a, b);
    assert_eq!(a.get_limbs()[0], 0x55);
    assert_eq!(b.get_limbs()[0], 0x56);
}

#[test]
fn udivrem_simple() {
    let a = ApInt::new(32, 100);
    let b = ApInt::new(32, 3);
    let (q, r) = a.udivrem(&b);
    assert_eq!(q.get_limbs()[0], 33);
    assert_eq!(r.get_limbs()[0], 1);
}

#[test]
fn udivrem_exact() {
    let a = ApInt::new(32, 100);
    let b = ApInt::new(32, 5);
    let (q, r) = a.udivrem(&b);
    assert_eq!(q.get_limbs()[0], 20);
    assert_eq!(r.get_limbs()[0], 0);
}

#[test]
fn udivrem_dividend_smaller() {
    let a = ApInt::new(32, 3);
    let b = ApInt::new(32, 100);
    let (q, r) = a.udivrem(&b);
    assert_eq!(q.get_limbs()[0], 0);
    assert_eq!(r.get_limbs()[0], 3);
}

#[test]
fn udivrem_dividend_zero() {
    let a = ApInt::zero(32);
    let b = ApInt::new(32, 100);
    let (q, r) = a.udivrem(&b);
    assert!(q.is_zero());
    assert!(r.is_zero());
}

#[test]
fn udivrem_by_one() {
    let a = ApInt::new(32, 12345);
    let b = ApInt::one(32);
    let (q, r) = a.udivrem(&b);
    assert_eq!(q.get_limbs()[0], 12345);
    assert_eq!(r.get_limbs()[0], 0);
}

#[test]
fn udivrem_max_values() {
    let a = ApInt::new(64, u64::MAX.into());
    let b = ApInt::new(64, u64::MAX.into());
    let (q, r) = a.udivrem(&b);
    assert_eq!(q.get_limbs()[0], 1);
    assert_eq!(r.get_limbs()[0], 0);
}

#[test]
fn udivrem_max_divisor() {
    let a = ApInt::new(64, u64::MAX.into());
    let b = ApInt::new(64, (u64::MAX - 1).into());
    let (q, r) = a.udivrem(&b);
    assert_eq!(q.get_limbs()[0], 1);
    assert_eq!(r.get_limbs()[0], 1);
}

#[test]
fn udivrem_large_remainder() {
    let a = ApInt::new(64, 1000);
    let b = ApInt::new(64, 333);
    let (q, r) = a.udivrem(&b);
    assert_eq!(q.get_limbs()[0], 3);
    assert_eq!(r.get_limbs()[0], 1);
}

#[test]
fn udivrem_multi_limb_simple() {
    let a = ApInt::from_limbs(128, vec![0, 2]); // 2^65
    let b = ApInt::new(128, 3);
    let (q, r) = a.udivrem(&b);
    // 2^65 / 3 = 12297829382473034410 remainder 2
    assert_eq!(q.get_limbs()[0], 12297829382473034410);
    assert_eq!(q.get_limbs()[1], 0);
    assert_eq!(r.get_limbs()[0], 2);
    assert_eq!(r.get_limbs()[1], 0);
}

#[test]
fn udivrem_multi_limb_large() {
    let a = ApInt::from_limbs(128, vec![u64::MAX, u64::MAX]);
    let b = ApInt::from_limbs(128, vec![u64::MAX, 0]);
    let (q, r) = a.udivrem(&b);
    // (2^128 - 1) / (2^64 - 1) = 2^64 + 1
    assert_eq!(q.get_limbs()[0], 1);
    assert_eq!(q.get_limbs()[1], 1);
    assert_eq!(r.get_limbs()[0], 0);
    assert_eq!(r.get_limbs()[1], 0);
}

#[test]
fn udivrem_multi_limb_complex() {
    let a = ApInt::from_limbs(128, vec![0x123456789abcdef, 0xfedcba987654321]);
    let b = ApInt::from_limbs(128, vec![0x10000, 0]);
    let (q, r) = a.udivrem(&b);
    // This is a complex division, we just verify it's correct
    let product = q.mul(&b).add(&r);
    assert_eq!(product, a);
    assert!(r.ult(&b));
}

#[test]
fn udivrem_non_power_of_two() {
    let a = ApInt::new(64, 0xffff_ffff_ffff_ffff);
    let b = ApInt::new(64, 0x1234_5678_9abc_def0);
    let (q, r) = a.udivrem(&b);
    let product = q.mul(&b).add(&r);
    assert_eq!(product, a);
    assert!(r.ult(&b));
}

#[test]
#[should_panic(expected = "division by zero")]
fn udivrem_by_zero() {
    let a = ApInt::new(32, 100);
    let b = ApInt::zero(32);
    a.udivrem(&b);
}

#[test]
#[should_panic]
fn udivrem_different_widths() {
    let a = ApInt::new(32, 100);
    let b = ApInt::new(64, 3);
    a.udivrem(&b);
}

#[test]
fn sdivrem_both_positive() {
    let a = ApInt::new(8, 100);
    let b = ApInt::new(8, 3);
    let (q, r) = a.sdivrem(&b);
    assert_eq!(q.get_limbs()[0], 33);
    assert_eq!(r.get_limbs()[0], 1);
}

#[test]
fn sdivrem_dividend_negative() {
    let a = ApInt::new(8, 0x80); // -128
    let b = ApInt::new(8, 2);
    let (q, r) = a.sdivrem(&b);
    assert_eq!(q.get_limbs()[0], 0xc0); // -64
    assert_eq!(r.get_limbs()[0], 0); // 0
}

#[test]
fn sdivrem_divisor_negative() {
    let a = ApInt::new(8, 100);
    let b = ApInt::new(8, 0xff); // -1
    let (q, r) = a.sdivrem(&b);
    assert_eq!(q.get_limbs()[0], 0x9c); // -100
    assert_eq!(r.get_limbs()[0], 0); // 0
}

#[test]
fn sdivrem_both_negative() {
    let a = ApInt::new(8, 0x80); // -128
    let b = ApInt::new(8, 0xff); // -1
    let (q, r) = a.sdivrem(&b);
    assert_eq!(q.get_limbs()[0], 0x80); // -128 / -1 = 128 -> signed: -128
    assert_eq!(r.get_limbs()[0], 0); // 0
}

#[test]
fn sdivrem_negative_remainder() {
    let a = ApInt::new(8, 0xff); // -1
    let b = ApInt::new(8, 2);
    let (q, r) = a.sdivrem(&b);
    assert_eq!(q.get_limbs()[0], 0); // 0
    assert_eq!(r.get_limbs()[0], 0xff); // -1
}

#[test]
fn sdivrem_negative_remainder_2() {
    let a = ApInt::new(8, 0xfd); // -3
    let b = ApInt::new(8, 2);
    let (q, r) = a.sdivrem(&b);
    assert_eq!(q.get_limbs()[0], 0xff); // -1
    assert_eq!(r.get_limbs()[0], 0xff); // -1
}

#[test]
fn sdivrem_zero_dividend() {
    let a = ApInt::zero(8);
    let b = ApInt::new(8, 5);
    let (q, r) = a.sdivrem(&b);
    assert!(q.is_zero());
    assert!(r.is_zero());
}

#[test]
fn sdivrem_dividend_smaller() {
    let a = ApInt::new(8, 3);
    let b = ApInt::new(8, 100);
    let (q, r) = a.sdivrem(&b);
    assert_eq!(q.get_limbs()[0], 0);
    assert_eq!(r.get_limbs()[0], 3);
}

#[test]
fn sdivrem_negative_dividend_smaller() {
    let a = ApInt::new(8, 0xfd); // -3
    let b = ApInt::new(8, 100);
    let (q, r) = a.sdivrem(&b);
    assert_eq!(q.get_limbs()[0], 0);
    assert_eq!(r.get_limbs()[0], 0xfd); // -3
}

#[test]
fn sdivrem_by_one() {
    let a = ApInt::new(8, 42);
    let b = ApInt::one(8);
    let (q, r) = a.sdivrem(&b);
    assert_eq!(q.get_limbs()[0], 42);
    assert_eq!(r.get_limbs()[0], 0);
}

#[test]
fn sdivrem_by_negative_one() {
    let a = ApInt::new(8, 42);
    let b = ApInt::new(8, 0xff); // -1
    let (q, r) = a.sdivrem(&b);
    assert_eq!(q.get_limbs()[0], 0xd6); // -42
    assert_eq!(r.get_limbs()[0], 0);
}

#[test]
fn sdivrem_negative_by_negative_one() {
    let a = ApInt::new(8, 0xd6); // -42
    let b = ApInt::new(8, 0xff); // -1
    let (q, r) = a.sdivrem(&b);
    assert_eq!(q.get_limbs()[0], 42);
    assert_eq!(r.get_limbs()[0], 0);
}

#[test]
fn sdivrem_max_values() {
    let a = ApInt::new(8, 0x80); // -128
    let b = ApInt::new(8, 0x80); // -128
    let (q, r) = a.sdivrem(&b);
    assert_eq!(q.get_limbs()[0], 1);
    assert_eq!(r.get_limbs()[0], 0);
}

#[test]
fn sdivrem_max_positive_by_negative() {
    let a = ApInt::new(8, 0x7f); // 127
    let b = ApInt::new(8, 0xff); // -1
    let (q, r) = a.sdivrem(&b);
    assert_eq!(q.get_limbs()[0], 0x81); // -127
    assert_eq!(r.get_limbs()[0], 0);
}

#[test]
fn sdivrem_multi_limb() {
    let a = ApInt::from_limbs(128, vec![0x123456789abcdef, 0x1]);
    let b = ApInt::from_limbs(128, vec![0x10000, 0]);
    let (q, r) = a.sdivrem(&b);
    let product = q.mul(&b).add(&r);
    assert_eq!(product, a);
    assert!(r.ult(&b));
}

#[test]
fn sdivrem_multi_limb_negative() {
    let a = ApInt::from_limbs(128, vec![0x123456789abcdef, 0x8000000000000000]);
    let b = ApInt::from_limbs(128, vec![0x10000, 0]);
    let (q, r) = a.sdivrem(&b);
    let product = q.mul(&b).add(&r);
    assert_eq!(product, a);
    let abs_r = if r.is_negative() { r.neg() } else { r.clone() };
    let abs_b = if b.is_negative() { b.neg() } else { b.clone() };
    assert!(abs_r.ult(&abs_b));
    assert_eq!(r.is_negative(), a.is_negative());
}

#[test]
#[should_panic(expected = "division by zero")]
fn sdivrem_by_zero() {
    let a = ApInt::new(8, 100);
    let b = ApInt::zero(8);
    a.sdivrem(&b);
}

#[test]
#[should_panic]
fn sdivrem_different_widths() {
    let a = ApInt::new(32, 100);
    let b = ApInt::new(64, 3);
    a.sdivrem(&b);
}

// ========== Combined Tests for udiv, urem, sdiv, srem ==========

#[test]
fn udiv_simple() {
    let a = ApInt::new(32, 100);
    let b = ApInt::new(32, 3);
    assert_eq!(a.udiv(&b).get_limbs()[0], 33);
}

#[test]
fn urem_simple() {
    let a = ApInt::new(32, 100);
    let b = ApInt::new(32, 3);
    assert_eq!(a.urem(&b).get_limbs()[0], 1);
}

#[test]
fn sdiv_simple() {
    let a = ApInt::new(8, 0x80); // -128
    let b = ApInt::new(8, 2);
    assert_eq!(a.sdiv(&b).get_limbs()[0], 0xc0); // -64
}

#[test]
fn srem_simple() {
    let a = ApInt::new(8, 0xff); // -1
    let b = ApInt::new(8, 2);
    assert_eq!(a.srem(&b).get_limbs()[0], 0xff); // -1
}

#[test]
fn sdiv_round_toward_zero() {
    // -7 / 2 = -3 (truncated toward zero)
    let a = ApInt::new(8, 0xf9); // -7
    let b = ApInt::new(8, 2);
    let q = a.sdiv(&b);
    assert_eq!(q.get_limbs()[0], 0xfd); // -3

    // -7 % 2 = -1
    let r = a.srem(&b);
    assert_eq!(r.get_limbs()[0], 0xff); // -1
}

#[test]
fn sdiv_round_toward_zero_2() {
    // 7 / -2 = -3 (truncated toward zero)
    let a = ApInt::new(8, 7);
    let b = ApInt::new(8, 0xfe); // -2
    let q = a.sdiv(&b);
    assert_eq!(q.get_limbs()[0], 0xfd); // -3

    // 7 % -2 = 1
    let r = a.srem(&b);
    assert_eq!(r.get_limbs()[0], 1);
}

// ========== Property-based Tests ==========

#[test]
fn udivrem_property() {
    // For any a, b > 0: a = q*b + r, 0 <= r < b
    let test_cases = vec![
        (100u128, 3u128, 32),
        (1000, 7, 64),
        (12345, 234, 32),
        (u64::MAX as u128, 3, 64),
        (u64::MAX as u128, u64::MAX as u128, 64),
        (0x123456789abcdef, 0x10000, 128),
    ];

    for (dividend, divisor, width) in test_cases {
        let a = ApInt::new(width, dividend);
        let b = ApInt::new(width, divisor);
        let (q, r) = a.udivrem(&b);

        // a = q * b + r
        let product = q.mul(&b);
        let sum = product.add(&r);
        assert_eq!(sum, a);

        // 0 <= r < b
        assert!(r.uge(&ApInt::zero(width)));
        assert!(r.ult(&b));
    }
}

#[test]
fn sdivrem_property() {
    let test_cases: Vec<(i128, i128, usize)> = vec![
        // 8-bit: [-128, 127]
        (100, 3, 8),
        (-100, 3, 8),
        (100, -3, 8),
        (-100, -3, 8),
        (127, 2, 8),
        (-128, 2, 8),
        (127, -2, 8),
        (-128, -2, 8),
        (10, 3, 8),
        (-10, 3, 8),
        (10, -3, 8),
        (-10, -3, 8),
        (127, 3, 8),
        (-128, 3, 8),
        (127, -3, 8),
        (-128, -3, 8),
        (1, 1, 8),
        (-1, 1, 8),
        (1, -1, 8),
        (-1, -1, 8),
        (0, 1, 8),
        (0, -1, 8),
        (127, 127, 8),
        (-128, 127, 8),
        (127, -128, 8),
        (-128, -128, 8),
        // 16-bit: [-32768, 32767]
        (1000, 7, 16),
        (-1000, 7, 16),
        (1000, -7, 16),
        (-1000, -7, 16),
        (32767, 2, 16),
        (-32768, 2, 16),
        (32767, -2, 16),
        (-32768, -2, 16),
        // 32-bit: [-2147483648, 2147483647]
        (12345, 234, 32),
        (-12345, 234, 32),
        (12345, -234, 32),
        (-12345, -234, 32),
        // 1-bit: [-1, 0]
        (0, 1, 1),
        (0, -1, 1),
        (1, 1, 1),
        (-1, -1, 1),
        // 7-bit: [-64, 63]
        (5, 2, 7),
        (-5, 2, 7),
        (5, -2, 7),
        (-5, -2, 7),
        (63, 2, 7),
        (-64, 2, 7),
        (63, -2, 7),
        (-64, -2, 7),
        // 128-bit
        (100, 3, 128),
        (-100, 3, 128),
        (100, -3, 128),
        (-100, -3, 128),
    ];

    for (dividend, divisor, width) in test_cases {
        if divisor == 0 {
            continue;
        }

        let (min_value, max_value) = if width == 128 {
            (i128::MIN, i128::MAX)
        } else if width == 1 {
            (-1, 0)
        } else {
            let max = (1i128 << (width - 1)) - 1;
            let min = -(1i128 << (width - 1));
            (min, max)
        };

        if dividend < min_value || dividend > max_value {
            continue;
        }
        if divisor < min_value || divisor > max_value {
            continue;
        }

        let a = if dividend < 0 {
            let abs = dividend.unsigned_abs();
            ApInt::new(width, abs).neg()
        } else {
            ApInt::new(width, dividend as u128)
        };

        let b = if divisor < 0 {
            let abs = divisor.unsigned_abs();
            ApInt::new(width, abs).neg()
        } else {
            ApInt::new(width, divisor as u128)
        };

        let (q, r) = a.sdivrem(&b);

        // a = q * b + r
        let product = q.mul(&b);
        let sum = product.add(&r);
        assert_eq!(
            sum, a,
            "Failed for dividend={}, divisor={}, width={}",
            dividend, divisor, width
        );

        // |r| < |b|
        let abs_r = if r.is_negative() { r.neg() } else { r.clone() };
        let abs_b = if b.is_negative() { b.neg() } else { b.clone() };
        assert!(
            abs_r.ult(&abs_b),
            "Failed for dividend={}, divisor={}, width={}",
            dividend,
            divisor,
            width
        );

        // sign(r) == sign(a) (only if r != 0)
        if !r.is_zero() {
            assert_eq!(
                r.is_negative(),
                a.is_negative(),
                "Failed for dividend={}, divisor={}, width={}: r={:?}, a={:?}",
                dividend,
                divisor,
                width,
                r,
                a
            );
        }
    }
}

#[test]
fn udivrem_single_bit_width() {
    let a = ApInt::new(1, 1);
    let b = ApInt::one(1);
    let (q, r) = a.udivrem(&b);
    assert_eq!(q.get_limbs()[0], 1);
    assert_eq!(r.get_limbs()[0], 0);
}

#[test]
fn sdivrem_single_bit() {
    let a = ApInt::new(1, 0);
    let b = ApInt::one(1);
    let (q, r) = a.sdivrem(&b);
    assert!(q.is_zero());
    assert!(r.is_zero());
}

#[test]
fn sdivrem_min_integer() {
    let a = ApInt::new(8, 0x80); // -128
    let b = ApInt::new(8, 0xff); // -1
    let (q, r) = a.sdivrem(&b);
    // -128 / -1 = 128 -> in 8-bit this wraps to -128
    assert_eq!(q.get_limbs()[0], 0x80); // -128
    assert_eq!(r.get_limbs()[0], 0);
}

#[test]
fn udivrem_odd_widths() {
    let a = ApInt::new(7, 0b1010101); // 85
    let b = ApInt::new(7, 0b0000010); // 2
    let (q, r) = a.udivrem(&b);
    assert_eq!(q.get_limbs()[0], 42);
    assert_eq!(r.get_limbs()[0], 1);
}

#[test]
fn sdivrem_odd_widths() {
    let a = ApInt::new(7, 0b1010101); // -43 (since 7-bit)
    let b = ApInt::new(7, 0b0000010); // 2
    let (q, r) = a.sdivrem(&b);
    // -43 / 2 = -21 remainder -1
    assert_eq!(q.get_limbs()[0], 0b1101011); // -21
    assert_eq!(r.get_limbs()[0], 0b1111111); // -1
}

#[test]
fn to_u8_lossy_basic() {
    let x = ApInt::new(8, 0xff);
    assert_eq!(x.to_u8_lossy(), 255);

    let x = ApInt::new(8, 0x00);
    assert_eq!(x.to_u8_lossy(), 0);

    let x = ApInt::new(8, 0x7f);
    assert_eq!(x.to_u8_lossy(), 127);
}

#[test]
fn to_u8_lossy_truncation() {
    let x = ApInt::new(16, 0x1234);
    assert_eq!(x.to_u8_lossy(), 0x34);

    let x = ApInt::new(32, 0x12345678);
    assert_eq!(x.to_u8_lossy(), 0x78);

    let x = ApInt::new(64, 0x123456789abcdef0);
    assert_eq!(x.to_u8_lossy(), 0xf0);

    let x = ApInt::new(128, 0x123456789abcdef0123456789abcdef0);
    assert_eq!(x.to_u8_lossy(), 0xf0);
}

#[test]
fn to_u8_lossy_negative() {
    let x = ApInt::new(8, 0xff); // -1
    assert_eq!(x.to_u8_lossy(), 255);

    let x = ApInt::new(8, 0x80); // -128
    assert_eq!(x.to_u8_lossy(), 128);
}

#[test]
fn to_u16_lossy_basic() {
    let x = ApInt::new(16, 0xffff);
    assert_eq!(x.to_u16_lossy(), 65535);

    let x = ApInt::new(16, 0x0000);
    assert_eq!(x.to_u16_lossy(), 0);

    let x = ApInt::new(16, 0x7fff);
    assert_eq!(x.to_u16_lossy(), 32767);
}

#[test]
fn to_u16_lossy_truncation() {
    let x = ApInt::new(32, 0x12345678);
    assert_eq!(x.to_u16_lossy(), 0x5678);

    let x = ApInt::new(64, 0x123456789abcdef0);
    assert_eq!(x.to_u16_lossy(), 0xdef0);
}

#[test]
fn to_u16_lossy_negative() {
    let x = ApInt::new(16, 0xffff); // -1
    assert_eq!(x.to_u16_lossy(), 65535);

    let x = ApInt::new(16, 0x8000); // -32768
    assert_eq!(x.to_u16_lossy(), 32768);
}

#[test]
fn to_u32_lossy_basic() {
    let x = ApInt::new(32, 0xffffffff);
    assert_eq!(x.to_u32_lossy(), 4294967295);

    let x = ApInt::new(32, 0x00000000);
    assert_eq!(x.to_u32_lossy(), 0);

    let x = ApInt::new(32, 0x7fffffff);
    assert_eq!(x.to_u32_lossy(), 2147483647);
}

#[test]
fn to_u32_lossy_truncation() {
    let x = ApInt::new(64, 0x123456789abcdef0);
    assert_eq!(x.to_u32_lossy(), 0x9abcdef0);
}

#[test]
fn to_u32_lossy_negative() {
    let x = ApInt::new(32, 0xffffffff); // -1
    assert_eq!(x.to_u32_lossy(), 4294967295);

    let x = ApInt::new(32, 0x80000000); // -2147483648
    assert_eq!(x.to_u32_lossy(), 2147483648);
}

#[test]
fn to_u64_lossy_basic() {
    let x = ApInt::new(64, u64::MAX as u128);
    assert_eq!(x.to_u64_lossy(), u64::MAX);

    let x = ApInt::new(64, 0);
    assert_eq!(x.to_u64_lossy(), 0);

    let x = ApInt::new(64, 0x123456789abcdef0);
    assert_eq!(x.to_u64_lossy(), 0x123456789abcdef0);
}

#[test]
fn to_u64_lossy_negative() {
    let x = ApInt::new(64, u64::MAX as u128); // -1
    assert_eq!(x.to_u64_lossy(), u64::MAX);

    let x = ApInt::new(64, 0x8000000000000000); // -2^63
    assert_eq!(x.to_u64_lossy(), 0x8000000000000000);
}

#[test]
fn to_u128_lossy_basic() {
    let x = ApInt::new(128, u128::MAX);
    assert_eq!(x.to_u128_lossy(), u128::MAX);

    let x = ApInt::new(128, 0);
    assert_eq!(x.to_u128_lossy(), 0);

    let x = ApInt::new(128, 0x123456789abcdef0123456789abcdef0);
    assert_eq!(x.to_u128_lossy(), 0x123456789abcdef0123456789abcdef0);
}

#[test]
fn to_u128_lossy_truncation() {
    let limbs = vec![0x123456789abcdef0, 0x123456789abcdef0, 0x1];
    let x = ApInt::from_limbs(192, limbs);
    assert_eq!(x.to_u128_lossy(), 0x123456789abcdef0123456789abcdef0);
}

#[test]
fn to_u128_lossy_negative() {
    let x = ApInt::new(128, u128::MAX); // -1
    assert_eq!(x.to_u128_lossy(), u128::MAX);

    let x = ApInt::new(128, 0x80000000000000000000000000000000); // -2^127
    assert_eq!(x.to_u128_lossy(), 0x80000000000000000000000000000000);
}

#[test]
fn conversion_edge_cases() {
    // 1-bit numbers
    let x = ApInt::new(1, 1);
    assert_eq!(x.to_u8_lossy(), 1);

    // 1-bit zero
    let x = ApInt::new(1, 0);
    assert_eq!(x.to_u8_lossy(), 0);

    // 1-bit negative
    let x = ApInt::new(1, 1);
    assert!(x.is_negative());
    assert_eq!(x.to_u8_lossy(), 1);

    // Odd widths
    let x = ApInt::new(7, 0b0111111); // 63
    assert_eq!(x.to_u8_lossy(), 63);

    let x = ApInt::new(9, 0x1ff); // 511
    assert_eq!(x.to_u8_lossy(), 0xff);
    assert_eq!(x.to_u16_lossy(), 511);
}

#[test]
fn conversion_consistency() {
    // For values that fit, lossy and safe should match
    let values = vec![0, 1, 127, 255, 256, 65535, 65536, 4294967295, 4294967296];

    for &val in &values {
        if val <= u8::MAX as u128 {
            let x = ApInt::new(8, val);
            assert_eq!(x.to_u8_lossy(), val as u8);
        }

        if val <= u16::MAX as u128 {
            let x = ApInt::new(16, val);
            assert_eq!(x.to_u16_lossy(), val as u16);
        }

        if val <= u32::MAX as u128 {
            let x = ApInt::new(32, val);
            assert_eq!(x.to_u32_lossy(), val as u32);
        }

        if val <= u64::MAX as u128 {
            let x = ApInt::new(64, val);
            assert_eq!(x.to_u64_lossy(), val as u64);
        }
    }
}

#[test]
fn to_u8_lossy_zero() {
    let x = ApInt::zero(8);
    assert_eq!(x.to_u8_lossy(), 0);

    let x = ApInt::zero(64);
    assert_eq!(x.to_u8_lossy(), 0);

    let x = ApInt::zero(128);
    assert_eq!(x.to_u8_lossy(), 0);
}

#[test]
fn to_u8_lossy_one_bit() {
    let x = ApInt::new(1, 0);
    assert_eq!(x.to_u8_lossy(), 0);

    let x = ApInt::new(1, 1);
    assert_eq!(x.to_u8_lossy(), 1);
}

#[test]
fn to_u8_lossy_odd_widths() {
    let x = ApInt::new(7, 0b0111111); // 63
    assert_eq!(x.to_u8_lossy(), 63);

    let x = ApInt::new(7, 0b1111111); // -1 in 7-bit
    assert_eq!(x.to_u8_lossy(), 127);

    let x = ApInt::new(9, 0x1ff); // 511
    assert_eq!(x.to_u8_lossy(), 0xff);
}

#[test]
fn to_u8_lossy_all_bits_set() {
    let x = ApInt::new(8, 0xff);
    assert_eq!(x.to_u8_lossy(), 0xff);

    let x = ApInt::new(16, 0xffff);
    assert_eq!(x.to_u8_lossy(), 0xff);

    let x = ApInt::new(32, 0xffffffff);
    assert_eq!(x.to_u8_lossy(), 0xff);

    let x = ApInt::new(64, u64::MAX.into());
    assert_eq!(x.to_u8_lossy(), 0xff);

    let x = ApInt::new(128, u128::MAX);
    assert_eq!(x.to_u8_lossy(), 0xff);
}

#[test]
fn to_u16_lossy_zero() {
    let x = ApInt::zero(16);
    assert_eq!(x.to_u16_lossy(), 0);

    let x = ApInt::zero(64);
    assert_eq!(x.to_u16_lossy(), 0);

    let x = ApInt::zero(128);
    assert_eq!(x.to_u16_lossy(), 0);
}

#[test]
fn to_u16_lossy_one_bit() {
    let x = ApInt::new(1, 0);
    assert_eq!(x.to_u16_lossy(), 0);

    let x = ApInt::new(1, 1);
    assert_eq!(x.to_u16_lossy(), 1);
}

#[test]
fn to_u16_lossy_odd_widths() {
    let x = ApInt::new(7, 0b0111111);
    assert_eq!(x.to_u16_lossy(), 63);

    let x = ApInt::new(9, 0x1ff);
    assert_eq!(x.to_u16_lossy(), 0x1ff);

    let x = ApInt::new(17, 0x1ffff);
    assert_eq!(x.to_u16_lossy(), 0xffff);
}

#[test]
fn to_u32_lossy_zero() {
    let x = ApInt::zero(32);
    assert_eq!(x.to_u32_lossy(), 0);

    let x = ApInt::zero(64);
    assert_eq!(x.to_u32_lossy(), 0);

    let x = ApInt::zero(128);
    assert_eq!(x.to_u32_lossy(), 0);
}

#[test]
fn to_u32_lossy_one_bit() {
    let x = ApInt::new(1, 0);
    assert_eq!(x.to_u32_lossy(), 0);

    let x = ApInt::new(1, 1);
    assert_eq!(x.to_u32_lossy(), 1);
}

#[test]
fn to_u64_lossy_truncation() {
    let x = ApInt::new(128, 0x123456789abcdef0123456789abcdef0);
    assert_eq!(x.to_u64_lossy(), 0x123456789abcdef0);

    let limbs = vec![0x123456789abcdef0, 0x123456789abcdef0, 0x1];
    let x = ApInt::from_limbs(192, limbs);
    assert_eq!(x.to_u64_lossy(), 0x123456789abcdef0);
}

#[test]
fn to_u64_lossy_zero() {
    let x = ApInt::zero(64);
    assert_eq!(x.to_u64_lossy(), 0);

    let x = ApInt::zero(128);
    assert_eq!(x.to_u64_lossy(), 0);
}

#[test]
fn to_u64_lossy_one_bit() {
    let x = ApInt::new(1, 0);
    assert_eq!(x.to_u64_lossy(), 0);

    let x = ApInt::new(1, 1);
    assert_eq!(x.to_u64_lossy(), 1);
}

#[test]
fn to_u64_lossy_odd_widths() {
    let x = ApInt::new(7, 0b0111111);
    assert_eq!(x.to_u64_lossy(), 63);

    let x = ApInt::new(9, 0x1ff);
    assert_eq!(x.to_u64_lossy(), 0x1ff);

    let x = ApInt::new(65, 0x1ffffffffffffffff);
    assert_eq!(x.to_u64_lossy(), 0xffffffffffffffff);
}

#[test]
fn to_u128_lossy_zero() {
    let x = ApInt::zero(128);
    assert_eq!(x.to_u128_lossy(), 0);

    let x = ApInt::zero(256);
    assert_eq!(x.to_u128_lossy(), 0);
}

#[test]
fn to_u128_lossy_one_bit() {
    let x = ApInt::new(1, 0);
    assert_eq!(x.to_u128_lossy(), 0);

    let x = ApInt::new(1, 1);
    assert_eq!(x.to_u128_lossy(), 1);
}

#[test]
fn conversion_lossy_edge_cases() {
    // Maximum values
    let x = ApInt::new(8, 0xff);
    assert_eq!(x.to_u8_lossy(), 0xff);
    assert_eq!(x.to_u16_lossy(), 0xff);
    assert_eq!(x.to_u32_lossy(), 0xff);
    assert_eq!(x.to_u64_lossy(), 0xff);
    assert_eq!(x.to_u128_lossy(), 0xff);

    // Minimum values (zero)
    let x = ApInt::zero(1);
    assert_eq!(x.to_u8_lossy(), 0);
    assert_eq!(x.to_u16_lossy(), 0);
    assert_eq!(x.to_u32_lossy(), 0);
    assert_eq!(x.to_u64_lossy(), 0);
    assert_eq!(x.to_u128_lossy(), 0);

    // All bits set in different widths
    let x = ApInt::new(7, 0b1111111);
    assert_eq!(x.to_u8_lossy(), 127);
    assert_eq!(x.to_u16_lossy(), 127);
    assert_eq!(x.to_u32_lossy(), 127);
    assert_eq!(x.to_u64_lossy(), 127);
    assert_eq!(x.to_u128_lossy(), 127);

    // Large numbers
    let x = ApInt::new(128, u128::MAX);
    assert_eq!(x.to_u8_lossy(), 0xff);
    assert_eq!(x.to_u16_lossy(), 0xffff);
    assert_eq!(x.to_u32_lossy(), 0xffffffff);
    assert_eq!(x.to_u64_lossy(), u64::MAX);
    assert_eq!(x.to_u128_lossy(), u128::MAX);
}

#[test]
fn conversion_lossy_consistency() {
    // For all values, lossy conversions should be consistent
    let test_values: Vec<u128> = vec![
        0,
        1,
        2,
        127,
        128,
        255,
        256,
        0x1234,
        0x12345678,
        0x123456789abcdef0,
        u64::MAX as u128,
        u128::MAX,
    ];

    for &val in &test_values {
        let width = if val <= 8 {
            8
        } else if val <= 16 {
            16
        } else if val <= 32 {
            32
        } else if val <= 64 {
            64
        } else {
            128
        };

        let x = ApInt::new(width, val);

        let u8_val = x.to_u8_lossy();
        let u16_val = x.to_u16_lossy();
        let u32_val = x.to_u32_lossy();
        let u64_val = x.to_u64_lossy();
        let u128_val = x.to_u128_lossy();

        // u8 should match lower 8 bits of all larger types
        assert_eq!(u8_val, (u16_val & 0xff) as u8);
        assert_eq!(u8_val, (u32_val & 0xff) as u8);
        assert_eq!(u8_val, (u64_val & 0xff) as u8);
        assert_eq!(u8_val, (u128_val & 0xff) as u8);

        // u16 should match lower 16 bits of all larger types
        assert_eq!(u16_val, (u32_val & 0xffff) as u16);
        assert_eq!(u16_val, (u64_val & 0xffff) as u16);
        assert_eq!(u16_val, (u128_val & 0xffff) as u16);

        // u32 should match lower 32 bits of all larger types
        assert_eq!(u32_val, (u64_val & 0xffffffff) as u32);
        assert_eq!(u32_val, (u128_val & 0xffffffff) as u32);

        // u64 should match lower 64 bits of u128
        assert_eq!(u64_val, (u128_val & u64::MAX as u128) as u64);
    }
}

#[test]
fn conversion_lossy_all_zero_limbs() {
    let x = ApInt::from_limbs(192, vec![0, 0, 0]);
    assert_eq!(x.to_u8_lossy(), 0);
    assert_eq!(x.to_u16_lossy(), 0);
    assert_eq!(x.to_u32_lossy(), 0);
    assert_eq!(x.to_u64_lossy(), 0);
    assert_eq!(x.to_u128_lossy(), 0);
}

#[test]
fn conversion_lossy_single_limb_max() {
    let x = ApInt::from_limbs(64, vec![u64::MAX]);
    assert_eq!(x.to_u8_lossy(), 0xff);
    assert_eq!(x.to_u16_lossy(), 0xffff);
    assert_eq!(x.to_u32_lossy(), 0xffffffff);
    assert_eq!(x.to_u64_lossy(), u64::MAX);
    assert_eq!(x.to_u128_lossy(), u64::MAX as u128);
}

#[test]
fn conversion_lossy_two_limbs() {
    let x = ApInt::from_limbs(128, vec![0x123456789abcdef0, 0x123456789abcdef0]);
    assert_eq!(x.to_u8_lossy(), 0xf0);
    assert_eq!(x.to_u16_lossy(), 0xdef0);
    assert_eq!(x.to_u32_lossy(), 0x9abcdef0);
    assert_eq!(x.to_u64_lossy(), 0x123456789abcdef0);
    assert_eq!(x.to_u128_lossy(), 0x123456789abcdef0123456789abcdef0);
}

#[test]
fn basic_test() {
    let a = ApInt::new(32, 100);
    let b = ApInt::new(32, 50);
    let sum = a + b;
    assert_eq!(sum.to_u32_lossy(), 150);
}

#[test]
fn karatsuba_mul() {
    let a_limbs: Vec<u64> = vec![
        0x123456789abcdef0,
        0x123456789abcdef0,
        0x123456789abcdef0,
        0x123456789abcdef0,
        0x123456789abcdef0,
        0x123456789abcdef0,
        0x123456789abcdef0,
        0x123456789abcdef0,
        0x123456789abcdef0,
        0x123456789abcdef0,
        0x123456789abcdef0,
        0x123456789abcdef0,
        0x123456789abcdef0,
        0x123456789abcdef0,
        0x123456789abcdef0,
        0x123456789abcdef0,
    ];

    let b_limbs: Vec<u64> = vec![
        0xfedcba9876543210,
        0xfedcba9876543210,
        0xfedcba9876543210,
        0xfedcba9876543210,
        0xfedcba9876543210,
        0xfedcba9876543210,
        0xfedcba9876543210,
        0xfedcba9876543210,
        0xfedcba9876543210,
        0xfedcba9876543210,
        0xfedcba9876543210,
        0xfedcba9876543210,
        0xfedcba9876543210,
        0xfedcba9876543210,
        0xfedcba9876543210,
        0xfedcba9876543210,
    ];

    let a = ApInt::from_limbs(1024, a_limbs);
    let b = ApInt::from_limbs(1024, b_limbs);

    let got = a * b;

    let expected_limbs: Vec<u64> = vec![
        0x236d88fe5618cf00,
        0x58fab20783af1222,
        0x8e87db10b1455544,
        0xc4150419dedb9866,
        0xf9a22d230c71db88,
        0x2f2f562c3a081eaa,
        0x64bc7f35679e61cd,
        0x9a49a83e9534a4ef,
        0xcfd6d147c2cae811,
        0x0563fa50f0612b33,
        0x3af1235a1df76e56,
        0x707e4c634b8db178,
        0xa60b756c7923f49a,
        0xdb989e75a6ba37bc,
        0x1125c77ed4507ade,
        0x46b2f08801e6be01,
    ];

    let expected = ApInt::from_limbs(1024, expected_limbs);

    assert_eq!(got, expected);
}
