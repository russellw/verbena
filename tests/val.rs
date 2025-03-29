use num_bigint::BigInt;
use num_traits::{One, Zero};
use verbena::*;

#[test]
fn test_boolean_values() {
    // Test boolean creation
    let true_val = Val::True;
    let false_val = Val::False;

    // Test from_bool
    let true_from_bool = Val::from_bool(true);
    let false_from_bool = Val::from_bool(false);

    assert_eq!(true_val, true_from_bool);
    assert_eq!(false_val, false_from_bool);

    // Test truth function
    assert!(true_val.truth());
    assert!(!false_val.truth());

    // Test equality
    assert_eq!(true_val, Val::True);
    assert_eq!(false_val, Val::False);
    assert_ne!(true_val, false_val);

    // Test conversion to numeric
    assert_eq!(true_val.num().unwrap(), Val::Int(BigInt::one()));
    assert_eq!(false_val.num().unwrap(), Val::Int(BigInt::zero()));

    // Test num_loose
    assert_eq!(true_val.num_loose(), Val::Int(BigInt::one()));
    assert_eq!(false_val.num_loose(), Val::Int(BigInt::zero()));

    // Test to_str
    assert_eq!(true_val.to_string(), "true");
    assert_eq!(false_val.to_string(), "false");
}

#[test]
fn test_null_value() {
    let null_val = Val::Null;

    // Test truth function
    assert!(!null_val.truth());

    // Test equality
    assert_eq!(null_val, Val::Null);
    assert_ne!(null_val, Val::True);

    // Test num conversion should fail
    assert!(null_val.num().is_err());

    // Test to_str
    assert_eq!(null_val.to_string(), "null");
}

#[test]
fn test_int_values() {
    // Test creation with different sizes of integers
    let zero = Val::Int(BigInt::zero());
    let one = Val::Int(BigInt::one());
    let large_number = Val::Int(BigInt::from(i64::MAX));
    let negative = Val::Int(BigInt::from(-42));

    // Test truth function
    assert!(!zero.truth());
    assert!(one.truth());
    assert!(large_number.truth());
    assert!(negative.truth());

    // Test equality
    assert_eq!(zero, Val::Int(BigInt::zero()));
    assert_ne!(zero, one);

    // Test to_bigint
    assert_eq!(one.to_bigint().unwrap(), BigInt::one());

    // Test integer conversions
    assert_eq!(zero.to_u32().unwrap(), 0u32);
    assert_eq!(one.to_i32().unwrap(), 1i32);
    assert_eq!(large_number.to_u64().unwrap(), i64::MAX as u64);
    assert_eq!(negative.to_i32().unwrap(), -42i32);

    // Test that out of range conversions fail
    let too_large = Val::Int(BigInt::from(u64::MAX));
    assert!(too_large.to_i32().is_err());

    // Test negative value conversion to unsigned
    assert!(negative.to_u32().is_err());

    // Test float conversion
    assert_eq!(one.to_f64().unwrap(), 1.0);

    // Test num passes through
    assert_eq!(one.num().unwrap(), one);

    // Test to_str
    assert_eq!(one.to_string(), "1");
    assert_eq!(negative.to_string(), "-42");
}

#[test]
fn test_float_values() {
    // Test creation with different float values
    let zero_float = Val::Float(0.0);
    let one_float = Val::Float(1.0);
    let pi = Val::Float(3.14159);
    let negative = Val::Float(-2.718);

    // Test infinity and NaN
    let infinity = Val::Float(f64::INFINITY);
    let neg_infinity = Val::Float(f64::NEG_INFINITY);
    let nan = Val::Float(f64::NAN);

    // Test truth function
    assert!(!zero_float.truth());
    assert!(one_float.truth());
    assert!(pi.truth());
    assert!(negative.truth());

    // Test equality
    assert_eq!(zero_float, Val::Float(0.0));
    assert_ne!(zero_float, one_float);

    // Float equalities involving NaN should behave like regular f64
    assert_ne!(nan, nan); // NaN != NaN in IEEE floating point

    // Test integer conversions
    assert_eq!(zero_float.to_u32().unwrap(), 0);
    assert_eq!(one_float.to_i32().unwrap(), 1);
    assert_eq!(pi.to_i32().unwrap(), 3);
    assert_eq!(negative.to_i32().unwrap(), -2);

    // Test conversions of non-finite values should fail
    assert!(infinity.to_bigint().is_err());
    assert!(neg_infinity.to_u32().is_err());
    assert!(nan.to_i32().is_err());

    // Test num passes through
    assert_eq!(one_float.num().unwrap(), one_float);

    // Test to_str
    assert_eq!(one_float.to_string(), "1");
    assert_eq!(pi.to_string(), "3.14159");
    assert_eq!(negative.to_string(), "-2.718");
}

#[test]
fn test_string_values() {
    // Test creation
    let empty_str = Val::Str("".to_string());
    let hello = Val::Str("Hello".to_string());
    let world = Val::Str("World".to_string());

    // Test from_string helper
    let from_string = Val::Str("Test".to_string());
    assert_eq!(from_string, Val::Str("Test".to_string()));

    // Test truth function
    assert!(!empty_str.truth());
    assert!(hello.truth());

    // Test equality
    assert_eq!(hello, Val::Str("Hello".to_string()));
    assert_ne!(hello, world);

    // Test num conversion should fail
    assert!(hello.num().is_err());

    // Test to_str
    assert_eq!(hello.to_string(), "Hello");
    assert_eq!(empty_str.to_string(), "");
}

#[test]
fn test_num2_function() {
    // Test with two integers
    let int1 = Val::Int(BigInt::from(5));
    let int2 = Val::Int(BigInt::from(10));
    let (a, b) = num2(&int1, &int2).unwrap();
    assert_eq!(a, int1);
    assert_eq!(b, int2);

    // Test with int and float - should convert int to float
    let float1 = Val::Float(5.5);
    let (a, b) = num2(&int1, &float1).unwrap();
    assert_eq!(a, Val::Float(5.0));
    assert_eq!(b, float1);

    // Test with float and int - should convert int to float
    let (a, b) = num2(&float1, &int1).unwrap();
    assert_eq!(a, float1);
    assert_eq!(b, Val::Float(5.0));

    // Test with non-numeric should fail
    let string = Val::Str("test".to_string());
    assert!(num2(&string, &int1).is_err());
}

#[test]
fn test_num2_loose_function() {
    // Test with booleans - should convert to 0 and 1
    let (a, b) = num2_loose(&Val::True, &Val::False);
    assert_eq!(a, Val::Int(BigInt::one()));
    assert_eq!(b, Val::Int(BigInt::zero()));

    // Test with int and float - should convert int to float
    let int1 = Val::Int(BigInt::from(5));
    let float1 = Val::Float(5.5);
    let (a, b) = num2_loose(&int1, &float1);
    assert_eq!(a, Val::Float(5.0));
    assert_eq!(b, float1);

    // Test with float and int - should convert int to float
    let (a, b) = num2_loose(&float1, &int1);
    assert_eq!(a, float1);
    assert_eq!(b, Val::Float(5.0));

    // Test with non-numeric - should pass through
    let string = Val::Str("test".to_string());
    let (a, b) = num2_loose(&string, &int1);
    assert_eq!(a, string);
    assert_eq!(b, int1);
}

#[test]
fn test_comparison_functions() {
    // Test eq_loose
    assert!(eq_loose(
        &Val::Int(BigInt::from(5)),
        &Val::Int(BigInt::from(5))
    ));
    assert!(eq_loose(&Val::Int(BigInt::from(5)), &Val::Float(5.0)));
    assert!(eq_loose(&Val::True, &Val::Int(BigInt::one())));
    assert!(!eq_loose(
        &Val::Int(BigInt::from(5)),
        &Val::Int(BigInt::from(6))
    ));

    // Test lt_loose
    assert!(lt_loose(
        &Val::Int(BigInt::from(5)),
        &Val::Int(BigInt::from(6))
    ));
    assert!(lt_loose(&Val::Float(5.0), &Val::Float(6.0)));
    assert!(lt_loose(&Val::Int(BigInt::from(5)), &Val::Float(6.0)));
    assert!(!lt_loose(
        &Val::Int(BigInt::from(6)),
        &Val::Int(BigInt::from(5))
    ));

    // String comparison
    assert!(lt_loose(
        &Val::Str("apple".to_string()),
        &Val::Str("banana".to_string())
    ));

    // Test le_loose
    assert!(le_loose(
        &Val::Int(BigInt::from(5)),
        &Val::Int(BigInt::from(5))
    ));
    assert!(le_loose(
        &Val::Int(BigInt::from(5)),
        &Val::Int(BigInt::from(6))
    ));
    assert!(!le_loose(
        &Val::Int(BigInt::from(6)),
        &Val::Int(BigInt::from(5))
    ));
}

#[test]
fn test_debug_and_display() {
    // Test Debug implementation
    let int_val = Val::Int(BigInt::from(42));
    let float_val = Val::Float(3.14);
    let str_val = Val::Str("Hello".to_string());

    // Convert debug output to string
    let int_debug = format!("{:?}", int_val);
    let float_debug = format!("{:?}", float_val);
    let str_debug = format!("{:?}", str_val);

    // Check debug format
    assert!(int_debug.starts_with("Int"));
    assert!(float_debug.starts_with("Float"));
    assert!(str_debug.starts_with("Str"));

    // Test Display implementation
    assert_eq!(format!("{}", int_val), "42");
    assert_eq!(format!("{}", float_val), "3.14");
    assert_eq!(format!("{}", str_val), "Hello");
    assert_eq!(format!("{}", Val::True), "true");
    assert_eq!(format!("{}", Val::False), "false");
    assert_eq!(format!("{}", Val::Null), "null");
}

#[test]
fn test_num3_loose_function() {
    let int_val = Val::Int(BigInt::from(1));
    let float_val = Val::Float(2.0);
    let bool_val = Val::True;

    // Test all ints
    let (a, b, c) = num3_loose(&int_val, &int_val, &int_val);
    assert_eq!(a, Val::Float(1.0));
    assert_eq!(b, Val::Float(1.0));
    assert_eq!(c, Val::Float(1.0));

    // Test mixed values
    let (a, b, c) = num3_loose(&int_val, &float_val, &bool_val);
    assert_eq!(a, Val::Float(1.0));
    assert_eq!(b, Val::Float(2.0));
    assert_eq!(c, Val::Float(1.0));

    // Test other combinations
    let (a, b, c) = num3_loose(&float_val, &int_val, &float_val);
    assert_eq!(a, Val::Float(2.0));
    assert_eq!(b, Val::Float(1.0));
    assert_eq!(c, Val::Float(2.0));

    let (a, b, c) = num3_loose(&int_val, &float_val, &int_val);
    assert_eq!(a, Val::Float(1.0));
    assert_eq!(b, Val::Float(2.0));
    assert_eq!(c, Val::Float(1.0));
}
