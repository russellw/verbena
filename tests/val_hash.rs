use num_bigint::BigInt;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::str::FromStr;
use verbena::*;

// Helper function to create a VM for testing
fn create_test_vm() -> VM {
    // This assumes VM has a constructor or can be created this way
    // Adjust as needed based on your VM implementation
    VM::new()
}

#[test]
fn test_val_in_hashset() {
    let mut set = HashSet::new();

    // Test basic values
    set.insert(Val::True);
    set.insert(Val::False);
    set.insert(Val::Null);

    assert!(set.contains(&Val::True));
    assert!(set.contains(&Val::False));
    assert!(set.contains(&Val::Null));
    assert_eq!(set.len(), 3);

    // Insert duplicates should not increase size
    set.insert(Val::True);
    assert_eq!(set.len(), 3);
}

#[test]
fn test_val_int_in_hashset() {
    let mut set = HashSet::new();

    // Test Int values
    set.insert(Val::Int(BigInt::from(42)));
    set.insert(Val::Int(BigInt::from(100)));

    assert!(set.contains(&Val::Int(BigInt::from(42))));
    assert!(set.contains(&Val::Int(BigInt::from(100))));
    assert!(!set.contains(&Val::Int(BigInt::from(101))));
    assert_eq!(set.len(), 2);

    // Test with larger numbers
    set.insert(Val::Int(BigInt::from_str("12345678901234567890").unwrap()));
    assert!(set.contains(&Val::Int(BigInt::from_str("12345678901234567890").unwrap())));
    assert_eq!(set.len(), 3);
}

#[test]
fn test_val_float_in_hashset() {
    let mut set = HashSet::new();

    // Test Float values
    set.insert(Val::Float(3.14));
    set.insert(Val::Float(2.71));

    assert!(set.contains(&Val::Float(3.14)));
    assert!(set.contains(&Val::Float(2.71)));
    assert!(!set.contains(&Val::Float(1.23)));
    assert_eq!(set.len(), 2);

    // Special float values
    set.insert(Val::Float(f64::NAN));
    set.insert(Val::Float(f64::INFINITY));
    set.insert(Val::Float(f64::NEG_INFINITY));

    // Note: NaN != NaN in IEEE 754, so this test might need special consideration
    // for NaN values, depending on how you've implemented the Hash trait
    assert!(set.contains(&Val::Float(f64::INFINITY)));
    assert!(set.contains(&Val::Float(f64::NEG_INFINITY)));
}

#[test]
fn test_val_str_in_hashset() {
    let mut set = HashSet::new();

    // Test Str values
    let str1 = Val::from_string("hello".to_string());
    let str2 = Val::from_string("world".to_string());

    set.insert(str1.clone());
    set.insert(str2.clone());

    assert!(set.contains(&str1));
    assert!(set.contains(&str2));
    assert!(!set.contains(&Val::from_string("other".to_string())));
    assert_eq!(set.len(), 2);

    // Insert duplicate strings
    set.insert(Val::from_string("hello".to_string()));
    assert_eq!(set.len(), 2); // Should still be 2 as "hello" is already in the set
}

#[test]
fn test_val_object_in_hashset() {
    let mut set = HashSet::new();

    // Create some objects
    let obj1 = Rc::new(RefCell::new(Object::new()));
    let obj2 = Rc::new(RefCell::new(Object::new()));

    set.insert(Val::Object(obj1.clone()));
    set.insert(Val::Object(obj2.clone()));

    assert!(set.contains(&Val::Object(obj1.clone())));
    assert!(set.contains(&Val::Object(obj2.clone())));
    assert_eq!(set.len(), 2);

    // Insert the same object again
    set.insert(Val::Object(obj1.clone()));
    assert_eq!(set.len(), 2); // Should still be 2
}

#[test]
fn test_val_function_in_hashset() {
    let mut set = HashSet::new();
    let mut vm = create_test_vm();

    // Create some function values
    let func0 = Val::func0(|_vm| Ok(Val::True));
    let func1 = Val::func1(|_vm, _arg| Ok(Val::True));

    set.insert(func0.clone());
    set.insert(func1.clone());

    assert!(set.contains(&func0));
    assert!(set.contains(&func1));
    assert_eq!(set.len(), 2);

    // Create a new function with the same implementation
    let func0_same_impl = Val::func0(|_vm| Ok(Val::True));
    set.insert(func0_same_impl.clone());

    // Since functions are compared by reference equality, the new function
    // should be considered distinct even with the same implementation
    assert_eq!(set.len(), 3);
}

#[test]
fn test_val_in_hashmap() {
    let mut map = HashMap::new();

    // Insert different types of Val as keys
    map.insert(Val::True, "boolean true");
    map.insert(Val::False, "boolean false");
    map.insert(Val::Null, "null value");
    map.insert(Val::Int(BigInt::from(42)), "answer");
    map.insert(Val::Float(3.14), "pi");
    map.insert(Val::from_string("hello".to_string()), "greeting");

    // Check lookups
    assert_eq!(map.get(&Val::True), Some(&"boolean true"));
    assert_eq!(map.get(&Val::False), Some(&"boolean false"));
    assert_eq!(map.get(&Val::Null), Some(&"null value"));
    assert_eq!(map.get(&Val::Int(BigInt::from(42))), Some(&"answer"));
    assert_eq!(map.get(&Val::Float(3.14)), Some(&"pi"));
    assert_eq!(
        map.get(&Val::from_string("hello".to_string())),
        Some(&"greeting")
    );

    // Check non-existent key
    assert_eq!(map.get(&Val::Int(BigInt::from(100))), None);

    // Update value
    map.insert(Val::True, "updated boolean true");
    assert_eq!(map.get(&Val::True), Some(&"updated boolean true"));

    // Get map size
    assert_eq!(map.len(), 6);
}

#[test]
fn test_mixed_val_types_in_hashmap() {
    let mut map = HashMap::new();

    // Create a complex HashMap with Val keys of different types
    map.insert(Val::True, 1);
    map.insert(Val::Int(BigInt::from(42)), 2);
    map.insert(Val::Float(3.14), 3);
    map.insert(Val::from_string("key".to_string()), 4);

    let obj = Rc::new(RefCell::new(Object::new()));
    map.insert(Val::Object(obj.clone()), 5);

    let func = Val::func0(|_vm| Ok(Val::True));
    map.insert(func.clone(), 6);

    // Verify all keys can be retrieved
    assert_eq!(map.get(&Val::True), Some(&1));
    assert_eq!(map.get(&Val::Int(BigInt::from(42))), Some(&2));
    assert_eq!(map.get(&Val::Float(3.14)), Some(&3));
    assert_eq!(map.get(&Val::from_string("key".to_string())), Some(&4));
    assert_eq!(map.get(&Val::Object(obj.clone())), Some(&5));
    assert_eq!(map.get(&func), Some(&6));

    // Size should be 6
    assert_eq!(map.len(), 6);
}

#[test]
fn test_val_equivalence_in_hash() {
    let mut map = HashMap::new();

    // Insert a value with Int key
    map.insert(Val::Int(BigInt::from(42)), "int value");

    // The same value as Float should be considered different
    map.insert(Val::Float(42.0), "float value");

    // Verify both keys are stored separately
    assert_eq!(map.get(&Val::Int(BigInt::from(42))), Some(&"int value"));
    assert_eq!(map.get(&Val::Float(42.0)), Some(&"float value"));
    assert_eq!(map.len(), 2);

    // Similarly for strings and other types
    map.insert(Val::from_string("42".to_string()), "string value");
    assert_eq!(
        map.get(&Val::from_string("42".to_string())),
        Some(&"string value")
    );
    assert_eq!(map.len(), 3);
}

#[test]
fn test_hashmap_with_val_values() {
    // Test using Val as both key and value
    let mut map = HashMap::new();

    map.insert(
        Val::from_string("key1".to_string()),
        Val::Int(BigInt::from(100)),
    );
    map.insert(Val::from_string("key2".to_string()), Val::Float(3.14));
    map.insert(Val::from_string("key3".to_string()), Val::True);

    assert_eq!(
        map.get(&Val::from_string("key1".to_string())),
        Some(&Val::Int(BigInt::from(100)))
    );
    assert_eq!(
        map.get(&Val::from_string("key2".to_string())),
        Some(&Val::Float(3.14))
    );
    assert_eq!(
        map.get(&Val::from_string("key3".to_string())),
        Some(&Val::True)
    );
}
