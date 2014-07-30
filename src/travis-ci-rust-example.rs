pub fn add_numbers(a: int, b: int) -> int {
    a + b
}

#[test]
fn test_add_numbers() {
    assert_eq!(3, add_numbers(1, 2));
    assert_eq!(-7, add_numbers(5, -12));
}
