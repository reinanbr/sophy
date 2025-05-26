//! # Sophi
//!
//! A mathematical library for numerical methods, functions,
//! and number utilities in Rust.


pub mod methods;
pub mod base;
#[cfg(test)]
mod tests {
    use crate::methods;

#[test]
    fn test_raphson() {
        let f = |x: f64| x * x - 2.0; // Function: x^2 - 2
        let df = |x: f64| 2.0 * x;    // Derivative: 2x
        let root = methods::raphson::raphson(1.0, f, df, 1e-10, 100);

        assert!((root - std::f64::consts::SQRT_2).abs() < 1e-10);
    }

}
