//! # Sophy Examples
//!
//! This example demonstrates various uses of the Sophy mathematical library.

use sophy::base::numbers::{EPSILON, EULER, PHI, PI};
use sophy::methods::raphson::raphson;

fn main() {
    println!("🧮 Sophy Mathematical Library Examples\n");

    // Example 1: Finding square root using Newton-Raphson
    println!("📐 Example 1: Finding √2 using Newton-Raphson");
    let f = |x: f64| x * x - 2.0;
    let df = |x: f64| 2.0 * x;
    let sqrt_2 = raphson(1.0, f, df, 1e-12, 100);
    println!("   √2 ≈ {:.12}", sqrt_2);
    println!("   Error: {:.2e}\n", (sqrt_2 - 2.0_f64.sqrt()).abs());

    // Example 2: Finding cube root of 27
    println!("📐 Example 2: Finding ∛27 using Newton-Raphson");
    let f = |x: f64| x.powi(3) - 27.0;
    let df = |x: f64| 3.0 * x.powi(2);
    let cbrt_27 = raphson(3.5, f, df, 1e-12, 100);
    println!("   ∛27 ≈ {:.12}", cbrt_27);
    println!("   Error: {:.2e}\n", (cbrt_27 - 3.0).abs());

    // Example 3: Finding root of a polynomial
    println!("📐 Example 3: Finding root of x³ - x - 1 = 0");
    let f = |x: f64| x.powi(3) - x - 1.0;
    let df = |x: f64| 3.0 * x.powi(2) - 1.0;
    let poly_root = raphson(1.5, f, df, 1e-12, 100);
    println!("   Root ≈ {:.12}", poly_root);
    println!("   f(root) = {:.2e}\n", f(poly_root));

    // Example 4: Mathematical constants
    println!("📊 Mathematical Constants:");
    println!("   π = {:.12}", PI);
    println!("   e = {:.12}", EULER);
    println!("   φ (Golden Ratio) = {:.12}", PHI);
    println!("   Machine Epsilon = {:.2e}\n", EPSILON);

    // Example 5: Golden ratio property verification
    println!("🔍 Golden Ratio Property: φ² = φ + 1");
    let phi_squared = PHI * PHI;
    let phi_plus_one = PHI + 1.0;
    println!("   φ² = {:.12}", phi_squared);
    println!("   φ + 1 = {:.12}", phi_plus_one);
    println!(
        "   Difference: {:.2e}\n",
        (phi_squared - phi_plus_one).abs()
    );

    println!("✨ All examples completed successfully!");
}
