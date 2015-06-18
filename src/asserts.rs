#[test]
fn assert_truth() {
    assert!(false);
}

#[test]
fn assert_with_message() {
    assert!(false, "This should be true!");
}

#[test]
fn assert_equality() {
    let expected_value = 1; // correct this
    let actual_value = 1 + 1;

    assert!(expected_value == actual_value);
}

#[test]
fn a_better_way_of_asserting_equality() {
    let expected_value = 1; // correct this
    let actual_value = 1 + 1;

    assert_eq!(expected_value, actual_value);
}

#[test]
fn fill_in_values() {
    assert_eq!(__, 1 + 1);
}