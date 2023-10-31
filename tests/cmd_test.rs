#[cfg(test)]
mod cmd_test {
    use std::env;

    #[test]
    fn test_args() {
        let args = env::args();
        println!("{:#?}", args);
    }
}
