use verbena::*;

#[test]
fn test_new_constructor() {
    let s = Str32::new("hello");
    assert_eq!(s.len(), 5);
    assert_eq!(s.to_string(), "hello");
}

#[test]
fn test_from_string_constructor() {
    let s = Str32::from_string("world".to_string());
    assert_eq!(s.len(), 5);
    assert_eq!(s.to_string(), "world");
}

#[test]
fn test_from_char_constructor() {
    let s = Str32::from_char('a');
    assert_eq!(s.len(), 1);
    assert_eq!(s.to_string(), "a");
}

#[test]
fn test_from_vec_constructor() {
    let s = Str32::from_vec(vec!['t', 'e', 's', 't']);
    assert_eq!(s.len(), 4);
    assert_eq!(s.to_string(), "test");
}

#[test]
fn test_len() {
    let s = Str32::new("hello");
    assert_eq!(s.len(), 5);

    let empty = Str32::new("");
    assert_eq!(empty.len(), 0);

    // Test with multi-byte Unicode characters
    let unicode = Str32::new("こんにちは");
    assert_eq!(unicode.len(), 5);
}

#[test]
fn test_is_empty() {
    let empty = Str32::new("");
    assert!(empty.is_empty());

    let non_empty = Str32::new("test");
    assert!(!non_empty.is_empty());
}

#[test]
fn test_at_valid_index() {
    let s = Str32::new("hello");
    assert_eq!(s.at(0), Ok('h'));
    assert_eq!(s.at(4), Ok('o'));

    // Test with Unicode
    let unicode = Str32::new("こんにちは");
    assert_eq!(unicode.at(0), Ok('こ'));
    assert_eq!(unicode.at(4), Ok('は'));
}

#[test]
fn test_at_invalid_index() {
    let s = Str32::new("hello");
    assert!(s.at(5).is_err());
    assert!(s.at(10).is_err());
}

#[test]
fn test_to_string() {
    let original = "Hello, world!";
    let s = Str32::new(original);
    assert_eq!(s.to_string(), original);

    // Test with Unicode
    let unicode_original = "こんにちは世界";
    let unicode = Str32::new(unicode_original);
    assert_eq!(unicode.to_string(), unicode_original);
}

#[test]
fn test_substr() {
    let s = Str32::new("hello world");

    let sub = s.substr(0, 5);
    assert_eq!(sub.to_string(), "hello");

    let sub2 = s.substr(6, 11);
    assert_eq!(sub2.to_string(), "world");

    // Empty substring
    let empty = s.substr(5, 5);
    assert!(empty.is_empty());

    // Unicode substring
    let unicode = Str32::new("こんにちは世界");
    let uni_sub = unicode.substr(0, 3);
    assert_eq!(uni_sub.to_string(), "こんに");
}

#[test]
#[should_panic]
fn test_substr_out_of_bounds() {
    let s = Str32::new("hello");
    let _ = s.substr(3, 10); // This should panic due to out of bounds
}

#[test]
fn test_add() {
    let s1 = Str32::new("hello");
    let s2 = Str32::new(" world");

    let combined = s1.add(&s2);
    assert_eq!(combined.to_string(), "hello world");
    assert_eq!(combined.len(), 11);

    // Original strings should remain unchanged
    assert_eq!(s1.to_string(), "hello");
    assert_eq!(s2.to_string(), " world");

    // Adding an empty string
    let empty = Str32::new("");
    assert_eq!(s1.add(&empty).to_string(), "hello");
    assert_eq!(empty.add(&s1).to_string(), "hello");
}

#[test]
fn test_upper() {
    let s = Str32::new("Hello, World!");
    let upper = s.upper();
    assert_eq!(upper.to_string(), "HELLO, WORLD!");

    // Original should remain unchanged
    assert_eq!(s.to_string(), "Hello, World!");

    // Test with mixed case and symbols
    let mixed = Str32::new("aBcD123!@#");
    assert_eq!(mixed.upper().to_string(), "ABCD123!@#");

    // Test with Unicode
    let unicode = Str32::new("こんにちはabc");
    assert_eq!(unicode.upper().to_string(), "こんにちはABC");
}

#[test]
fn test_lower() {
    let s = Str32::new("Hello, World!");
    let lower = s.lower();
    assert_eq!(lower.to_string(), "hello, world!");

    // Original should remain unchanged
    assert_eq!(s.to_string(), "Hello, World!");

    // Test with mixed case and symbols
    let mixed = Str32::new("aBcD123!@#");
    assert_eq!(mixed.lower().to_string(), "abcd123!@#");

    // Test with Unicode
    let unicode = Str32::new("こんにちはABC");
    assert_eq!(unicode.lower().to_string(), "こんにちはabc");
}

#[test]
fn test_repeat() {
    let s = Str32::new("abc");

    // Repeat 3 times
    let repeated = s.repeat(3);
    assert_eq!(repeated.to_string(), "abcabcabc");
    assert_eq!(repeated.len(), 9);

    // Original should remain unchanged
    assert_eq!(s.to_string(), "abc");

    // Repeat 0 times should give empty string
    let zero_repeat = s.repeat(0);
    assert!(zero_repeat.is_empty());

    // Repeat with Unicode
    let unicode = Str32::new("こん");
    assert_eq!(unicode.repeat(2).to_string(), "こんこん");
}

#[test]
fn test_display_trait() {
    let s = Str32::new("hello");
    assert_eq!(format!("{}", s), "hello");

    // Test with Unicode
    let unicode = Str32::new("こんにちは");
    assert_eq!(format!("{}", unicode), "こんにちは");
}

#[test]
fn test_debug_trait() {
    let s = Str32::new("hello");
    assert_eq!(format!("{:?}", s), "Str32(\"hello\")");

    // Test with Unicode
    let unicode = Str32::new("こんにちは");
    assert_eq!(format!("{:?}", unicode), "Str32(\"こんにちは\")");
}

#[test]
fn test_clone() {
    let s1 = Str32::new("hello");
    let s2 = s1.clone();

    // Both should have the same content
    assert_eq!(s1.to_string(), s2.to_string());

    // Modifying one shouldn't affect the other
    // (We can't directly test this with current API, but the test is valuable
    // to document the expected behavior)
}

#[test]
fn test_eq() {
    let s1 = Str32::new("hello");
    let s2 = Str32::new("hello");
    let s3 = Str32::new("world");

    assert_eq!(s1, s2);
    assert!(s1 != s3);

    // Test with different constructors
    let s4 = Str32::from_string("hello".to_string());
    assert_eq!(s1, s4);

    // Test with Unicode
    let unicode1 = Str32::new("こんにちは");
    let unicode2 = Str32::new("こんにちは");
    let unicode3 = Str32::new("さようなら");

    assert_eq!(unicode1, unicode2);
    assert!(unicode1 != unicode3);
}

#[test]
fn test_performance_large_strings() {
    // Create a large string
    let large_str = "a".repeat(10000);
    let s = Str32::new(&large_str);

    // Basic operations should complete without issues
    assert_eq!(s.len(), 10000);
    assert!(!s.is_empty());
    assert_eq!(s.at(5000), Ok('a'));
    assert_eq!(s.at(9999), Ok('a'));
    assert!(s.at(10000).is_err());
}

#[test]
fn test_hash() {
    use std::collections::HashMap;

    let mut map = HashMap::new();

    let s1 = Str32::new("hello");
    let s2 = Str32::new("world");
    let s3 = Str32::new("hello"); // Same as s1

    map.insert(s1.clone(), 1);
    map.insert(s2.clone(), 2);

    // s3 should hash to the same value as s1
    assert_eq!(map.get(&s3), Some(&1));
    assert_eq!(map.get(&s2), Some(&2));

    // Adding s3 should overwrite the value for s1
    map.insert(s3, 3);
    assert_eq!(map.get(&s1), Some(&3));
}
