pub mod functions;
pub mod base;
#[cfg(test)]
mod tests {
    use crate::functions;


    #[test]
    fn it_works() {
        let e = functions::exp::exp(1.0);
        assert_eq!(e,  2.7182818284590455);
    }
}
