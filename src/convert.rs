use core::array::from_fn;

/// [`Into`] with relaxed orphan rule, you can define non-owned
/// conversion with a owned `Marker`. Keep in mind this inference
/// will fail if multiple conversion paths are found.
pub trait InferInto<A, Marker>: Sized {
    fn into(self) -> A;
}

impl<T, U> InferInto<U, ()> for T where T: Into<U> {
    fn into(self) -> U {
        Into::<U>::into(self)
    }
}

/// Convert via [`InferInto`]
pub fn infer_into<T, U, M>(item: T) -> U where T: InferInto<U, M> {
    InferInto::into(item)
}

/// Provides conversion from integer literal `i32` to other numerical types.
pub trait StandardConverters<F> {
    fn into(self) -> F;
}

impl<T, F> InferInto<F, bool> for T where T: StandardConverters<F> {
    fn into(self) -> F {
        StandardConverters::<F>::into(self)
    }
}

impl StandardConverters<f64> for i64 {
    fn into(self) -> f64 {
        self as f64
    }
}

macro_rules! std_convert {
    ($($ty: ty),*) => {
        $(
            impl StandardConverters<$ty> for i32 {
                fn into(self) -> $ty {
                    self as $ty
                }
            }
    
            impl<const N: usize> StandardConverters<[$ty; N]> for [i32; N] {
                fn into(self) -> [$ty; N] {
                    from_fn(|i| self[i] as $ty)
                }
            }
        )*
    };
}

std_convert!(u8, u16, u32, u64, i8, i16, f32);