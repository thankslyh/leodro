#[cfg(test)]
mod tests {
    use std::env;
    #[derive(Debug)]
    #[cfg_attr(not(test), rustc_diagnostic_item = "AsMut")]
    struct User {
        id: i8,
        name: String
    }

    impl PartialEq for User {
        fn eq(&self, other: &Self) -> bool {
            self.id.eq(&other.id)
        }
    }

    // impl<T> AsMut<[T]> for User {
    //     fn as_mut(&mut self) -> &mut [T] {
    //         self
    //     }
    // }
    #[test]
    fn test_ownership() {
        let a = 1;
        let _b = a;
        let a_str = "123";
        let _b_str = a_str;
        let mut c = &1;
        let d = c;
        c = &2;
        println!("{}", c);
        println!("{}", a);
        println!("{}", a_str);
    }

    #[test]
    fn test_dedup() {
        let mut int_vec = vec![1, 1, 2, 3, 3, 4];
        int_vec.dedup();
        println!("{:?}", int_vec);

        let mut struct_vec = Vec::new();
        struct_vec.push(User {
            id: 1,
            name: String::new()
        });
        struct_vec.push(User {
            id: 2,
            name: String::new()
        });
        struct_vec.push(User {
            id: 3,
            name: String::new()
        });
        struct_vec.push(User {
            id: 3,
            name: String::new()
        });
        struct_vec.push(User {
            id: 2,
            name: String::new()
        });
        struct_vec.push(User {
            id: 10,
            name: String::new()
        });
        struct_vec.dedup();
        println!("{:#?}", struct_vec)
    }

    #[test]
    fn test_as_mut() {
        let mut a = vec![1, 2, 3];
        let b = &mut a.clone();
        b.push(4);
        println!("{:?}", a);
        println!("{:?}", b);
        // let mut a: Vec<i32> = Vec::new();
        // a.push(1);
        // let b = a.as_mut();
        let mut a1 = vec![User{id: 1, name: String::new()}];
        let b1 = &mut a1;
        b1.push(User{
            id: 2,
            name: String::new()
        });
        println!(":? {:?}", b1);
    }
}