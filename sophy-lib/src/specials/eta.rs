//! Dirichlet eta function implementation
//!
//! The Dirichlet eta function η(s) is related to the Riemann zeta function
//! and appears in analytic number theory and mathematical analysis.

use crate::specials::zeta::zeta;

/// Dirichlet eta function η(s) - alternating series variant of zeta function
///
/// The Dirichlet eta function is defined as:
/// η(s) = Σ((-1)^(n-1)/n^s) = 1 - 1/2^s + 1/3^s - 1/4^s + ...
///
/// ## Mathematical Definition
///
/// η(s) = Σ((-1)^(n-1)/n^s) for n from 1 to infinity
///
/// ## Relationship to Zeta Function
///
/// For s ≠ 1: η(s) = (1 - 2^(1-s)) * ζ(s)
///
/// ## Special Values
///
/// - η(1) = ln(2) ≈ 0.6931
/// - η(2) = π²/12 ≈ 0.8225
/// - η(0) = 1/2
///
/// ## Implementation
///
/// - For s = 1: returns ln(2) exactly
/// - For s > 1: uses relationship with zeta function
/// - For 0 < s ≤ 1: uses direct alternating series
///
/// ## Examples
///
/// ```rust
/// use sophy::specials::eta;
///
/// // Special value: η(1) = ln(2)
/// let eta1 = eta(1.0);
/// let ln2 = 2.0_f64.ln();
/// assert!((eta1 - ln2).abs() < 1e-10);
///
/// // Convergent series
/// assert!(eta(2.0) > 0.0);
/// assert!(eta(3.0) > 0.0);
/// ```
///
/// ## Panics
///
/// Panics if s ≤ 0, as the implementation is not defined for non-positive values.
pub fn eta(s: f64) -> f64 {
    if s <= 0.0 {
        panic!("Eta function implementation requires s > 0");
    }

    // Special case: η(1) = ln(2)
    if (s - 1.0).abs() < 1e-15 {
        return 2.0_f64.ln();
    }

    // For s != 1, use relation: η(s) = (1 - 2^(1-s)) * ζ(s)
    if s > 1.0 {
        let factor = 1.0 - 2.0_f64.powf(1.0 - s);
        return factor * zeta(s);
    }

    // Direct series calculation for 0 < s <= 1
    let mut sum = 0.0;
    let mut sign = 1.0;
    let tolerance = 1e-15;

    for n in 1..=1000000 {
        let term = sign / (n as f64).powf(s);
        if term.abs() < tolerance {
            break;
        }
        sum += term;
        sign *= -1.0;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eta_one() {
        // Test η(1) = ln(2)
        let computed = eta(1.0);
        let expected = 2.0_f64.ln();
        assert!((computed - expected).abs() < 1e-10);
    }

    #[test]
    fn test_eta_positive() {
        // Test that eta is positive for positive s
        let test_values = [0.5, 1.0, 1.5, 2.0, 3.0];
        for &s in &test_values {
            assert!(eta(s) > 0.0);
        }
    }

    #[test]
    fn test_eta_convergence() {
        // Test basic properties - eta should be well-defined for positive values
        let eta_values = [eta(1.0), eta(2.0), eta(3.0), eta(5.0)];

        // All values should be finite and positive for s > 0
        for &val in &eta_values {
            assert!(val.is_finite());
            assert!(val > 0.0);
        }

        // eta(1) should be the largest among small integer values due to ln(2)
        assert!(eta(1.0) > 0.5);
    }

    #[test]
    fn test_eta_known_approximations() {
        // Test some known approximate values
        assert!((eta(2.0) - 0.8225).abs() < 1e-3); // π²/12 ≈ 0.8225
    }

    #[test]
    #[should_panic(expected = "Eta function implementation requires s > 0")]
    fn test_eta_zero() {
        eta(0.0);
    }

    #[test]
    #[should_panic(expected = "Eta function implementation requires s > 0")]
    fn test_eta_negative() {
        eta(-1.0);
    }
}
