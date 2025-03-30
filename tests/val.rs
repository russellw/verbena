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

    // Test to_str
    assert_eq!(null_val.to_string(), "null");
}

#[test]
fn test_float_values() {
    // Test creation with different float values
    let zero_float = Val::Num(0.0);
    let one_float = Val::Num(1.0);
    let pi = Val::Num(3.14159);
    let negative = Val::Num(-2.718);

    // Test infinity and NaN
    let neg_infinity = Val::Num(f64::NEG_INFINITY);
    let nan = Val::Num(f64::NAN);

    // Test truth function
    assert!(!zero_float.truth());
    assert!(one_float.truth());
    assert!(pi.truth());
    assert!(negative.truth());

    // Test equality
    assert_eq!(zero_float, Val::Num(0.0));
    assert_ne!(zero_float, one_float);

    // Float equalities involving NaN should behave like regular f64
    assert_ne!(nan, nan); // NaN != NaN in IEEE floating point

    // Test integer conversions
    assert_eq!(zero_float.get_u32().unwrap(), 0);
    assert_eq!(one_float.get_i32().unwrap(), 1);
    assert_eq!(pi.get_i32().unwrap(), 3);
    assert_eq!(negative.get_i32().unwrap(), -2);

    // Test conversions of non-finite values should fail
    assert!(neg_infinity.get_u32().is_err());
    assert!(nan.get_i32().is_err());

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

    // Test to_str
    assert_eq!(hello.to_string(), "Hello");
    assert_eq!(empty_str.to_string(), "");
}

#[test]
fn test_comparison_functions() {
    // String comparison
    assert!(lt(
        &Val::Str("apple".to_string()),
        &Val::Str("banana".to_string())
    ));
}

#[test]
fn test_debug_and_display() {
    // Test Debug implementation
    let float_val = Val::Num(3.14);
    let str_val = Val::Str("Hello".to_string());

    // Convert debug output to string
    let float_debug = format!("{:?}", float_val);
    let str_debug = format!("{:?}", str_val);

    // Check debug format
    assert!(float_debug.starts_with("Num"));
    assert!(str_debug.starts_with("Str"));

    // Test Display implementation
    assert_eq!(format!("{}", float_val), "3.14");
    assert_eq!(format!("{}", str_val), "Hello");
    assert_eq!(format!("{}", Val::True), "true");
    assert_eq!(format!("{}", Val::False), "false");
    assert_eq!(format!("{}", Val::Null), "null");
}
