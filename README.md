# enumacro

macro for enums

Educational project to learn using macros (with enum, for now)

## `#[derive(EDefault)]`

Inspired by the `enum_default` crate, this derive macro will automagically implement the `Default` trait for an `Enum`.
A simple example, where the first variant is the default one:

```rust
    use enumacro::EDefault;

    #[derive(EDefault, PartialEq, Debug)]
    enum MyEnum {
        One,
        Two,
    };
    assert_eq!(MyEnum::default(), MyEnum::One);
    assert_ne!(MyEnum::default(), MyEnum::Two);
```

A more complex example, with generics, unnamed fields and the attribute `edefault` to specify the default variant:

```rust
    use enumacro::EDefault;

    #[derive(EDefault, PartialEq, Debug)]
    enum MyEnum<T, V, Z> {
        One(T, Z),
        #[edefault]
        Two(V),
    };
    assert_eq!(
        MyEnum::<u64, f64, String>::default(),
        MyEnum::Two(f64::default())
    );
    assert_ne!(
        MyEnum::<u64, f64, String>::default(),
        MyEnum::One(0, "".to_string())
    );
```
