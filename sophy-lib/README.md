
# Sophy Library 🧮

A lightweight mathematical library for Rust providing numerical methods and mathematical utilities.

[![Crates.io](https://img.shields.io/crates/v/sophy.svg)](https://crates.io/crates/sophy)
[![Documentation](https://docs.rs/sophy/badge.svg)](https://docs.rs/sophy)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

This is the core library crate. For usage examples and complete documentation, see the [main project README](../README.md).

## ✨ Features

- 🔢 **Numerical Methods**: Newton-Raphson root finding and more
- 📊 **Mathematical Constants**: π, e, φ, and other important constants  
- 🧮 **Number Utilities**: Base operations and mathematical utilities
- 🦀 **Pure Rust**: Memory-safe, zero-cost abstractions
- 📚 **Well Documented**: Comprehensive API documentation

## 📦 Installation

Add `sophy` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
sophy = "0.1.23"
````

## 🚀 Quick Example

```rust
use sophy::methods::raphson::raphson;

fn main() {
    let f = |x: f64| x * x - 2.0;       // Function: x^2 - 2
    let df = |x: f64| 2.0 * x;          // Derivative: 2x

    let root = raphson(1.0, f, df, 1e-10, 100);
    println!("Root approximation: {:.10}", root); // √2 ≈ 1.4142135624
}
```
```

## 📂 Project Structure

```
sophi
├── Cargo.toml
├── README.md
└── src
    ├── base          // Number utilities
    ├── functions     // Mathematical functions
    ├── methods       // Numerical methods
    └── lib.rs
```

## 🔧 Roadmap

* [ ] Add more numerical methods (e.g., integration, interpolation)
* [ ] Implement linear algebra operations
* [ ] Add probability and statistics functions
* [ ] Improve error handling (return `Result` instead of `panic!`)
* [ ] Publish on [crates.io](https://crates.io/)

## 🤝 Contributions

Contributions are welcome! Feel free to open issues, suggest features, or submit pull requests.

## 📜 License

This project is licensed under the MIT License.

---

Made with ❤️ in Rust.



