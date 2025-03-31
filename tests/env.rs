use std::cell::RefCell;
use std::rc::Rc;
use verbena::*;

#[test]
fn test_env_new() {
    // Test creating an environment with no outer env
    let _env = Env::new(None, 3);

    // Test creating an environment with an outer env
    let outer_env = Rc::new(RefCell::new(Env::new(None, 2)));
    let _env = Env::new(Some(Rc::clone(&outer_env)), 3);
}

#[test]
fn test_env_get_set_same_level() {
    // Test setting and getting values at level 0
    let mut env = Env::new(None, 3);

    // Set values
    env.set(0, 0, Val::Num(42.0));
    env.set(0, 1, Val::Str("hello".to_string()));
    env.set(0, 2, Val::True);

    // Get values and verify
    assert_eq!(env.get(0, 0), Val::Num(42.0));
    assert_eq!(env.get(0, 1), Val::Str("hello".to_string()));
    assert_eq!(env.get(0, 2), Val::True);
}

#[test]
fn test_env_get_set_nested_one_level() {
    // Create outer environment
    let outer_env = Rc::new(RefCell::new(Env::new(None, 2)));

    // Set values in outer env
    outer_env.borrow_mut().set(0, 0, Val::Num(100.0));
    outer_env
        .borrow_mut()
        .set(0, 1, Val::Str("outer".to_string()));

    // Create inner environment
    let mut inner_env = Env::new(Some(Rc::clone(&outer_env)), 1);
    inner_env.set(0, 0, Val::False);

    // Get values from inner env (level 0)
    assert_eq!(inner_env.get(0, 0), Val::False);

    // Get values from outer env (level 1)
    assert_eq!(inner_env.get(1, 0), Val::Num(100.0));
    assert_eq!(inner_env.get(1, 1), Val::Str("outer".to_string()));

    // Modify outer env through inner env
    inner_env.set(1, 0, Val::Num(200.0));
    assert_eq!(inner_env.get(1, 0), Val::Num(200.0));
    assert_eq!(outer_env.borrow().get(0, 0), Val::Num(200.0));
}

#[test]
fn test_env_get_set_nested_multiple_levels() {
    // Create level 2 (outermost) environment
    let env_lvl2 = Rc::new(RefCell::new(Env::new(None, 1)));
    env_lvl2.borrow_mut().set(0, 0, Val::Num(2.0));

    // Create level 1 environment
    let env_lvl1 = Rc::new(RefCell::new(Env::new(Some(Rc::clone(&env_lvl2)), 1)));
    env_lvl1.borrow_mut().set(0, 0, Val::Num(1.0));

    // Create level 0 (innermost) environment
    let mut env_lvl0 = Env::new(Some(Rc::clone(&env_lvl1)), 1);
    env_lvl0.set(0, 0, Val::Num(0.0));

    // Get values from different levels
    assert_eq!(env_lvl0.get(0, 0), Val::Num(0.0)); // Level 0
    assert_eq!(env_lvl0.get(1, 0), Val::Num(1.0)); // Level 1
    assert_eq!(env_lvl0.get(2, 0), Val::Num(2.0)); // Level 2

    // Modify values at different levels
    env_lvl0.set(0, 0, Val::Num(10.0)); // Modify level 0
    env_lvl0.set(1, 0, Val::Num(11.0)); // Modify level 1
    env_lvl0.set(2, 0, Val::Num(12.0)); // Modify level 2

    // Verify changes
    assert_eq!(env_lvl0.get(0, 0), Val::Num(10.0));
    assert_eq!(env_lvl0.get(1, 0), Val::Num(11.0));
    assert_eq!(env_lvl0.get(2, 0), Val::Num(12.0));

    // Verify changes in the original environments
    assert_eq!(env_lvl1.borrow().get(0, 0), Val::Num(11.0));
    assert_eq!(env_lvl2.borrow().get(0, 0), Val::Num(12.0));
}

#[test]
fn test_env_with_complex_values() {
    // Create an environment with various Val types
    let mut env = Env::new(None, 5);

    // Set different types of values
    env.set(0, 0, Val::Num(3.14));
    env.set(0, 1, Val::Str("Complex".to_string()));
    env.set(0, 2, Val::True);
    env.set(0, 3, Val::False);
    env.set(0, 4, Val::Null);

    // Verify all types are stored and retrieved correctly
    assert_eq!(env.get(0, 0), Val::Num(3.14));
    assert_eq!(env.get(0, 1), Val::Str("Complex".to_string()));
    assert_eq!(env.get(0, 2), Val::True);
    assert_eq!(env.get(0, 3), Val::False);
    assert_eq!(env.get(0, 4), Val::Null);

    // Test overwriting values with different types
    env.set(0, 0, Val::Str("Overwritten".to_string()));
    env.set(0, 1, Val::Num(42.0));

    assert_eq!(env.get(0, 0), Val::Str("Overwritten".to_string()));
    assert_eq!(env.get(0, 1), Val::Num(42.0));
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_env_out_of_bounds_get() {
    let env = Env::new(None, 1);
    // This should panic because there's only one element at index 0
    let _ = env.get(0, 1);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_env_out_of_bounds_set() {
    let mut env = Env::new(None, 1);
    // This should panic because there's only one element at index 0
    env.set(0, 1, Val::Null);
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn test_env_invalid_level_get() {
    let env = Env::new(None, 1);
    // This should panic because there's no outer environment at level 1
    let _ = env.get(1, 0);
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn test_env_invalid_level_set() {
    let mut env = Env::new(None, 1);
    // This should panic because there's no outer environment at level 1
    env.set(1, 0, Val::Null);
}
