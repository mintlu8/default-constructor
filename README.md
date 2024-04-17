# default-constructor

Macros for creating pseudo-dsls that constructs structs through
default construction and field conversion.

## Motivation

This crates is primary designed for `bevy` where constructing large bundles
with `..Default::default()` is common.

## Syntax

Take `construct!` that uses `Into`.

```rust
construct! {
    Student {
        name: "Timmy",
        age: 10,
    }
}
```

This converts to

```rust
Student {
    name: Into::into("Timmy"),
    age: Into::into(10.into()),
    ..Default::default()
}
```

## Meta Constructor

The meta constructor macro allows you to define your own macro with
custom configurations.
See documentation on `meta_default_constructor!` for detail.

## `InferInto`

InferInto allows the user to bypass the orphan rule to create conversions.

By default we provide `i32` -> `f32` and `i64` -> `f64` conversions
in addition to the standard `From` and `Into`.

If multiple conversion paths are found, the conversion will fail,
thus failing the `infer_construct` macro.

## Missing Features

[ ] Allow paths as type name.
[ ] Tuple structs support.

## Possible Features

[ ] Generics parsing.
