
use sophi::methods::raphson;


fn main() {
    
    let f = |x: f64| x * x - 2.0; // Function: x^2 - 2
    let df = |x: f64| 2.0 * x; // Derivative: 2x
    let root = raphson(1.0, &f, &df, 1e-10, 100);
    println!("Root of x^2 - 2 = {}", root);
    assert!((root - 1.4142135623730951).abs() < 1e-10); // sqrt(2) is approximately 1.4142135623730951
    println!("Test passed!");
}
