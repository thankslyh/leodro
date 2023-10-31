#[cfg(test)]
mod hrtb {
    struct Closure<F> {
        data: (u8, u16),
        func: F,
    }

    impl<F> Closure<F>
    where
        F: Fn(&(u8, u16)) -> &u8,
    {
        fn call(&self) -> &u8 {
            (self.func)(&self.data)
        }
    }

    fn do_it(data: &(u8, u16)) -> &u8 {
        &data.0
    }

    #[test]
    fn test_example1() {
        let c = Closure {
            data: (1, 2),
            func: do_it,
        };
        println!("{}", c.call())
    }
}
