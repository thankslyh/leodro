#[cfg(test)]
mod latebount_earlybound_test {
    use std::fmt::Debug;

    fn m<'a>() {}
    #[derive(Debug)]
    struct A<T: Debug> {
        _a: T,
    }

    struct Buffer {
        buf: Vec<u8>,
        pos: usize,
    }

    impl Buffer {
        fn new() -> Self {
            Buffer {
                buf: vec![1, 2, 3, 4, 5, 6],
                pos: 0,
            }
        }

        fn read_bytes(&mut self) -> &[u8] {
            self.pos += 3;
            &self.buf[self.pos - 3..self.pos]
        }
    }

    struct Buffer2<'b> {
        pos: usize,
        buf: &'b Vec<u8>,
    }
    // 异常
    // late bound 生命周期晚限定，与 buffer new 的时候生命周期一致，所以会出现两次可变借用的问题
    // impl<'a> Buffer2<'a> {
    //     fn new(buf: &'a Vec<u8>) -> Self {
    //         Buffer2 { pos: 0, buf }
    //     }
    //
    //     fn read_bytes(&'a mut self) -> &'a [u8] {
    //         self.pos += 3;
    //         &self.buf[self.pos - 3..self.pos]
    //     }
    // }
    // 正常
    // 早限定，调用 read_bytes时确定生命周期，这样就只会出现一次可变借用，最终是为了保证不出现悬垂指针
    impl<'a, 'b: 'a> Buffer2<'b> {
        fn new(buf: &'b Vec<u8>) -> Self {
            Buffer2 { pos: 0, buf }
        }

        fn read_bytes(&'a mut self) -> &'b [u8] {
            self.pos += 3;
            &self.buf[self.pos - 3..self.pos]
        }
    }
    #[test]
    fn test1() {
        let _m1 = m;
        let _m2 = m;
    }

    #[test]
    fn test2() {
        let a: A<i32> = A { _a: 1 };
        println!("{:?}", a)
    }
    fn print(b1: &[u8], b2: &[u8]) {
        println!("{:#?} {:#?}", b1, b2)
    }

    #[test]
    fn buf_test() {
        let mut buf = Buffer::new();
        let read = Buffer::read_bytes;
        // let b1 = buf.read_bytes();
        // println!("{:?}", b1)
        let b1 = read(&mut buf);
        println!("1 {:?}", b1);

        let vec1 = vec![1, 2, 3, 4, 5, 6];
        let mut buf2 = Buffer2::new(&vec1);
        let res1 = buf2.read_bytes();
        let res2 = buf2.read_bytes(); // no work 此时是 late bound
        print(res1, res2);
    }
}
