use enumacro_derive::EDefault;

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

#[test]
fn derive_default_multiple_generics_traits() {
    #[derive(EDefault, PartialEq, Debug)]
    enum EnumUT<T, V: ::std::fmt::Display, Z> {
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

#[test]
fn derive_default_multiple_generics_traits_where_clause() {
    #[derive(EDefault, PartialEq, Debug)]
    enum EnumUT<T: Default, V, Z>
    where
        Z: 'static + Clone + ::std::fmt::Display,
        V: Clone,
        T: Default,
    {
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
fn derive_variants() {
    use enumacro::EnumVariantsVec;
    use enumacro_derive::EVariants;
    #[derive(EVariants)]
    enum EnumUT {
        _One,
        _Two,
    };
    let variants = EnumUT::get_variants();
    assert_eq!(variants.len(), 2);
    assert!(variants.iter().any(|x| x == "_One"));
    assert!(variants.iter().any(|x| x == "_Two"));
}
#[test]
fn derive_variants_generics() {
    use enumacro::EnumVariantsVec;
    use enumacro_derive::EVariants;
    #[derive(EVariants)]
    enum EnumUT<T: Default, V, Z>
    where
        Z: 'static + Clone + ::std::fmt::Display,
        V: Clone,
        T: Default,
    {
        _One(T, Z),
        _Two(V),
        _Three,
    };
    let variants = EnumUT::<u64, String, f64>::get_variants();
    assert_eq!(variants.len(), 3);
    assert!(variants.iter().any(|x| x == "_One"));
    assert!(variants.iter().any(|x| x == "_Two"));
    assert!(variants.iter().any(|x| x == "_Three"));
}

// This test won't compile, but an appropriate error message will be displayed
//#[test]
//fn defive_default_on_struct() {
//#[derive(EDefault, PartialEq, Debug)]
//struct NotEnum {
//a: u64,
//};
//}
//
//#[test]
//fn derive_default_empty_enum() {
//#[derive(EDefault)]
//enum _EnumUT {};
//}
