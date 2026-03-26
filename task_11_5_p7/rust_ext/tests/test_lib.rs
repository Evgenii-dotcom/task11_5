use rust_ext::sum_as_string;

#[test]
fn test_sum_positive() {
    let result = sum_as_string(2, 3);
    assert_eq!(result, "5");
}

#[test]
fn test_sum_zero() {
    let result = sum_as_string(0, 0);
    assert_eq!(result, "0");
}

#[test]
fn test_sum_negative() {
    let result = sum_as_string(-2, -3);
    assert_eq!(result, "-5");
}