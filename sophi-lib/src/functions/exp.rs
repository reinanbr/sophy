
use crate::base::numbers::epsilon;

pub fn exp(x: f64) -> f64 {
    let mut result = 1.0;
    let mut term = 1.0;
    let mut n = 1;

    while term > epsilon {
        term *= x / n as f64;
        result += term;
        n += 1;
    }

    result
}