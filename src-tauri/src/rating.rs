// Helpers for rating calculation.

// Make sure a percentage cannot go lower than a given value.
pub fn restrict_minimum(mut value: f64, floor: f64) -> f64 {
    let room = 1.0 - floor;
    value *= room;

    return floor + value;
}

// Stretch a value so that low becomes 0.0 and high becomes 1.0
pub fn stretch(value: usize, low: usize, high: usize) -> f64 {
    let new_high = high - low;
    let new_value = value - low;
    return new_value as f64 / new_high as f64;
}