// Helpers for rating calculation.

// Make sure a percentage cannot go lower than a given value.
pub fn cap(mut value: f64, low_cap: f64) -> f64 {
    let room = 1.0 - low_cap;
    value *= room;

    return low_cap + value;
}