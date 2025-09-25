//! # Numerical Methods Module
//!
//! This module provides implementations of various numerical algorithms for solving
//! mathematical problems that cannot be solved analytically.
//!
//! ## Available Methods
//!
//! ### Root Finding
//! - [`raphson()`]: Newton-Raphson method for finding roots of equations
//!
//! ## Usage Examples
//!
//! ```rust
//! use sophy::methods::raphson;
//!
//! // Find the root of f(x) = x³ - x - 1
//! let f = |x: f64| x.powi(3) - x - 1.0;
//! let df = |x: f64| 3.0 * x.powi(2) - 1.0;
//!
//! let root = raphson(1.5, f, df, 1e-12, 100);
//! println!("Root: {:.12}", root);
//! ```
//!
//! ## Future Methods
//!
//! Planned additions include:
//! - Bisection method
//! - Secant method  
//! - Numerical integration (Simpson's rule, trapezoidal rule)
//! - Interpolation methods (Lagrange, spline)

pub mod raphson;
pub use raphson::raphson;
