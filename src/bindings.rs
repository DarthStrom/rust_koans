#[test]
fn bindings_are_immutable_by_default() {
    let x = 7;  // `let mut x` to make it mutable
    assert_eq!(7, x);
    x = 27;
    assert_eq!(27, x);
}

#[test]
fn types_are_inferred() {
    let x: u32 = 7.0;  // allow the type to be inferred
    assert_eq!(7.0, x);
}

#[test]
fn bindings_must_be_initialized_before_use() {
    let x: i32;
    println!("x is {}", x);
}

#[test]
fn bindings_use_patterns() {
    let (x, y) = (7, 27);
    assert_eq!(__, x);
    assert_eq!(__, y);
}