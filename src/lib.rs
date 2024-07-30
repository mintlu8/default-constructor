#![doc = include_str!("../README.md")]
#![no_std]
mod convert;
pub use convert::{infer_into, InferInto, StandardConverters};
pub use default_constructor_macros::meta_default_constructor;
/// Standard intermediate functions that can be called via the `@name` syntax.
///
/// This enables conversion chains like `&str --into--> String --@some--> Option<String>`.
pub mod effects {
    extern crate alloc;

    /// Construct a `Box`.
    #[cfg(feature = "std")]
    pub fn boxed<T>(item: T) -> alloc::boxed::Box<T> {
        alloc::boxed::Box::new(item)
    }

    /// Construct an `Rc`.
    #[cfg(feature = "std")]
    pub fn rc<T>(item: T) -> alloc::rc::Rc<T> {
        alloc::rc::Rc::new(item)
    }

    /// Construct an `Arc`.
    #[cfg(feature = "std")]
    pub fn arc<T>(item: T) -> alloc::sync::Arc<T> {
        alloc::sync::Arc::new(item)
    }

    /// Construct a `Cow`.
    #[cfg(feature = "std")]
    pub fn cow<T: Clone>(item: T) -> alloc::borrow::Cow<'static, T> {
        alloc::borrow::Cow::Owned(item)
    }

    /// Construct a `Some`.
    pub fn some<T>(item: T) -> Option<T> {
        Option::Some(item)
    }

    /// Construct an iterator from an array.
    ///
    /// This is magic since this also enables macro recursion on array literals.
    ///
    /// `[a, b, c] -> [a.into(), b.into(), c.into()].into_iter().collect()`.
    pub fn arr<T, I: IntoIterator<Item = T> + FromIterator<T>, const N: usize>(item: [T; N]) -> I {
        item.into_iter().collect()
    }
}

/// A standard constructor that uses [`Into`].
///
/// # Syntax
///
/// ```
/// # /*
/// construct! {
///     Student {
///         name: "Timmy",
///         age: 10,
///         father : {
///             name: "Tommy",
///             age: 35,
///         }
///     }
/// }
/// # */
/// ```
#[macro_export]
macro_rules! construct {
    ($($tt: tt)*) => {
        {
            use $crate::effects::*;
            $crate::meta_default_constructor! {
                [::std::convert::Into::into]
                $($tt)*
            }
        }
    };
}

/// A standard constructor that uses [`InferInto`].
///
/// [`InferInto`] is inference based and will fail if multiple implementations
/// of the same conversion exists.
///
/// # Syntax
///
/// ```
/// # /*
/// infer_construct! {
///     Student {
///         name: "Timmy",
///         age: 10,
///         father : {
///             name: "Tommy",
///             age: 35,
///         }
///     }
/// }
/// # */
/// ```
#[macro_export]
macro_rules! infer_construct {
    ($($tt: tt)*) => {
        {
            use $crate::effects::*;
            $crate::meta_default_constructor! {
                [$crate::infer_into]
                $($tt)*
            }
        }
    };
}
