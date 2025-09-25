//! # Special Functions Example
//!
//! This example demonstrates the special mathematical functions in Sophy.

use sophy::specials::{erf, eta, gamma, sigma, zeta};

fn main() {
    println!("🎯 Sophy Special Functions Demo\n");

    // Gamma Function Examples
    println!("📊 Gamma Function Γ(x):");
    println!("   Γ(1) = {}  (should be 1)", gamma(1.0));
    println!("   Γ(2) = {}  (should be 1)", gamma(2.0));
    println!("   Γ(3) = {}  (should be 2)", gamma(3.0));
    println!("   Γ(4) = {}  (should be 6)", gamma(4.0));
    println!("   Γ(5) = {}  (should be 24)", gamma(5.0));
    println!("   Γ(0.5) = {}  (should be √π ≈ 1.772)", gamma(0.5));
    println!();

    // Riemann Zeta Function Examples
    println!("🔢 Riemann Zeta Function ζ(s):");
    println!("   ζ(2) = {}  (should be π²/6 ≈ 1.6449)", zeta(2.0));
    println!("   ζ(4) = {}  (should be π⁴/90 ≈ 1.0823)", zeta(4.0));
    println!("   ζ(3) = {}  (Apéry's constant)", zeta(3.0));
    println!("   ζ(6) = {}  (should be π⁶/945)", zeta(6.0));
    println!();

    // Error Function Examples
    println!("📈 Error Function erf(x):");
    println!("   erf(0) = {}  (should be 0)", erf(0.0));
    println!("   erf(1) = {}  (≈ 0.8427)", erf(1.0));
    println!("   erf(-1) = {}  (should be -erf(1))", erf(-1.0));
    println!("   erf(2) = {}  (≈ 0.9953)", erf(2.0));
    println!("   erf(∞) ≈ erf(5) = {}  (approaches 1)", erf(5.0));
    println!();

    // Dirichlet Eta Function Examples
    println!("🌊 Dirichlet Eta Function η(s):");
    println!("   η(1) = {}  (should be ln(2) ≈ 0.6931)", eta(1.0));
    println!("   η(2) = {}  (should be π²/12 ≈ 0.8225)", eta(2.0));
    println!("   η(3) = {}  (≈ 0.9015)", eta(3.0));
    println!();

    // Sum of Divisors Function Examples
    println!("➕ Sum of Divisors σ(n):");
    println!("   σ(1) = {}  (divisors: 1)", sigma(1));
    println!("   σ(6) = {}  (divisors: 1,2,3,6)", sigma(6));
    println!("   σ(12) = {}  (divisors: 1,2,3,4,6,12)", sigma(12));
    println!("   σ(28) = {}  (perfect number)", sigma(28));
    println!("   σ(100) = {}  (divisors sum)", sigma(100));
    println!();

    // Mathematical Relationships
    println!("🔗 Mathematical Relationships:");

    // Verify Γ(n) = (n-1)!
    println!("   Factorial relationship: Γ(6) = 5! = {}", gamma(6.0));

    // Basel problem: ζ(2) = π²/6
    let basel_exact = std::f64::consts::PI.powi(2) / 6.0;
    let basel_computed = zeta(2.0);
    println!("   Basel problem: ζ(2) = π²/6");
    println!("     Exact: {:.10}", basel_exact);
    println!("     Computed: {:.10}", basel_computed);
    println!("     Error: {:.2e}", (basel_exact - basel_computed).abs());

    // Error function symmetry: erf(-x) = -erf(x)
    let x = 1.5;
    println!(
        "   Error function symmetry: erf(-{}) + erf({}) = {:.2e} (should be 0)",
        x,
        x,
        erf(-x) + erf(x)
    );

    // Perfect numbers
    println!("   Perfect numbers (σ(n) = 2n):");
    for n in [6, 28, 496] {
        let div_sum = sigma(n);
        println!(
            "     n={}, σ(n)={}, 2n={}, perfect: {}",
            n,
            div_sum,
            2 * n,
            div_sum == 2 * n
        );
    }

    println!("\n✨ Special functions demonstration complete!");
}
