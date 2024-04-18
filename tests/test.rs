use default_constructor::{construct, infer_construct, meta_default_constructor};

#[derive(Default)]
pub struct B {
    a: String,
    b: f32,
    c: char,
    d: Vec<u8>,
    e: A,
    f: Box<A>,
}

#[derive(Default)]
pub struct C<T> {
    a: String,
    b: f32,
    c: char,
    t: T
}

mod a {
    use std::collections::HashMap;

    #[derive(Default)]
    pub struct A{
        pub a: String,
        pub b: f32,
        pub c: char,
        pub d: Vec<u8>,
        pub e: HashMap<Option<f32>, Box<char>>,
        pub f: i32,
    }
}

use a::A;

fn do_thing() {

    let _: A = construct!(
        A {
            a: "hello",
            b: 1i16,
            d: [1, 2, 3, 4]
        }
    );
    let _: A = infer_construct!(
        A {
            a: "hello",
            b: 1i16,
            e: std::collections::HashMap::new(),
            f: false,
        }
    );

    let _: a::A = infer_construct!(
        a::A {
            a: "hello",
            b: 1i16,
            e: ::std::collections::HashMap::new(),
            f: false,
        }
    );

    let _: B = infer_construct!(
        B {
            a: "hello",
            b: 1i16,
            e: A {
                a: "hello",
                b: 1,
                d: [1, 2, 3, 4]
            },
            f: box A {
                b: 1,
                d: [1, 2, 3, 4]
            }
        }
    );

    let _: B = infer_construct!(
        B {
            a: "hello",
            b: 1i16,
            e: A {
                a: "hello",
                b: 1,
                d: [1, 2, 3, 4]
            },
            f: a::A {
                b: 1,
                d: [1, 2, 3, 4]
            }
        }
    );

    // generics work with inference
    let _: C<i32> = infer_construct!(
        C {
            a: "hello",
            b: 1i16,
            t: 69
        }
    );

    // or specify it
    let _ = infer_construct!(
        C[i64] {
            a: "hello",
            b: 1i16,
            t: 69
        }
    );

    meta_default_constructor!(
        {
            use std::str::FromStr;
        }
        [(|x| FromStr::from_str(x).unwrap())]
        A {
            a: "hello",
            b: "1.12",
        }
    );
}

