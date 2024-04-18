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
        father: Parent {
            name: "Tommy",
            age: 42
        }
    }
}
```

This expands to

```rust
Student {
    name: Into::into("Timmy"),
    age: Into::into(10),
    father: construct! {
        Parent {
            name: "Tommy",
            age: 42
        }
    }
    ..Default::default()
}
```

The macro is recursive on nested struct declarations,
if the behavior is not desired, wrap nested structs in brackets.

```rust
construct! {
    Student {
        name: "Timmy",
        age: 10,
        father: { Parent {
            name: "Tommy",
            age: 42
        }}
    }
}
```

## Meta Constructor

The meta constructor macro allows you to define your own macro with
custom configurations.

See documentation on `meta_default_constructor!` for details.

## `InferInto`

InferInto allows the user to bypass the orphan rule to create conversions.

By default we provide `i32` (integer literal) to all numeric types
and `i64` -> `f64` in addition to the standard `From` and `Into`.

If multiple conversion paths are found, the conversion will fail,
thus failing the `infer_construct` macro.

## Missing Features

- [x] Allow paths as type names.
- [ ] Tuple struct support.

## Possible Features

- [ ] Generics parsing.

## License

License under either of

Apache License, Version 2.0 (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>)
at your option.

## Contribution

Contributions are welcome!

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
