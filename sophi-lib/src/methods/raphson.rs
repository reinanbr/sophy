/// Newton-Raphson root-finding method.
///
/// This function finds an approximate root of the equation `f(x) = 0`
/// using the Newton-Raphson iterative method.
///
/// # Arguments
///
/// * `x` - Initial guess for the root.
/// * `f` - The function whose root is sought. Must be a function or closure of type `Fn(f64) -> f64`.
/// * `df` - The derivative of the function `f`. Same signature as `f`.
/// * `tol` - Tolerance for convergence. The method stops when the change between iterations is less than this.
/// * `max_iter` - Maximum number of iterations.
///
/// # Returns
///
/// * A `f64` representing the approximated root.
///
/// # Panics
///
/// * If the derivative is too close to zero (singularity or flat slope).
///
/// # Example
///
/// ```
/// use sophi::methods::raphson::raphson;
///
/// let f = |x: f64| x * x - 2.0;
/// let df = |x: f64| 2.0 * x;
///
/// let root = raphson(1.0, f, df, 1e-10, 100);
/// assert!((root - 1.414213).abs() < 1e-6);
/// ```
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
