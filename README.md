# enumacro

macro for enums

Educational project to learn using macros (with enum, for now)

## `#[derive(EDefault)]`

Inspired by the `enum_default` crate, this derive macro will automagically implement the `Default` trait for an `Enum`.
A simple example:

```rust
	use enumacro::EDefault;

    #[derive(EDefault, PartialEq, Debug)]
    enum EnumUT {
        One,
        Two,
    };
    assert_eq!(EnumUT::default(), EnumUT::One);
    assert_ne!(EnumUT::default(), EnumUT::Two);
```

A more complex example, with generics and unnamed fields:

```rust
	use enumacro::EDefault;

    #[derive(EDefault, PartialEq, Debug)]
    enum EnumUT<T, V, Z> {
        One(T, Z),
        #[edefault]
        Two(V),
    };
    assert_eq!(
        EnumUT::<u64, f64, String>::default(),
        EnumUT::Two(f64::default())
    );
    assert_ne!(
        EnumUT::<u64, f64, String>::default(),
        EnumUT::One(0, "".to_string())
    );
```
