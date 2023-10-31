fn return_str() -> String {
    let mut s = "hello".to_string();
    for _i in 0..3 {
        s.push_str("world");
    }
    s.clone()
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn f<'a>() {} // late bound
              // f::<'static>() as fn() // 错误，因为是 late bound，不需要指明生命周期参数
fn g<'a: 'a>() {} // early bound 因为生命周期泛型有了限制，符合 early bound规则

trait Trait {
    fn f(&self) {}
}

impl<T> Trait for fn(T) {
    fn f(&self) {
        println!("1")
    }
}

impl<T> Trait for fn(&T) {
    fn f(&self) {
        println!("2")
    }
}

#[cfg(test)]
mod lifetime1_test {
    use crate::g;
    use crate::longest;
    use crate::return_str;
    use crate::{f, Trait};
    use std::collections::HashSet;

    #[test]
    fn test1() {
        let str = return_str();
        println!("{}", str)
    }
    #[test]
    fn longest_test() {
        // let a = "hello";
        // {
        //     let b = "C";
        //     println!("{}", longest(a, b))
        // }
        let s1 = String::from("Rust");
        let s1_r = &s1;
        {
            let s2 = String::from("C");
            let res = longest(s1_r, &s2);
            println!("{} is the longest", res);
        }
    }

    #[test]
    fn bound_test() {
        let pf = f as fn();
        let pg = g::<'static> as fn();
    }

    #[test]
    fn t_test() {
        let f1: fn(_) = |_: u32| {};
        let f2: fn(_) = |_: &u32| {};
        let f3: fn(&_) = |_: &u32| {};
        f1.f();
        f2.f();
        f3.f();
    }

    #[test]
    fn hash_test() {
        let hello = "hello".to_owned();
        let mut items = HashSet::new();
        items.insert(hello.as_str());
        let mut global_set = HashSet::new();
        global_set.insert(hello.as_str());
        println!("{:#?}", items);
        println!("{:#?}", global_set);
        while !global_set.is_empty() {
            let mut tmp_set = HashSet::new();
            for &item in global_set.iter() {
                let copy = item.to_owned();
                let copy_str = copy.as_str();
                if let Some(inner) = items.get(copy_str).cloned() {
                    tmp_set.insert(inner);
                }
            }
            std::mem::swap(&mut items, &mut tmp_set);
        }
    }
}
