// Helpers for rating calculation.

// Make sure a percentage cannot go lower than a given value.
pub fn minimum(mut value: f64, floor: f64) -> f64 {
    let room = 1.0 - floor;
    value *= room;

    return floor + value;
}

// Stretch a value so that low becomes 0.0 and high becomes 1.0
pub fn stretch(mut value: f64, low: f64, high: f64) -> f64 {
    let room = high - low;
    value -= low;
    value /= room;
    return value;
}