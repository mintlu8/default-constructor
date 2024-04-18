
#![doc = include_str!("../README.md")]
#![no_std]
mod convert;
pub use convert::{InferInto, infer_into, StandardConverters};

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
///         // name value pairs like normal
///         name: value,
///         // value is converted via the conversion function
///         name: value,
///         // OtherStruct will be constructed using `meta_default_constructor!`
///         // use another syntax like wrapping it in parenthesis to ignore this
///         name: OtherStruct {
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
            $crate::meta_default_constructor!($($tt)*)
        }
    };

    // Pad output
    (
        [$func: expr]
        ::$($ty: ident)::*
        $([$($generics: tt)*])?
        {$($tt: tt)*}
    ) => {
        $crate::meta_default_constructor!(
            [$func]
            [::$($ty)::*]
            $([$($generics)*])?
            {$($tt)*} 
            {} 
        )
    };

    // Pad output
    (
        [$func: expr]
        $($ty: ident)::*
        $([$($generics: tt)*])?
        {$($tt: tt)*}
    ) => {
        $crate::meta_default_constructor!(
            [$func]
            [$($ty)::*]
            $([$($generics)*])?
            {$($tt)*} 
            {} 
        )
    };

    // special handle blocks, since empty path is matched later
    (
        [$func: expr]
        [$($ty: tt)*]
        $([$($generics: tt)*])?
        {$field: ident: $block: block $(, $($tt: tt)*)?} 
        {$($out_field: ident: $out_expr: expr),*} 
    ) => {
        $crate::meta_default_constructor!(
            [$func]
            [$($ty)*]
            $([$($generics)*])?
            {$($($tt)*)?} 
            {$($out_field: $out_expr,)* $field: $block} 
        )
    };

    // Nested structs
    (
        [$func: expr]
        [$($ty: tt)*]
        $([$($generics: tt)*])?
        {$field: ident: ::$($ty2: ident)::* {$($fields: tt)*} $(, $($tt: tt)*)?} 
        {$($out_field: ident: $out_expr: expr),*} 
    ) => {
        $crate::meta_default_constructor!(
            [$func]
            [$($ty)*]
            $([$($generics)*])?
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
        [$func: expr]
        [$($ty: tt)*]
        $([$($generics: tt)*])?
        {$field: ident: $($ty2: ident)::* {$($fields: tt)*} $(, $($tt: tt)*)?} 
        {$($out_field: ident: $out_expr: expr),*} 
    ) => {
        $crate::meta_default_constructor!(
            [$func]
            [$($ty)*]
            $([$($generics)*])?
            {$($($tt)*)?} 
            {$($out_field: $out_expr,)* $field: $crate::meta_default_constructor!(
                [$func]
                [$($ty2)::*]
                {$($fields)*}
                {}
            )} 
        )
    };

    // Supports the box syntax
    (
        [$func: expr]
        [$($ty: tt)*]
        $([$($generics: tt)*])?
        {$field: ident: box ::$($ty2: ident)::* {$($fields: tt)*} $(, $($tt: tt)*)?} 
        {$($out_field: ident: $out_expr: expr),*} 
    ) => {
        $crate::meta_default_constructor!(
            [$func]
            [$($ty)*]
            $([$($generics)*])?
            {$($($tt)*)?} 
            {$($out_field: $out_expr,)* $field: 
                ::std::boxed::Box::new(
                    $crate::meta_default_constructor!(
                        [$func]
                        [::$($ty2)::*]
                        {$($fields)*}
                        {}
                    )
                )
            } 
        )
    };

    // Supports the box syntax
    (
        [$func: expr]
        [$($ty: tt)*]
        $([$($generics: tt)*])?
        {$field: ident: box $($ty2: ident)::* {$($fields: tt)*} $(, $($tt: tt)*)?} 
        {$($out_field: ident: $out_expr: expr),*} 
    ) => {
        $crate::meta_default_constructor!(
            [$func]
            [$($ty)*]
            $([$($generics)*])?
            {$($($tt)*)?} 
            {$($out_field: $out_expr,)* $field: 
                ::std::boxed::Box::new(
                    $crate::meta_default_constructor!(
                        [$func]
                        [$($ty2)::*]
                        {$($fields)*}
                        {}
                    )
                )
            } 
        )
    };

    // Boxed normal expressions
    (
        [$func: expr]
        [$($ty: tt)*]
        $([$($generics: tt)*])?
        {$field: ident: box $expr: expr $(, $($tt: tt)*)?} 
        {$($out_field: ident: $out_expr: expr),*} 
    ) => {
        $crate::meta_default_constructor!(
            [$func]
            [$($ty)*]
            $([$($generics)*])?
            {$($($tt)*)?} 
            {$($out_field: $out_expr,)* $field: 
                ::std::boxed::Box::new($expr)
            }
        )
    };

    // Normal expressions
    (
        [$func: expr]
        [$($ty: tt)*]
        $([$($generics: tt)*])?
        {$field: ident: $expr: expr $(, $($tt: tt)*)?} 
        {$($out_field: ident: $out_expr: expr),*} 
    ) => {
        $crate::meta_default_constructor!(
            [$func]
            [$($ty)*]
            $([$($generics)*])?
            {$($($tt)*)?} 
            {$($out_field: $out_expr,)* $field: $expr} 
        )
    };

    // generate result
    (
        [$func: expr]
        [$($ty: tt)*]
        $([$($generics: tt)*])?
        {}
        {$($field: ident: $expr: expr),*} 
    ) => {
        $($ty)* $(::<$($generics)*>)? {
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
            [$crate::infer_into]
            $($tt)*
        }
    };
}

