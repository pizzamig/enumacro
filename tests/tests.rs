use enumacro::EDefault;

#[test]
fn derive_no_default_no_generics() {
    #[derive(EDefault, PartialEq, Debug)]
    enum EnumUT {
        One,
        Two,
    };
    assert_eq!(EnumUT::default(), EnumUT::One);
    assert_ne!(EnumUT::default(), EnumUT::Two);
}

#[test]
fn derive_default_no_generics() {
    #[derive(EDefault, PartialEq, Debug)]
    enum EnumUT {
        One,
        #[edefault]
        Two,
    };
    assert_ne!(EnumUT::default(), EnumUT::One);
    assert_eq!(EnumUT::default(), EnumUT::Two);
}

#[test]
fn derive_default_no_generics_used() {
    #[derive(EDefault, PartialEq, Debug)]
    enum EnumUT<T> {
        One(T),
        #[edefault]
        Two,
    };
    assert!(EnumUT::<()>::default() == EnumUT::Two);
    assert_ne!(EnumUT::<()>::default(), EnumUT::One(()));
}
#[test]
fn derive_default_generics() {
    #[derive(EDefault, PartialEq, Debug)]
    enum EnumUT<T> {
        One(T),
        #[edefault]
        Two(T),
    };
    assert_eq!(EnumUT::<u64>::default(), EnumUT::Two(0));
    assert_ne!(EnumUT::<u64>::default(), EnumUT::One(0));
}

#[test]
fn derive_multiple_generics() {
    #[derive(EDefault, PartialEq, Debug)]
    enum EnumUT<T, V, Z> {
        One(T, Z),
        Two(V),
    };
    assert_eq!(
        EnumUT::<u64, f64, String>::default(),
        EnumUT::One(0, "".to_string())
    );
    assert_ne!(
        EnumUT::<u64, f64, String>::default(),
        EnumUT::Two(f64::default())
    );
}

#[test]
fn derive_default_multiple_generics() {
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
}
