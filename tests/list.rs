//! Integration tests for the List implementation

use std::cell::RefCell;
use std::rc::Rc;
use verbena::*;

// Helper function to create a list with numeric values
fn create_numeric_list(values: Vec<f64>) -> List {
    let vals: Vec<_> = values.into_iter().map(Val::Num).collect();
    List::from(vals)
}

// Helper function to create a list with string values
fn create_string_list(values: Vec<&str>) -> List {
    let vals: Vec<_> = values
        .into_iter()
        .map(|s| Val::Str(s.to_string()))
        .collect();
    List::from(vals)
}

#[test]
fn test_list_creation() {
    // Test empty list creation
    let empty_list = List::new();
    assert_eq!(empty_list.len(), 0);
    assert_eq!(empty_list.to_string(), "[]");

    // Test list creation from vector
    let nums = create_numeric_list(vec![1.0, 2.0, 3.0]);
    assert_eq!(nums.len(), 3);
    assert_eq!(nums.to_string(), "[1, 2, 3]");

    let strings = create_string_list(vec!["a", "b", "c"]);
    assert_eq!(strings.len(), 3);
    assert_eq!(strings.to_string(), "[a, b, c]");
}

#[test]
fn test_list_repeat() {
    // Test repeating an empty list
    let empty = List::new();
    let repeated_empty = empty.repeat(5);
    assert_eq!(repeated_empty.len(), 0);

    // Test repeating a list with elements
    let nums = create_numeric_list(vec![1.0, 2.0]);

    let repeated_once = nums.repeat(1);
    assert_eq!(repeated_once.len(), 2);
    assert_eq!(repeated_once.to_string(), "[1, 2]");

    let repeated_thrice = nums.repeat(3);
    assert_eq!(repeated_thrice.len(), 6);
    assert_eq!(repeated_thrice.to_string(), "[1, 2, 1, 2, 1, 2]");

    // Test repeating 0 times
    let repeated_zero = nums.repeat(0);
    assert_eq!(repeated_zero.len(), 0);
    assert_eq!(repeated_zero.to_string(), "[]");
}

#[test]
fn test_list_index_access() {
    // Create a list for testing indexing
    let mut list = create_numeric_list(vec![10.0, 20.0, 30.0, 40.0, 50.0]);

    // Test accessing individual elements
    assert_eq!(list[0], Val::Num(10.0));
    assert_eq!(list[2], Val::Num(30.0));
    assert_eq!(list[4], Val::Num(50.0));

    // Test modifying elements
    list[1] = Val::Num(25.0);
    assert_eq!(list[1], Val::Num(25.0));
    list[3] = Val::Str("modified".to_string());
    assert_eq!(list[3], Val::Str("modified".to_string()));
}

#[test]
#[should_panic]
fn test_list_index_out_of_bounds() {
    let list = create_numeric_list(vec![1.0, 2.0, 3.0]);
    let _value = &list[5]; // This should panic
}

#[test]
fn test_list_range_indexing() {
    let list = create_numeric_list(vec![10.0, 20.0, 30.0, 40.0, 50.0]);

    // Test Range (a..b)
    let range_slice = &list[1..4];
    assert_eq!(range_slice.len(), 3);
    assert_eq!(range_slice[0], Val::Num(20.0));
    assert_eq!(range_slice[2], Val::Num(40.0));

    // Test RangeInclusive (a..=b)
    let inclusive_slice = &list[1..=3];
    assert_eq!(inclusive_slice.len(), 3);
    assert_eq!(inclusive_slice[0], Val::Num(20.0));
    assert_eq!(inclusive_slice[2], Val::Num(40.0));

    // Test RangeFrom (a..)
    let from_slice = &list[2..];
    assert_eq!(from_slice.len(), 3);
    assert_eq!(from_slice[0], Val::Num(30.0));
    assert_eq!(from_slice[2], Val::Num(50.0));

    // Test RangeTo (..b)
    let to_slice = &list[..3];
    assert_eq!(to_slice.len(), 3);
    assert_eq!(to_slice[0], Val::Num(10.0));
    assert_eq!(to_slice[2], Val::Num(30.0));

    // Test RangeFull (..)
    let full_slice = &list[..];
    assert_eq!(full_slice.len(), 5);
    assert_eq!(full_slice[0], Val::Num(10.0));
    assert_eq!(full_slice[4], Val::Num(50.0));
}

#[test]
fn test_list_mutability_with_ranges() {
    let mut list = create_numeric_list(vec![10.0, 20.0, 30.0, 40.0, 50.0]);

    // Modify a range
    list[1..4][0] = Val::Num(25.0);
    list[1..4][1] = Val::Num(35.0);
    list[1..4][2] = Val::Num(45.0);

    assert_eq!(list[1], Val::Num(25.0));
    assert_eq!(list[2], Val::Num(35.0));
    assert_eq!(list[3], Val::Num(45.0));

    // Modify with inclusive range
    list[0..=2][1] = Val::Num(22.0);
    assert_eq!(list[1], Val::Num(22.0));

    // Modify with from range
    list[3..][0] = Val::Num(44.0);
    assert_eq!(list[3], Val::Num(44.0));

    // Modify with to range
    list[..2][0] = Val::Num(11.0);
    assert_eq!(list[0], Val::Num(11.0));

    // Modify with full range
    list[..][4] = Val::Num(55.0);
    assert_eq!(list[4], Val::Num(55.0));
}

#[test]
fn test_list_display() {
    // Test empty list
    let empty = List::new();
    assert_eq!(empty.to_string(), "[]");

    // Test numeric list
    let nums = create_numeric_list(vec![1.0, 2.0, 3.0]);
    assert_eq!(nums.to_string(), "[1, 2, 3]");

    // Test mixed content list
    let mut mixed = create_numeric_list(vec![1.0, 2.0]);
    mixed[0] = Val::Str("hello".to_string());
    mixed[1] = Val::True;
    assert_eq!(mixed.to_string(), "[hello, true]");

    // Test nested list
    let inner_list = create_numeric_list(vec![10.0, 20.0]);
    let mut outer_list = create_numeric_list(vec![1.0, 2.0]);
    outer_list[1] = Val::List(Rc::new(RefCell::new(inner_list)));
    assert_eq!(outer_list.to_string(), "[1, [10, 20]]");
}

#[test]
fn test_list_equality() {
    // Create two lists with the same content
    let list1 = create_numeric_list(vec![1.0, 2.0, 3.0]);
    let list2 = create_numeric_list(vec![1.0, 2.0, 3.0]);

    // Even with the same content, they should not be equal
    // because PartialEq compares by identity rather than contents
    assert_ne!(list1, list2);

    // A list should be equal to itself
    assert_eq!(list1, list1);

    // Clone the list (creating a new list)
    let list3 = List::from(list1.clone());
    assert_ne!(list1, list3);

    // Create a List reference and compare
    let rc_list1 = Rc::new(RefCell::new(list1));
    let val1 = Val::List(Rc::clone(&rc_list1));
    let val2 = Val::List(Rc::clone(&rc_list1));

    // These should be equal because they reference the same list
    assert_eq!(val1, val2);
}

#[test]
fn test_mixed_value_types() {
    // Create a list with various Val types
    let mut list = List::new();
    list.push(Val::Num(42.0));
    list.push(Val::Str("hello".to_string()));
    list.push(Val::True);
    list.push(Val::False);
    list.push(Val::Null);

    // Check each value type is preserved
    assert_eq!(list[0], Val::Num(42.0));
    assert_eq!(list[1], Val::Str("hello".to_string()));
    assert_eq!(list[2], Val::True);
    assert_eq!(list[3], Val::False);
    assert_eq!(list[4], Val::Null);

    // Test display formatting
    assert_eq!(list.to_string(), "[42, hello, true, false, null]");
}

#[test]
fn test_nested_lists() {
    // Create nested lists structure
    let inner1 = create_numeric_list(vec![1.0, 2.0]);
    let inner2 = create_string_list(vec!["a", "b"]);

    let mut outer = List::new();
    outer.push(Val::List(Rc::new(RefCell::new(inner1))));
    outer.push(Val::List(Rc::new(RefCell::new(inner2))));

    // Test access to nested lists
    if let Val::List(inner_rc) = &outer[0] {
        let inner = inner_rc.borrow();
        assert_eq!(inner[0], Val::Num(1.0));
        assert_eq!(inner[1], Val::Num(2.0));
    } else {
        panic!("Expected List type");
    }

    if let Val::List(inner_rc) = &outer[1] {
        let inner = inner_rc.borrow();
        assert_eq!(inner[0], Val::Str("a".to_string()));
        assert_eq!(inner[1], Val::Str("b".to_string()));
    } else {
        panic!("Expected List type");
    }

    // Test display of nested lists
    assert_eq!(outer.to_string(), "[[1, 2], [a, b]]");
}
