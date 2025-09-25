//! Error function implementation
//!
//! The error function is a special function that appears frequently in
//! probability theory, statistics, and partial differential equations.

/// Error function erf(x) - fundamental in probability and statistics
///
/// The error function is defined as:
/// erf(x) = (2/√π) * ∫₀^x e^(-t²) dt
///
/// ## Mathematical Properties
///
/// - erf(0) = 0
/// - erf(∞) = 1
/// - erf(-x) = -erf(x) (odd function)
/// - erf(x) is closely related to the cumulative distribution function of the normal distribution
///
/// ## Applications
///
/// - Normal distribution calculations
/// - Heat conduction problems
/// - Diffusion processes
/// - Signal processing
///
/// ## Implementation
///
/// Uses Abramowitz and Stegun approximation with maximum error of 1.5 × 10^(-7).
///
/// ## Examples
///
/// ```rust
/// use sophy::specials::erf;
///
/// // Basic properties
/// assert!((erf(0.0) - 0.0).abs() < 1e-10);
/// assert!(erf(5.0) > 0.999);  // erf(∞) ≈ 1
///
/// // Odd function property: erf(-x) = -erf(x)
/// let x = 1.5;
/// assert!((erf(-x) + erf(x)).abs() < 1e-10);
///
/// // Monotonic increase
/// assert!(erf(1.0) < erf(2.0));
/// ```
pub fn erf(x: f64) -> f64 {
    // erf(0) = 0
    if x == 0.0 {
        return 0.0;
    }

    // Use odd function property: erf(-x) = -erf(x)
    if x < 0.0 {
        return -erf(-x);
    }

    // For large x, erf(x) approaches 1
    if x > 5.0 {
        return 1.0;
    }

    // Abramowitz and Stegun approximation
    // Maximum error: 1.5 × 10^(-7)
    const A1: f64 = 0.254829592;
    const A2: f64 = -0.284496736;
    const A3: f64 = 1.421413741;
    const A4: f64 = -1.453152027;
    const A5: f64 = 1.061405429;
    const P: f64 = 0.3275911;

    let t = 1.0 / (1.0 + P * x);
    let poly = A1 * t + A2 * t * t + A3 * t * t * t + A4 * t.powi(4) + A5 * t.powi(5);

    1.0 - poly * (-x * x).exp()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_erf_zero() {
        assert!((erf(0.0) - 0.0).abs() < 1e-15);
    }

    #[test]
    fn test_erf_symmetry() {
        // Test odd function property: erf(-x) = -erf(x)
        let test_values = [0.5, 1.0, 1.5, 2.0];
        for &x in &test_values {
            assert!((erf(-x) + erf(x)).abs() < 1e-10);
        }
    }

    #[test]
    fn test_erf_monotonic() {
        // Test that erf is monotonically increasing
        let values = [0.0, 0.5, 1.0, 1.5, 2.0, 3.0];
        for i in 1..values.len() {
            assert!(erf(values[i - 1]) < erf(values[i]));
        }
    }

    #[test]
    fn test_erf_limits() {
        // Test limits
        assert!(erf(5.0) > 0.999); // erf(∞) ≈ 1
        assert!(erf(-5.0) < -0.999); // erf(-∞) ≈ -1
    }

    #[test]
    fn test_erf_known_values() {
        // Test some approximately known values
        assert!((erf(1.0) - 0.8427).abs() < 1e-3);
        assert!((erf(2.0) - 0.9953).abs() < 1e-3);
    }
}
