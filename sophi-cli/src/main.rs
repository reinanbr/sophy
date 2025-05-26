
use sophi::methods::raphson;


fn main() {
    let f = |x: f64| x * x - 2.0;
    let df = |x: f64| 2.0 * x;
    let root = raphson(1.0, f, df, 1e-10, 100);

    println!("Root of x^2 - 2 = {root}");

    assert!((root - std::f64::consts::SQRT_2).abs() < 1e-10);
}

