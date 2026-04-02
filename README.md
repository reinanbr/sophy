
# sophy

sophy is a lightweight, efficient, and extensible math library written in pure Rust.  
It provides mathematical functions, numerical methods, and number utilities for scientific computing, educational tools, or general-purpose applications


- ✅ Mathematical functions (e.g., exponential)
- ✅ Numerical methods (e.g., Newton-Raphson root finder)
- ✅ Number utilities and base operations
- ✅ Written in safe, idiomatic Rust


## 📦 Installation

Add `sophy` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
sophy = { path = "./sophy" }
````

*(Published version coming soon)*

## 🚀 Usage Example

### Newton-Raphson Method

```rust
use sophy::methods::raphson::raphson;

fn main() {
    let f = |x: f64| x * x - 2.0;       // Function: x^2 - 2
    let df = |x: f64| 2.0 * x;          // Derivative: 2x

    let root = raphson(1.0, f, df, 1e-10, 100);
    println!("Root approximation: {}", root);
}
```

### Exponential Function

```rust
use sophy::functions::exp::exp;

fn main() {
    let result = exp(1.0);
    println!("exp(1) ≈ {}", result); // Should be close to 2.71828...
}
```

## 📂 Project Structure

```
sophy
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



