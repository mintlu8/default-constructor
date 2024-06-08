use std::{borrow::Cow, marker::PhantomData, rc::Rc, sync::Arc};

use default_constructor::infer_construct;

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
    t: T,
}

#[derive(Default)]
pub struct D<T>(String, f32, char, T);

#[derive(Default)]
pub struct Lifetime<'a> {
    p: PhantomData<&'a ()>,
}
#[derive(Default)]
pub struct ComplexWeirdness<'a, 'b, A, B> {
    p: PhantomData<(&'a A, &'b B)>,
}

#[derive(Default)]
pub struct TestAutoCompletion {
    aaaa: f32,
    aaab: f32,
    aaac: f32,
    bbba: f32,
    bcac: f32,
    bbab: f32,
}

mod a {
    use std::collections::HashMap;

    #[derive(Default)]
    pub struct A {
        pub a: String,
        pub b: f32,
        pub c: char,
        pub d: Vec<u8>,
        pub e: HashMap<Option<f32>, Box<char>>,
        pub f: i32,
        pub g: Vec<String>,
    }
}

use a::A;

fn do_thing() {
    let _: A = infer_construct!(A {
        a: "hello",
        b: 1i16,
        e: std::collections::HashMap::new(),
        f: false,
    });

    let _: a::A = infer_construct!(
        a::A {
            a: "hello",
            b: 1i16,
            e: ::std::collections::HashMap::new(),
            f: false,
            g: @arr ["Hello", "World!"]
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
            f: @box A {
                b: 1,
                d: [1, 2, 3, 4]
            }
        }
    );

    let _: B = infer_construct!(B {
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
    });

    let _: B = infer_construct!(B {
        a: "hello",
        b: 1i16,
        e: {
            A {
                ..Default::default()
            }
        },
        f: a::A {
            b: 1,
            d: [1, 2, 3, 4]
        }
    });

    // generics work with inference
    let _: C<i32> = infer_construct!(C {
        a: "hello",
        b: 1i16,
        t: 69
    });

    // generics work with inference
    let _: D<i32> = infer_construct!(D("hello", 1i16, 69, 4,));

    // or specify it
    let _ = infer_construct!(C::<i64> {
        a: "hello",
        b: 1i16,
        t: 69
    });

    // lifetimes and fields
    let _ = infer_construct!(Lifetime::<'static> {});

    // lifetimes and fields
    let _ = infer_construct!(Lifetime::<'static> {});

    // lifetimes and fields
    let _ = infer_construct!(ComplexWeirdness::<'static, 'static, A, B> {});

    // lifetimes and fields
    let _ = infer_construct!(ComplexWeirdness::<'static, 'static, A, B> {});
}

#[allow(clippy::box_collection)]
#[derive(Default)]
pub struct E {
    a: Box<String>,
    b: Rc<String>,
    c: Arc<String>,
    d: Option<String>,
}

fn test_effect() {
    let _ = infer_construct! {
        E {
            a: @boxed "The",
            b: @rc "Rust",
            c: @arc "Programming".to_owned(),
            d: @some Cow::Borrowed("Language"),
        }
    };
}
