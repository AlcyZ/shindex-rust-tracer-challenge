pub(crate) mod matrix;
pub(crate) mod transformation;
pub(crate) mod tuple;

pub(crate) const EPSILON: f64 = 0.00001;

pub(crate) fn f64_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}
