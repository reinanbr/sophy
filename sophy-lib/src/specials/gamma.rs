//! Gamma function implementation
//!
//! The gamma function Γ(x) extends the factorial function to real and complex numbers.
//! For positive integers n, Γ(n) = (n-1)!

use crate::base::numbers::PI;

/// Gamma function Γ(x) - generalization of factorial to real numbers
///
/// The gamma function extends the factorial function to real and complex numbers.
/// For positive integers n, Γ(n) = (n-1)!
///
/// ## Mathematical Definition
///
/// Γ(x) = ∫₀^∞ t^(x-1) * e^(-t) dt
///
/// ## Properties
///
/// - Γ(n) = (n-1)! for positive integers n
/// - Γ(x+1) = x * Γ(x) (recurrence relation)
/// - Γ(1/2) = √π
///
/// ## Implementation
///
/// Uses Lanczos approximation for high precision (~15 decimal digits).
///
/// ## Examples
///
/// ```rust
/// use sophy::specials::gamma;
/// use std::f64::consts::PI;
///
/// // Factorial relationship
/// assert!((gamma(5.0) - 24.0).abs() < 1e-10);  // Γ(5) = 4! = 24
/// assert!((gamma(3.0) - 2.0).abs() < 1e-10);   // Γ(3) = 2! = 2
///
/// // Special values
/// let sqrt_pi = PI.sqrt();
/// assert!((gamma(0.5) - sqrt_pi).abs() < 1e-10);  // Γ(1/2) = √π
/// ```
///
/// ## Panics
///
/// Panics if x ≤ 0, as gamma function is undefined for non-positive values.
pub fn gamma(x: f64) -> f64 {
    if x <= 0.0 {
        panic!("Gamma function undefined for non-positive values");
    }

    // Use recurrence relation to shift x into range [1, 2)
    if x < 1.0 {
        return gamma(x + 1.0) / x;
    }

    // For x >= 2, use recurrence relation Γ(x) = (x-1) * Γ(x-1)
    if x >= 2.0 {
        return (x - 1.0) * gamma(x - 1.0);
    }

    // Lanczos approximation for x in [1, 2)
    const G: f64 = 7.0;
    const COEFFICIENTS: [f64; 9] = [
        0.999_999_999_999_809_9,
        676.520_368_121_885_1,
        -1_259.139_216_722_402_8,
        771.323_428_777_653_1,
        -176.615_029_162_140_6,
        12.507_343_278_686_905,
        -0.138_571_095_265_720_12,
        9.984_369_578_019_572e-6,
        1.505_632_735_149_311_6e-7,
    ];

    let z = x - 1.0;
    let mut a = COEFFICIENTS[0];

    for (i, &coeff) in COEFFICIENTS.iter().enumerate().skip(1) {
        a += coeff / (z + i as f64);
    }

    let t = z + G + 0.5;
    (2.0 * PI).sqrt() * t.powf(z + 0.5) * (-t).exp() * a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gamma_factorial() {
        // Test factorial relationship: Γ(n) = (n-1)!
        assert!((gamma(1.0) - 1.0).abs() < 1e-10); // Γ(1) = 0! = 1
        assert!((gamma(2.0) - 1.0).abs() < 1e-10); // Γ(2) = 1! = 1
        assert!((gamma(3.0) - 2.0).abs() < 1e-10); // Γ(3) = 2! = 2
        assert!((gamma(4.0) - 6.0).abs() < 1e-10); // Γ(4) = 3! = 6
        assert!((gamma(5.0) - 24.0).abs() < 1e-10); // Γ(5) = 4! = 24
    }

    #[test]
    fn test_gamma_half() {
        // Test Γ(1/2) = √π
        let sqrt_pi = std::f64::consts::PI.sqrt();
        assert!((gamma(0.5) - sqrt_pi).abs() < 1e-10);
    }

    #[test]
    #[should_panic(expected = "Gamma function undefined for non-positive values")]
    fn test_gamma_negative() {
        gamma(-1.0);
    }

    #[test]
    #[should_panic(expected = "Gamma function undefined for non-positive values")]
    fn test_gamma_zero() {
        gamma(0.0);
    }
}
