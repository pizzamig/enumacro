use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_derive(EDefault, attributes(edefault))]
pub fn enum_default_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    //dbg!(&ast);
    match ast.data {
        syn::Data::Enum(data) => {
            let name = ast.ident;
            let generics = ast.generics;
            if generics.params.is_empty() {
                if let Some(ident) = find_variant_ident(&data) {
                    impl_edefault(&name, &generics, &ident)
                } else {
                    TokenStream::default()
                }
            } else if let Some(variant_ident) = find_variant_ident(&data) {
                let generics_idents = get_generics_idents(&generics);
                let variant_generic_idents =
                    get_variant_generic_idents(&data, &variant_ident, &generics_idents);
                if variant_generic_idents.is_empty() {
                    impl_edefault(&name, &generics, &variant_ident)
                } else {
                    impl_edefault2(&name, &generics, &variant_ident, &variant_generic_idents)
                }
            } else {
                TokenStream::default()
            }
        }
        _ => TokenStream::default(),
    }
}

fn get_generics_idents(generics: &syn::Generics) -> Vec<syn::Ident> {
    let mut result = Vec::new();
    for g in generics.params.iter() {
        if let syn::GenericParam::Type(tp) = g {
            result.push(tp.ident.clone())
        }
    }
    result
}

fn get_variant_generic_idents(
    data: &syn::DataEnum,
    variant_ident: &syn::Ident,
    generic_idents: &[syn::Ident],
) -> Vec<syn::Ident> {
    let mut result = Vec::new();
    for g in generic_idents.iter() {
        for v in data.variants.iter() {
            if v.ident == *variant_ident.to_string() {
                if let syn::Fields::Unnamed(f) = v.fields.clone() {
                    for u in f.unnamed.iter() {
                        if let syn::Type::Path(tp) = u.ty.clone() {
                            if tp.path.is_ident(&g.to_string()) {
                                result.push(tp.path.get_ident().unwrap().clone())
                            }
                        }
                    }
                }
            }
        }
    }
    result
}

fn find_variant_ident(data: &syn::DataEnum) -> Option<syn::Ident> {
    for v in data.variants.iter() {
        for a in &v.attrs {
            if a.path.is_ident("edefault") {
                return Some(v.ident.clone());
            }
        }
    }
    if let Some(v) = data.variants.first() {
        Some(v.ident.clone())
    } else {
        None
    }
}

fn impl_edefault(name: &syn::Ident, generics: &syn::Generics, variant: &syn::Ident) -> TokenStream {
    let result = quote! {
        impl#generics Default for #name#generics {
            fn default() -> Self {
                Self::#variant
            }
        }
    };
    result.into()
}

fn impl_edefault2(
    name: &syn::Ident,
    generics: &syn::Generics,
    variant: &syn::Ident,
    generic_idents: &[syn::Ident],
) -> TokenStream {
    let mut where_clause = "where ".to_string();
    let mut init_fields = "".to_string();
    for gi in generic_idents.iter() {
        where_clause.push_str(&gi.to_string());
        where_clause.push_str(": Default,");
        init_fields.push_str(&gi.to_string());
        init_fields.push_str("::default(),");
    }
    let init_fields = init_fields.strip_suffix(',').unwrap();

    let where_clause: proc_macro2::TokenStream = where_clause.parse().unwrap();
    let init_fields: proc_macro2::TokenStream = init_fields.parse().unwrap();
    let result = quote! {
        impl#generics Default for #name#generics
            #where_clause
        {
            fn default() -> Self {
                Self::#variant(#init_fields)
            }
        }
    };
    result.into()
}
