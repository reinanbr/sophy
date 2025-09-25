//! Sum of divisors function implementation
//!
//! The sum of divisors function σ(n) is fundamental in number theory
//! and appears in the study of perfect numbers and arithmetic functions.

/// Sum of divisors function σ(n) - sum of all positive divisors of n
///
/// The sum of divisors function returns the sum of all positive divisors of n,
/// including 1 and n itself.
///
/// ## Mathematical Definition
///
/// σ(n) = Σ d, where the sum is over all positive divisors d of n
///
/// ## Examples of Values
///
/// - σ(1) = 1 (divisors: {1})
/// - σ(6) = 1 + 2 + 3 + 6 = 12 (divisors: {1, 2, 3, 6})
/// - σ(12) = 1 + 2 + 3 + 4 + 6 + 12 = 28
///
/// ## Perfect Numbers
///
/// A positive integer n is perfect if σ(n) = 2n. Known perfect numbers include:
/// - 6 (σ(6) = 12 = 2×6)
/// - 28 (σ(28) = 56 = 2×28)
/// - 496 (σ(496) = 992 = 2×496)
///
/// ## Implementation
///
/// Uses efficient algorithm that only checks divisors up to √n, with O(√n) complexity.
///
/// ## Examples
///
/// ```rust
/// use sophy::specials::sigma;
///
/// // Basic examples
/// assert_eq!(sigma(1), 1);    // σ(1) = 1
/// assert_eq!(sigma(6), 12);   // σ(6) = 1+2+3+6 = 12
/// assert_eq!(sigma(12), 28);  // σ(12) = 1+2+3+4+6+12 = 28
///
/// // Perfect numbers: σ(n) = 2n
/// assert_eq!(sigma(6), 12);   // 2×6 = 12
/// assert_eq!(sigma(28), 56);  // 2×28 = 56
/// ```
///
/// ## Panics
///
/// Panics if n = 0, as the sum of divisors is undefined for zero.
pub fn sigma(n: u64) -> u64 {
    if n == 0 {
        panic!("Sum of divisors undefined for n = 0");
    }

    if n == 1 {
        return 1;
    }

    let mut sum = 0;
    let sqrt_n = (n as f64).sqrt() as u64;

    for i in 1..=sqrt_n {
        if n.is_multiple_of(i) {
            sum += i;
            // Add the corresponding divisor n/i, but avoid double counting
            if i != n / i {
                sum += n / i;
            }
        }
    }

    sum
}

/// Check if a number is perfect (σ(n) = 2n)
///
/// A perfect number is a positive integer that is equal to the sum of its
/// proper positive divisors (divisors excluding the number itself).
///
/// ## Examples
///
/// ```rust
/// use sophy::specials::is_perfect;
///
/// assert!(is_perfect(6));    // 6 = 1+2+3
/// assert!(is_perfect(28));   // 28 = 1+2+4+7+14
/// assert!(is_perfect(496));  // Perfect number
/// assert!(!is_perfect(10));  // Not perfect
/// ```
pub fn is_perfect(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    sigma(n) == 2 * n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sigma_basic() {
        assert_eq!(sigma(1), 1); // σ(1) = 1
        assert_eq!(sigma(2), 3); // σ(2) = 1+2 = 3
        assert_eq!(sigma(3), 4); // σ(3) = 1+3 = 4
        assert_eq!(sigma(4), 7); // σ(4) = 1+2+4 = 7
        assert_eq!(sigma(6), 12); // σ(6) = 1+2+3+6 = 12
        assert_eq!(sigma(12), 28); // σ(12) = 1+2+3+4+6+12 = 28
    }

    #[test]
    fn test_sigma_perfect_numbers() {
        // Test known perfect numbers
        assert_eq!(sigma(6), 12); // 2×6 = 12
        assert_eq!(sigma(28), 56); // 2×28 = 56
        assert_eq!(sigma(496), 992); // 2×496 = 992
    }

    #[test]
    fn test_sigma_primes() {
        // For prime p: σ(p) = 1 + p
        let primes = [2, 3, 5, 7, 11, 13];
        for &p in &primes {
            assert_eq!(sigma(p), 1 + p);
        }
    }

    #[test]
    fn test_is_perfect() {
        // Test perfect number detection
        assert!(is_perfect(6));
        assert!(is_perfect(28));
        assert!(is_perfect(496));

        // Test non-perfect numbers
        assert!(!is_perfect(1));
        assert!(!is_perfect(10));
        assert!(!is_perfect(100));
    }

    #[test]
    #[should_panic(expected = "Sum of divisors undefined for n = 0")]
    fn test_sigma_zero() {
        sigma(0);
    }
}
