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
    obj.m.insert("null".to_string(), Val::Null);
    obj.m.insert("true".to_string(), Val::True);
    obj.m.insert("false".to_string(), Val::False);
    obj.m.insert("number".to_string(), Val::Num(42.5));
    obj.m
        .insert("string".to_string(), Val::Str("hello".to_string()));

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
    inner_obj.m.insert("inner_key".to_string(), Val::Num(99.9));

    let mut obj = Object::new();
    obj.m.insert(
        "nested".to_string(),
        Val::Object(Rc::new(RefCell::new(inner_obj))),
    );

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
    obj.m
        .insert("array".to_string(), Val::List(Rc::new(RefCell::new(list))));

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
    inner_obj.m.insert("x".to_string(), Val::Num(10.0));
    inner_obj.m.insert("y".to_string(), Val::Num(20.0));

    let mut obj = Object::new();
    obj.m.insert(
        "list".to_string(),
        Val::List(Rc::new(RefCell::new(inner_list))),
    );
    obj.m.insert(
        "object".to_string(),
        Val::Object(Rc::new(RefCell::new(inner_obj))),
    );
    obj.m.insert("scalar".to_string(), Val::Num(30.0));

    let display_str = obj.to_string();
    assert!(display_str.contains("\"list\": [a, b]"));
    assert!(display_str.contains("\"object\": {"));
    assert!(display_str.contains("\"x\": 10"));
    assert!(display_str.contains("\"y\": 20"));
    assert!(display_str.contains("\"scalar\": 30"));
}

#[test]
fn test_object_with_functions() {
    use crate::VM;

    let func = Val::func0(|_vm| Ok(Val::Null));

    let mut obj = Object::new();
    obj.m.insert("function".to_string(), func);

    let display_str = obj.to_string();
    assert!(display_str.contains("\"function\": <fn>"));
}
