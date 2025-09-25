//! # Newton-Raphson Method
//!
//! Implementation of the Newton-Raphson method for finding roots of real-valued functions.
//!
//! The Newton-Raphson method is an iterative root-finding algorithm that uses the formula:
//! 
//! **x_{n+1} = x_n - f(x_n) / f'(x_n)**
//!
//! ## Algorithm Properties
//!
//! - **Convergence**: Quadratic convergence when close to the root
//! - **Requirements**: Function must be differentiable and derivative must not be zero
//! - **Sensitivity**: Can fail if derivative is close to zero or initial guess is poor
//!
//! ## Mathematical Background
//!
//! Given a function f(x), we want to find x such that f(x) = 0. Starting with an initial
//! guess x₀, we iteratively apply the Newton-Raphson formula until convergence.

/// Newton-Raphson root-finding method.
///
/// This function finds an approximate root of the equation `f(x) = 0` using the 
/// Newton-Raphson iterative method. The algorithm converges quadratically when
/// the initial guess is sufficiently close to the actual root.
///
/// ## Mathematical Formula
///
/// The method uses the iterative formula:
/// ```text
/// x_{n+1} = x_n - f(x_n) / f'(x_n)
/// ```
///
/// ## Arguments
///
/// * `x` - Initial guess for the root. Choose a value reasonably close to the expected root.
/// * `f` - The function whose root is sought. Must implement `Fn(f64) -> f64`.
/// * `df` - The derivative of function `f`. Must implement `Fn(f64) -> f64`.
/// * `tol` - Convergence tolerance. Algorithm stops when `|x_{n+1} - x_n| < tol`.
/// * `max_iter` - Maximum number of iterations to prevent infinite loops.
///
/// ## Returns
///
/// Returns a `f64` representing the approximated root of the equation `f(x) = 0`.
///
/// ## Panics
///
/// This function will panic if:
/// * The derivative is too close to zero (`|f'(x)| < tol`)
/// * Maximum iterations are reached without convergence
///
/// ## Examples
///
/// ### Finding √2
/// 
/// Find the square root of 2 by solving x² - 2 = 0:
///
/// ```rust
/// use sophy::methods::raphson::raphson;
///
/// let f = |x: f64| x * x - 2.0;          // f(x) = x² - 2
/// let df = |x: f64| 2.0 * x;             // f'(x) = 2x
///
/// let root = raphson(1.0, f, df, 1e-10, 100);
/// assert!((root - std::f64::consts::SQRT_2).abs() < 1e-10);
/// ```
///
/// ### Finding cube root of 27
///
/// Solve x³ - 27 = 0:
///
/// ```rust
/// use sophy::methods::raphson::raphson;
///
/// let f = |x: f64| x.powi(3) - 27.0;     // f(x) = x³ - 27
/// let df = |x: f64| 3.0 * x.powi(2);     // f'(x) = 3x²
///
/// let root = raphson(3.5, f, df, 1e-12, 100);
/// assert!((root - 3.0).abs() < 1e-12);
/// ```
///
/// ### Complex polynomial root
///
/// Find a root of x³ - x - 1 = 0:
///
/// ```rust
/// use sophy::methods::raphson::raphson;
///
/// let f = |x: f64| x.powi(3) - x - 1.0;  // f(x) = x³ - x - 1
/// let df = |x: f64| 3.0 * x.powi(2) - 1.0; // f'(x) = 3x² - 1
///
/// let root = raphson(1.5, f, df, 1e-12, 100);
/// // Verify that f(root) ≈ 0
/// assert!(f(root).abs() < 1e-12);
/// ```
///
/// ## Convergence Notes
///
/// The Newton-Raphson method has quadratic convergence when:
/// - The initial guess is sufficiently close to the root
/// - The function is well-behaved (smooth, derivative doesn't vanish)
/// - The root is simple (multiplicity 1)
///
/// For functions with multiple roots, different initial guesses may converge to different roots.
pub fn raphson<F, DF>(mut x: f64, f: F, df: DF, tol: f64, max_iter: usize) -> f64
where
    F: Fn(f64) -> f64,
    DF: Fn(f64) -> f64,
{
    for _ in 0..max_iter {
        let y = f(x);
        let y_prime = df(x);

        if y_prime.abs() < tol {
            panic!("Derivative too small");
        }

        let x_new = x - y / y_prime;

        if (x_new - x).abs() < tol {
            return x_new;
        }

        x = x_new;
    }

    x
}
