//! # Sophy 🧮
//!
//! [![Crates.io](https://img.shields.io/crates/v/sophy.svg)](https://crates.io/crates/sophy)
//! [![Documentation](https://docs.rs/sophy/badge.svg)](https://docs.rs/sophy)
//! [![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/reinanbr/sophy/blob/main/LICENSE-MIT)
//! [![Build Status](https://github.com/reinanbr/sophy/workflows/CI/badge.svg)](https://github.com/reinanbr/sophy/actions)
//!
//! **Sophy** is a lightweight, efficient, and extensible mathematical library written in pure Rust.
//! It provides numerical methods, mathematical functions, and number utilities for scientific computing,
//! educational tools, and general-purpose applications.
//!
//! ## ✨ Features
//!
//! - 🔢 **Numerical Methods**: Newton-Raphson root finding, integration, interpolation
//! - 📊 **Mathematical Functions**: Exponential, logarithmic, trigonometric functions
//! - 🧮 **Number Utilities**: Base operations, number theory functions
//! - 🦀 **Pure Rust**: Memory-safe, zero-cost abstractions
//! - 📚 **Well Documented**: Comprehensive documentation with examples
//! - 🧪 **Well Tested**: Extensive test suite with high coverage
//!
//! ## 🚀 Quick Start
//!
//! Add Sophy to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! sophy = "0.1.23"
//! ```
//!
//! ## 📖 Examples
//!
//! ### Newton-Raphson Root Finding
//!
//! Find the square root of 2 using Newton-Raphson method:
//!
//! ```rust
//! use sophy::methods::raphson::raphson;
//!
//! // Define the function f(x) = x² - 2
//! let f = |x: f64| x * x - 2.0;
//! // Define its derivative f'(x) = 2x
//! let df = |x: f64| 2.0 * x;
//!
//! // Find the root starting from x₀ = 1.0
//! let root = raphson(1.0, f, df, 1e-10, 100);
//!
//! println!("√2 ≈ {:.10}", root); // Output: √2 ≈ 1.4142135624
//! assert!((root - std::f64::consts::SQRT_2).abs() < 1e-10);
//! ```
//!
//! ### Base Number Operations
//!
//! Work with different number bases and conversions:
//!
//! ```rust
//! use sophy::base::numbers;
//!
//! // Example: Convert decimal to binary (when implemented)
//! // let binary = numbers::to_binary(42);
//! // println!("42 in binary: {}", binary); // Output: 101010
//! ```
//!
//! ## 🏗️ Architecture
//!
//! Sophy is organized into focused modules:
//!
//! - [`methods`]: Numerical methods for solving mathematical problems
//! - [`base`]: Fundamental number operations and utilities
//!
//! ## 🔬 Precision & Performance
//!
//! Sophy uses `f64` precision by default for optimal balance between accuracy and performance.
//! All algorithms are implemented with numerical stability in mind.
//!
//! ## 🤝 Contributing
//!
//! Contributions are welcome! Please see our [contributing guide](https://github.com/reinanbr/sophy/blob/main/CONTRIBUTING.md) for details.
//!
//! ## 📜 License
//!
//! This project is dual-licensed under the [MIT License](https://opensource.org/licenses/MIT) 
//! or [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0).

pub mod base;
pub mod methods;
#[cfg(test)]
mod tests {
    use crate::methods;

    #[test]
    fn test_raphson() {
        let f = |x: f64| x * x - 2.0; // Function: x^2 - 2
        let df = |x: f64| 2.0 * x; // Derivative: 2x
        let root = methods::raphson::raphson(1.0, f, df, 1e-10, 100);

        assert!((root - std::f64::consts::SQRT_2).abs() < 1e-10);
    }
}
