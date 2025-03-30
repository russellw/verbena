use std::cell::RefCell;
use std::rc::Rc;
use verbena::*;

#[test]
fn test_empty_object() {
    let obj = Object::new();
    assert!(obj.is_empty());
    assert_eq!(obj.len(), 0);
    assert_eq!(obj.to_string(), "{}");
}

#[test]
fn test_object_with_primitives() {
    let mut obj = Object::new();
    obj.insert("null", Val::Null);
    obj.insert("true", Val::True);
    obj.insert("false", Val::False);
    obj.insert("number", Val::Num(42.5));
    obj.insert("string", Val::Str("hello".to_string()));

    assert_eq!(obj.len(), 5);
    assert!(!obj.is_empty());

    // Display representation should be properly formatted
    let display_str = obj.to_string();
    assert!(display_str.contains("\"null\": null"));
    assert!(display_str.contains("\"true\": true"));
    assert!(display_str.contains("\"false\": false"));
    assert!(display_str.contains("\"number\": 42.5"));
    assert!(display_str.contains("\"string\": hello"));
}

#[test]
fn test_nested_objects() {
    let mut inner_obj = Object::new();
    inner_obj.insert("inner_key", Val::Num(99.9));

    let mut obj = Object::new();
    obj.insert("nested", Val::Object(Rc::new(RefCell::new(inner_obj))));

    assert_eq!(obj.len(), 1);
    let display_str = obj.to_string();
    assert!(display_str.contains("\"nested\": {\"inner_key\": 99.9}"));
}

#[test]
fn test_objects_with_lists() {
    use crate::list::List;

    let mut list = List::new();
    list.push(Val::Num(1.0));
    list.push(Val::Num(2.0));
    list.push(Val::Num(3.0));

    let mut obj = Object::new();
    obj.insert("array", Val::List(Rc::new(RefCell::new(list))));

    assert_eq!(obj.len(), 1);
    let display_str = obj.to_string();
    assert!(display_str.contains("\"array\": [1, 2, 3]"));
}

#[test]
fn test_object_equality() {
    let obj1 = Object::new();
    let obj2 = Object::new();

    // Different objects with same content should not be equal (reference equality)
    assert_ne!(obj1, obj2);

    // Same object should be equal to itself
    assert_eq!(obj1, obj1);

    // Testing with Val::Object
    let val_obj1 = Val::Object(Rc::new(RefCell::new(obj1)));
    let obj3 = Object::new();
    let val_obj2 = Val::Object(Rc::new(RefCell::new(obj3)));

    assert_ne!(val_obj1, val_obj2);
}

#[test]
fn test_complex_object() {
    use crate::list::List;

    // Create a complex nested structure
    let mut inner_list = List::new();
    inner_list.push(Val::Str("a".to_string()));
    inner_list.push(Val::Str("b".to_string()));

    let mut inner_obj = Object::new();
    inner_obj.insert("x", Val::Num(10.0));
    inner_obj.insert("y", Val::Num(20.0));

    let mut obj = Object::new();
    obj.insert("list", Val::List(Rc::new(RefCell::new(inner_list))));
    obj.insert("object", Val::Object(Rc::new(RefCell::new(inner_obj))));
    obj.insert("scalar", Val::Num(30.0));

    let display_str = obj.to_string();
    assert!(display_str.contains("\"list\": [a, b]"));
    assert!(display_str.contains("\"object\": {"));
    assert!(display_str.contains("\"x\": 10"));
    assert!(display_str.contains("\"y\": 20"));
    assert!(display_str.contains("\"scalar\": 30"));
}

#[test]
fn test_object_with_functions() {
    let func = Val::func0(|_vm| Ok(Val::Null));

    let mut obj = Object::new();
    obj.insert("function", func);

    let display_str = obj.to_string();
    assert!(display_str.contains("\"function\": <fn>"));
}

#[test]
fn test_insert_method() {
    let mut obj = Object::new();

    // Test inserting with &str
    let prev = obj.insert("key1", Val::Num(100.0));
    assert!(prev.is_none());

    // Test inserting with String
    let prev = obj.insert(String::from("key2"), Val::Num(200.0));
    assert!(prev.is_none());

    // Test overwriting an existing key
    let prev = obj.insert("key1", Val::Num(300.0));
    assert!(prev.is_some());
    if let Some(Val::Num(val)) = prev {
        assert_eq!(val, 100.0);
    } else {
        panic!("Previous value was not a Num(100.0)");
    }

    // Verify the object contains the expected values
    let display_str = obj.to_string();
    assert!(display_str.contains("\"key1\": 300"));
    assert!(display_str.contains("\"key2\": 200"));
}

#[test]
fn test_get_method() {
    let mut obj = Object::new();

    // Insert some test values
    obj.insert("string", Val::Str("hello".to_string()));
    obj.insert("number", Val::Num(42.0));
    obj.insert("bool", Val::True);

    // Test get with existing keys
    match obj.get("string") {
        Val::Str(s) => assert_eq!(s, "hello"),
        _ => panic!("Expected string value"),
    }

    match obj.get("number") {
        Val::Num(n) => assert_eq!(n, 42.0),
        _ => panic!("Expected number value"),
    }

    assert_eq!(obj.get("bool"), Val::True);

    // Test get with a nonexistent key - should return Val::Null
    assert_eq!(obj.get("nonexistent"), Val::Null);

    // Test get with String
    let key = String::from("string");
    assert!(matches!(obj.get(key), Val::Str(_)));

    // Test get with &String
    let key = String::from("number");
    assert!(matches!(obj.get(&key), Val::Num(_)));
}
