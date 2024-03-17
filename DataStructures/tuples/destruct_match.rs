/// Returns the minimum of two `Option<i32>` values.
/// When either of the values is `None`, returns the other values.
/// When both values are `None`, returns `None`.

fn min_option((a, b): (Option<i32>, Option<i32>)) -> Option<i32> {
    match (a, b) {
        (Some(x), Some(y)) => Some(x.min(y)),
        (Some(x), None) => Some(x),
        (None, Some(y)) => Some(y),
        (None, None) => None,
    }
}

fn main() {
    assert_eq!(min_option((Some(2), Some(3))), Some(2));
    assert_eq!(min_option((Some(2), None)), Some(2));
    assert_eq!(min_option((None, Some(3))), Some(3));
    assert_eq!(min_option((None, None)), None);
}
