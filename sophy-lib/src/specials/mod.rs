//! # Special Mathematical Functions Module
//!
//! This module provides implementations of special mathematical functions that appear
//! frequently in advanced mathematics, physics, and engineering applications.
//!
//! ## Available Functions
//!
//! ### Special Functions
//! - [`gamma`]: Gamma function Γ(x) - extends factorials to real numbers
//! - [`zeta`]: Riemann zeta function ζ(s) - fundamental in number theory  
//! - [`erf`]: Error function erf(x) - critical for probability and statistics
//! - [`eta`]: Dirichlet eta function η(s) - alternating series variant of zeta
//! - [`sigma`]: Sum of divisors function σ(n) - number theory and perfect numbers
//! - [`is_perfect`]: Check if a number is perfect (σ(n) = 2n)
//!
//! ## Module Organization
//!
//! Each function is implemented in its own module for better organization:
//! - `gamma.rs` - Gamma function implementation
//! - `zeta.rs` - Riemann zeta function implementation  
//! - `erf.rs` - Error function implementation
//! - `eta.rs` - Dirichlet eta function implementation
//! - `sigma.rs` - Sum of divisors function implementation
//!
//! ## Usage Examples
//!
//! ```rust
//! use sophy::specials::{gamma, zeta, erf, eta, sigma, is_perfect};
//!
//! // Gamma function: Γ(5) = 4! = 24
//! let factorial_4 = gamma(5.0);
//!
//! // Riemann zeta function: ζ(2) = π²/6
//! let zeta_2 = zeta(2.0);
//!
//! // Error function for probability calculations
//! let error_val = erf(1.0);
//!
//! // Dirichlet eta function: η(1) = ln(2)
//! let eta_1 = eta(1.0);
//!
//! // Sum of divisors and perfect numbers
//! let sum_div = sigma(28);
//! let is_28_perfect = is_perfect(28);  // true
//! ```
//!
//! ## Mathematical Background
//!
//! These functions are fundamental in:
//! - **Probability theory**: Error function in normal distributions
//! - **Number theory**: Zeta, eta, and sigma functions in prime number studies
//! - **Statistical mechanics**: Gamma function in probability distributions
//! - **Combinatorics**: Gamma function as generalization of factorial
//! - **Analysis**: Special functions in differential equations and integral transforms
//!
//! ## Performance and Precision
//!
//! All functions are implemented with:
//! - **High precision algorithms** (typically 10-15 decimal digits accuracy)
//! - **Efficient computation** using proven mathematical methods
//! - **Comprehensive testing** with known mathematical relationships
//! - **Zero external dependencies** - pure Rust implementations

// Import individual function modules
pub mod erf;
pub mod eta;
pub mod gamma;
pub mod sigma;
pub mod zeta;

// Re-export all public functions for convenient access
pub use erf::erf;
pub use eta::eta;
pub use gamma::gamma;
pub use sigma::{is_perfect, sigma};
pub use zeta::zeta;
