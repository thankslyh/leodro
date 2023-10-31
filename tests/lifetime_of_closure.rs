#[cfg(test)]
mod lifetime1_test {
    fn annotate<T, F>(f: F) -> F
    where
        F: for<'a> Fn(&'a T) -> &'a T,
    {
        f
    }
    #[test]
    fn test1() {
        // let c = |x: &usize| x; // early bound
        // let c = |x| x; // late bound
        let c = annotate(|x: &usize| x);
        let a: &usize = &3;
        c(a);
    }
}
