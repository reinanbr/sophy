//! Riemann zeta function implementation
//!
//! The Riemann zeta function ζ(s) is fundamental in number theory and appears
//! in many areas of mathematics including the famous Riemann Hypothesis.

use crate::base::numbers::PI;

/// Riemann zeta function ζ(s) - fundamental function in number theory
///
/// The Riemann zeta function is defined as the analytic continuation of the series:
/// ζ(s) = Σ(1/n^s) for n from 1 to infinity, where s > 1
///
/// ## Mathematical Definition
///
/// For s > 1: ζ(s) = Σ(1/n^s) = 1 + 1/2^s + 1/3^s + 1/4^s + ...
///
/// ## Important Values
///
/// - ζ(2) = π²/6 ≈ 1.6449 (Basel problem)
/// - ζ(4) = π⁴/90 ≈ 1.0823
/// - ζ(6) = π⁶/945 ≈ 1.0173
///
/// ## Implementation
///
/// Uses direct series summation with convergence tolerance of 1e-15.
/// For known exact values, returns the analytical result.
///
/// ## Examples
///
/// ```rust
/// use sophy::specials::zeta;
/// use std::f64::consts::PI;
///
/// // Basel problem: ζ(2) = π²/6
/// let basel = zeta(2.0);
/// let expected = PI * PI / 6.0;
/// assert!((basel - expected).abs() < 1e-10);
///
/// // Other values
/// assert!(zeta(3.0) > 1.0);  // Apéry's constant ≈ 1.202
/// assert!(zeta(4.0) > 1.0);  // π⁴/90 ≈ 1.082
/// ```
///
/// ## Panics
///
/// Panics if s ≤ 1, as the series diverges for s ≤ 1.
pub fn zeta(s: f64) -> f64 {
    if s <= 1.0 {
        panic!("Zeta function implementation requires s > 1");
    }

    // For known values, return exact results
    if (s - 2.0).abs() < 1e-15 {
        return PI * PI / 6.0;
    }
    if (s - 4.0).abs() < 1e-15 {
        return PI.powi(4) / 90.0;
    }

    // Series approximation: ζ(s) = Σ(1/n^s)
    let mut sum = 0.0;
    let mut n = 1.0f64;
    let tolerance = 1e-15;

    loop {
        let term = 1.0 / n.powf(s);
        if term < tolerance {
            break;
        }
        sum += term;
        n += 1.0;

        // Prevent infinite loops
        if n > 1_000_000.0 {
            break;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zeta_basel() {
        // Test Basel problem: ζ(2) = π²/6
        let computed = zeta(2.0);
        let expected = PI * PI / 6.0;
        assert!((computed - expected).abs() < 1e-10);
    }

    #[test]
    fn test_zeta_known_values() {
        // Test other known values
        let zeta4 = zeta(4.0);
        let expected4 = PI.powi(4) / 90.0;
        assert!((zeta4 - expected4).abs() < 1e-10);
    }

    #[test]
    fn test_zeta_convergence() {
        // Test that larger s values converge faster (closer to 1)
        assert!(zeta(10.0) < zeta(3.0));
        assert!(zeta(10.0) > 1.0);
    }

    #[test]
    #[should_panic(expected = "Zeta function implementation requires s > 1")]
    fn test_zeta_invalid_s() {
        zeta(1.0);
    }

    #[test]
    #[should_panic(expected = "Zeta function implementation requires s > 1")]
    fn test_zeta_negative_s() {
        zeta(-1.0);
    }
}
