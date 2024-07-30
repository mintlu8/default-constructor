use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{token_stream::IntoIter, Delimiter, Group, Ident, Span, TokenStream, TokenTree};
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;

/// The meta macro.
///
/// # Syntax
///
/// ```
/// # /*
/// meta_default_constructor!(
///     // conversion function
///     [Into::into]
///     // struct name and optional generics
///     MyStruct::<T>
///     // fields
///     {
///         // name value pairs like normal structs
///         //
///         // value is converted via the conversion function
///         // name: Into::into(value),
///         name: value,
///         // use `effect` boxed to do another conversion like boxing, see the `effect` module.
///         boxed: @boxed inner,
///         // Nested structs will be recursively applied this macro
///         // `OtherStruct` will be constructed recursively using the same `meta_default_constructor!`
///         other: OtherStruct {
///             ..
///         },
///         // Ignore this behavior like this.
///         other2: {OtherStruct {
///             ..
///         }},
///         // The `arr` effect uses the same conversion as fields.
///         array: @arr [
///             "Hello", "World!"
///         ],
///         // append [..Default::default()] at the end
///     }
/// )
/// # */
/// ```
#[proc_macro]
#[proc_macro_error]
pub fn meta_default_constructor(tokens: TokenStream1) -> TokenStream1 {
    meta_default_constructor2(tokens.into()).into()
}

fn parse_until_comma(
    stream: &mut IntoIter,
    pfx: impl IntoIterator<Item = TokenTree>,
) -> Vec<TokenTree> {
    let mut result = Vec::from_iter(pfx);
    for tt in stream.by_ref() {
        match tt {
            TokenTree::Punct(p) if p.as_char() == ',' => break,
            _ => result.push(tt),
        }
    }
    result
}

/// Expected expression as an input.
///
/// * `path::to::Struct { .. }`: apply `parse_struct_definition` recursively.
/// * `[a, b, ..]`: apply `.into_iter().collect()`.
///
/// Tuple structs are not parsed since they are identical to functions.
fn transform_field(
    convert_fn: &TokenStream,
    mut expr: Vec<TokenTree>,
    arr: bool,
) -> Vec<TokenTree> {
    match expr.last() {
        Some(TokenTree::Group(g))
            if arr && g.delimiter() == Delimiter::Bracket && expr.len() == 1 =>
        {
            let buf = parse_delimited(convert_fn, g.stream());
            quote! {
                [#buf]
            }
            .into_iter()
            .collect()
        }
        Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace && expr.len() > 1 => {
            let fields = parse_struct_definition(convert_fn, g.stream());
            expr.pop();
            quote! {
                #(#expr)* #fields
            }
            .into_iter()
            .collect()
        }
        _ => expr,
    }
}

fn parse_delimited(convert_fn: &TokenStream, stream: TokenStream) -> TokenStream {
    let mut result = Vec::new();
    let mut iter = stream.into_iter();
    loop {
        match iter.next() {
            Some(TokenTree::Punct(p)) if p.as_char() == '@' => {
                let Some(TokenTree::Ident(mut convert_fn2)) = iter.next() else {
                    abort!(p.span(), "Expected convert function after '@'.")
                };
                if convert_fn2 == "box" {
                    convert_fn2 = Ident::new("boxed", convert_fn2.span())
                }
                let arr = convert_fn2 == "arr";
                let iter = transform_field(convert_fn, parse_until_comma(&mut iter, []), arr);
                result.extend(quote! {{
                    use ::default_constructor::effects::*;
                    #convert_fn2(#convert_fn(#(#iter)*))
                },})
            }
            Some(pfx) => {
                let iter = transform_field(convert_fn, parse_until_comma(&mut iter, [pfx]), false);
                result.extend(quote! {#convert_fn(#(#iter)*),})
            }
            None => break,
        }
    }
    result.into_iter().collect()
}

fn parse_struct_definition(convert_fn: &TokenStream, stream: TokenStream) -> TokenTree {
    let mut result = Vec::new();
    let mut iter = stream.into_iter();
    while let Some(field) = iter.next() {
        iter.next();
        match iter.next() {
            Some(TokenTree::Punct(p)) if p.as_char() == '@' => {
                let Some(TokenTree::Ident(mut convert_fn2)) = iter.next() else {
                    abort!(p.span(), "Expected convert function after '@'.")
                };
                if convert_fn2 == "box" {
                    convert_fn2 = Ident::new("boxed", convert_fn2.span())
                }
                let arr = convert_fn2 == "arr";
                let iter = transform_field(convert_fn, parse_until_comma(&mut iter, []), arr);
                result.extend(quote! {#field: {
                    #convert_fn2(#convert_fn(#(#iter)*))
                },})
            }
            Some(pfx) => {
                let iter = transform_field(convert_fn, parse_until_comma(&mut iter, [pfx]), false);
                result.extend(quote! {#field: #convert_fn(#(#iter)*),})
            }
            None => abort!(Span::call_site(), "Expected field."),
        }
    }
    TokenTree::Group(Group::new(
        Delimiter::Brace,
        quote! {
            #(#result)*
            ..::core::default::Default::default()
        },
    ))
}

fn meta_default_constructor2(tokens: TokenStream) -> TokenStream {
    let mut iter = tokens.into_iter();
    let Some(TokenTree::Group(convert_fn)) = iter.next() else {
        abort!(Span::call_site(), "Missing conversion function.")
    };
    let convert_fn = convert_fn.stream();
    let tokens: Vec<_> = iter.collect();
    let mut turbofish_counter = 0;
    let result: Vec<_> = tokens
        .split(|x| match x {
            TokenTree::Punct(p) if p.as_char() == ',' && turbofish_counter == 0 => true,
            TokenTree::Punct(p) if p.as_char() == '<' => {
                turbofish_counter += 1;
                false
            }
            TokenTree::Punct(p) if p.as_char() == '>' => {
                turbofish_counter -= 1;
                false
            }
            _ => false,
        })
        .filter_map(|segment| {
            match segment {
                // ignore things wrapped in `()` or `{}`
                [TokenTree::Group(g)] => {
                    Some(quote! {#g})
                }
                [tt @ .., TokenTree::Group(g)] if g.delimiter() == Delimiter::Parenthesis => {
                    let block = parse_delimited(&convert_fn, g.stream());
                    Some(quote! {
                        {
                            #[allow(unused_imports)]
                            #[allow(clippy::needless_update)]
                            {
                                #(#tt)* (#block)
                            }
                        }
                    })
                }
                [tt @ .., TokenTree::Group(g)] if g.delimiter() == Delimiter::Brace => {
                    let block = parse_struct_definition(&convert_fn, g.stream());
                    Some(quote! {
                        {
                            #[allow(unused_imports)]
                            #[allow(clippy::needless_update)]
                            {
                                #(#tt)* #block
                            }
                        }
                    })
                }
                [] => None,
                // Assume this is a type
                tt => Some(quote! {
                    <#(#tt)* as ::core::default::Default>::default()
                }),
            }
        })
        .collect();
    quote! {(#(#result),*)}
}
