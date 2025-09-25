//! # Number Constants and Utilities
//!
//! This module provides fundamental mathematical constants and number manipulation utilities.

/// Machine epsilon for f64 precision
///
/// The smallest representable positive number such that 1.0 + EPSILON != 1.0
///
/// # Examples
///
/// ```rust
/// use sophy::base::numbers::EPSILON;
///
/// assert!(EPSILON > 0.0);
/// assert!(1.0 + EPSILON != 1.0);
/// ```
pub const EPSILON: f64 = f64::EPSILON;

/// The mathematical constant π (pi)
///
/// The ratio of a circle's circumference to its diameter
///
/// # Examples
///
/// ```rust
/// use sophy::base::numbers::PI;
///
/// let circumference = 2.0 * PI * 5.0; // radius = 5
/// assert!((circumference - 31.415926535897932).abs() < 1e-15);
/// ```
pub const PI: f64 = std::f64::consts::PI;

/// Euler's number (e)
///
/// The base of the natural logarithm
///
/// # Examples
///
/// ```rust
/// use sophy::base::numbers::EULER;
///
/// // e^1 should equal EULER, and ln(e) should equal 1
/// assert!((EULER.ln() - 1.0).abs() < 1e-15);
/// ```
pub const EULER: f64 = std::f64::consts::E;

/// The golden ratio (φ)
///
/// The golden ratio (1 + √5) / 2 ≈ 1.618033988749895
///
/// # Examples
///
/// ```rust
/// use sophy::base::numbers::PHI;
///
/// // φ² = φ + 1
/// assert!((PHI * PHI - (PHI + 1.0)).abs() < 1e-15);
/// ```
pub const PHI: f64 = 1.618033988749895;
