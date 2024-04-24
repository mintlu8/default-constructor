
#![doc = include_str!("../README.md")]
#![no_std]
mod convert;
pub use convert::{InferInto, infer_into, StandardConverters};

/// Standard intermediate functions that can be called via the `@name` syntax.
/// 
/// This enables conversion chains like `&str --into--> String --@some--> Option<String>`.
pub mod effects {
    extern crate alloc;

    /// Construct a `Box`.
    #[cfg(feature="std")]
    pub fn boxed<T>(item: T) -> alloc::boxed::Box<T> {
        alloc::boxed::Box::new(item)
    }

    /// Construct an `Rc`.
    #[cfg(feature="std")]
    pub fn rc<T>(item: T) -> alloc::rc::Rc<T> {
        alloc::rc::Rc::new(item)
    }

    /// Construct an `Arc`.
    #[cfg(feature="std")]
    pub fn arc<T>(item: T) -> alloc::sync::Arc<T> {
        alloc::sync::Arc::new(item)
    }

    /// Construct a `Cow`.
    #[cfg(feature="std")]
    pub fn cow<T: Clone>(item: T) -> alloc::borrow::Cow<'static, T> {
        alloc::borrow::Cow::Owned(item)
    }

    /// Construct a `Some`.
    pub fn some<T>(item: T) -> Option<T> {
        Option::Some(item)
    }
}

/// Add another conversion operation on top of the standard `Into`,
/// this can either be a function in scope or a function in the [`effects`] module.
#[macro_export]
macro_rules! effect {
    (box, $expr: expr) => {
        ::std::boxed::Box::new($expr)
    };
    ($ident: ident, $expr: expr) => {
        {
            use $crate::effects::*;
            $ident($expr)
        }
    };
}

/// The meta macro.
/// 
/// # Syntax
/// 
/// ```
/// # /*
/// meta_default_constructor!(
///     // scoped imports, optional, must be in braces `{..}`
///     {
///         import std::future::Future;
///         import rand::prelude::*;
///     }
///     // conversion function, required
///     [Into::into]
///     // struct name, required
///     MyStruct
///     // generics, optional, must be in brackets `[..]`
///     // this is equivalent to specifying `::<f32, String>`
///     [f32, String]
///     // fields
///     {
///         // name value pairs like normal structs
///         //
///         // value is converted via the conversion function
///         // name: Into::into(value),
///         name: value,
///         // use `effect` boxed to do another conversion, see the `effect!` macro.
///         boxed: @boxed inner,
///         // Nested structs will be recursively applied this macro
///         // `OtherStruct` will be constructed using the same `meta_default_constructor!`
///         // use another syntax like wrapping it in parenthesis to ignore this
///         other: OtherStruct {
///             ..
///         },
///         // append [..Default::default()] at the end
///     }
/// )
/// # */
/// ```
#[macro_export]
macro_rules! meta_default_constructor {

    // Extract imports.
    (
        {$($imports: stmt);* $(;)?}
        $($tt: tt)*
    ) => {
        {
            $($imports;)*
            $crate::meta_default_constructor!(@ty $($tt)*)
        }
    };

    // Parse struct path
    (
        @ty 
        [$func: expr]
        ::$($ty: ident)::*
        ::< $($tt: tt)*
    ) => {
        $crate::meta_default_constructor!(
            @generics
            [$func]
            [::$($ty)::*]
            []
            ::< $($tt)*
        )
    };

    // Parse struct path
    (
        @ty 
        [$func: expr]
        ::$($ty: ident)::*
        {$($tt: tt)*}
    ) => {
        $crate::meta_default_constructor!(
            @fields
            [$func]
            [::$($ty)::*]
            []
            {$($tt)*}{}
        )
    };

    // Parse struct path
    (
        @ty 
        [$func: expr]
        $($ty: ident)::*
        ::< $($tt: tt)*
    ) => {
        $crate::meta_default_constructor!(
            @generics
            [$func]
            [$($ty)::*]
            []
            ::< $($tt)*
        )
    };


    // Parse struct path
    (
        @ty 
        [$func: expr]
        $($ty: ident)::*
        {$($tt: tt)*}
    ) => {
        $crate::meta_default_constructor!(
            @fields
            [$func]
            [$($ty)::*]
            []
            {$($tt)*}{}
        )
    };

    // parse generics
    (
        @generics
        [$func: expr]
        [$($ty: tt)*]
        [$($generics: tt)*]
        ::<$lt: lifetime, $($tt: tt)*
        
    ) => {
        $crate::meta_default_constructor!(
            @generics
            [$func]
            [$($ty)*]
            [$($generics)* $lt,]
            ::<
            $($tt)*
        )
    };

    // parse generics
    (
        @generics
        [$func: expr]
        [$($ty: tt)*]
        [$($generics: tt)*]
        ::<$lt: lifetime> $($tt: tt)*
        
    ) => {
        $crate::meta_default_constructor!(
            @fields
            [$func]
            [$($ty)*]
            [$($generics)* $lt]
            $($tt)* {}
        )
    };

    // parse generics
    (
        @generics
        [$func: expr]
        [$($ty: tt)*]
        [$($generics: tt)*]
        ::<$($gty: ty),* $(,)?>
        $($tt: tt)*
    ) => {
        $crate::meta_default_constructor!(
            @fields
            [$func]
            [$($ty)*]
            [$($generics)* $($gty),*]
            $($tt)* {}
        )
    };

    // special handle blocks, since empty path is matched later
    (
        @fields
        [$func: expr]
        [$($ty: tt)*]
        [$($generics: tt)*]
        {$field: ident: $block: block $(, $($tt: tt)*)?} 
        {$($out_field: ident: $out_expr: expr),*} 
    ) => {
        $crate::meta_default_constructor!(
            @fields
            [$func]
            [$($ty)*]
            [$($generics)*]
            {$($($tt)*)?} 
            {$($out_field: $out_expr,)* $field: $block} 
        )
    };

    // Nested structs
    (
        @fields
        [$func: expr]
        [$($ty: tt)*]
        [$($generics: tt)*]
        {$field: ident: ::$($ty2: ident)::* {$($fields: tt)*} $(, $($tt: tt)*)?} 
        {$($out_field: ident: $out_expr: expr),*} 
    ) => {
        $crate::meta_default_constructor!(
            @fields
            [$func]
            [$($ty)*]
            [$($generics)*]
            {$($($tt)*)?} 
            {$($out_field: $out_expr,)* $field: $crate::meta_default_constructor!(
                [$func]
                [::$($ty2)::*]
                {$($fields)*}
                {}
            )} 
        )
    };

    // Nested structs
    (
        @fields
        [$func: expr]
        [$($ty: tt)*]
        [$($generics: tt)*]
        {$field: ident: $($ty2: ident)::* {$($fields: tt)*} $(, $($tt: tt)*)?} 
        {$($out_field: ident: $out_expr: expr),*} 
    ) => {
        $crate::meta_default_constructor!(
            @fields
            [$func]
            [$($ty)*]
            [$($generics)*]
            {$($($tt)*)?} 
            {$($out_field: $out_expr,)* $field: $crate::meta_default_constructor!(
                @fields
                [$func]
                [$($ty2)::*][]
                {$($fields)*}
                {}
            )} 
        )
    };

    // Nested structs + effect
    (
        @fields
        [$func: expr]
        [$($ty: tt)*]
        [$($generics: tt)*]
        {$field: ident: @$effect:ident ::$($ty2: ident)::* {$($fields: tt)*} $(, $($tt: tt)*)?} 
        {$($out_field: ident: $out_expr: expr),*} 
    ) => {
        $crate::meta_default_constructor!(
            @fields
            [$func]
            [$($ty)*]
            [$($generics)*]
            {$($($tt)*)?} 
            {$($out_field: $out_expr,)* $field: 
                $crate::effect!{
                    $effect,
                    ($func)($crate::meta_default_constructor!(
                        [$func]
                        [::$($ty2)::*][]
                        {$($fields)*}
                        {}
                    ))
                }
            } 
        )
    };

    // Nested structs + effect
    (
        @fields
        [$func: expr]
        [$($ty: tt)*]
        [$($generics: tt)*]
        {$field: ident: @$effect:ident $($ty2: ident)::* {$($fields: tt)*} $(, $($tt: tt)*)?} 
        {$($out_field: ident: $out_expr: expr),*} 
    ) => {
        $crate::meta_default_constructor!(
            @fields
            [$func]
            [$($ty)*]
            [$($generics)*]
            {$($($tt)*)?} 
            {$($out_field: $out_expr,)* $field: 
                $crate::effect!{
                    $effect,
                    ($func)($crate::meta_default_constructor!(
                        @fields
                        [$func]
                        [$($ty2)::*][]
                        {$($fields)*}
                        {}
                    ))
                }
            } 
        )
    };

    // Normal expressions + effect
    (
        @fields
        [$func: expr]
        [$($ty: tt)*]
        [$($generics: tt)*]
        {$field: ident: @$effect:ident $expr: expr $(, $($tt: tt)*)?} 
        {$($out_field: ident: $out_expr: expr),*} 
    ) => {
        $crate::meta_default_constructor!(
            @fields
            [$func]
            [$($ty)*]
            [$($generics)*]
            {$($($tt)*)?} 
            {$($out_field: $out_expr,)* $field: 
                $crate::effect!{
                    $effect,
                    ($func)($expr)
                }
            }
        )
    };

    // Normal expressions
    (
        @fields
        [$func: expr]
        [$($ty: tt)*]
        [$($generics: tt)*]
        {$field: ident: $expr: expr $(, $($tt: tt)*)?} 
        {$($out_field: ident: $out_expr: expr),*} 
    ) => {
        $crate::meta_default_constructor!(
            @fields
            [$func]
            [$($ty)*]
            [$($generics)*]
            {$($($tt)*)?} 
            {$($out_field: $out_expr,)* $field: $expr} 
        )
    };

    // generate result without generics
    (
        @fields
        [$func: expr]
        [$($ty: tt)*]
        []
        {}
        {$($field: ident: $expr: expr),*} 
    ) => {
        #[allow(clippy::needless_update)]
        $($ty)* {
            $($field: ($func)($expr),)*
            ..core::default::Default::default()
        }
    };

    // generate result with generics
    (
        @fields
        [$func: expr]
        [$($ty: tt)*]
        [$($generics: tt)*]
        {}
        {$($field: ident: $expr: expr),*} 
    ) => {
        #[allow(clippy::needless_update)]
        $($ty)* ::<$($generics)*> {
            $($field: ($func)($expr),)*
            ..core::default::Default::default()
        }
    };
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
        $crate::meta_default_constructor! {
            {}
            [::std::convert::Into::into]
            $($tt)*
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
        $crate::meta_default_constructor! {
            {}
            [$crate::infer_into]
            $($tt)*
        }
    };
}

