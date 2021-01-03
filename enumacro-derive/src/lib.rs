use proc_macro::TokenStream;
use proc_macro_error::{abort_call_site, emit_warning, proc_macro_error};
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_derive(EDefault, attributes(edefault))]
#[proc_macro_error]
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
                    emit_warning!(name, "No variants available for enum {}", name; help="Add at least a variant to the enum {}", name; note = "the Default trait is not implemented");
                    TokenStream::default()
                }
            } else if let Some(variant_ident) = find_variant_ident(&data) {
                let variant_generic_idents = get_variant_generic_idents(&data, &variant_ident);
                if variant_generic_idents.is_empty() {
                    impl_edefault(&name, &generics, &variant_ident)
                } else {
                    impl_edefault2(&name, &generics, &variant_ident, &variant_generic_idents)
                }
            } else {
                emit_warning!(name, "No variants available for enum {}", name; help="Add at least a variant to the enum {}", name; note = "the Default trait is not implemented");
                TokenStream::default()
            }
        }
        _ => abort_call_site!("Only enum are supported"),
    }
}

#[proc_macro_derive(EVariants, attributes())]
pub fn enum_variants_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    //dbg!(&ast);
    match ast.data {
        syn::Data::Enum(data) => {
            let mut variants = Vec::new();
            data.variants
                .iter()
                .for_each(|v| variants.push(v.ident.clone()));
            let name = ast.ident;
            let generics = ast.generics;
            impl_evariants(&name, &generics, &variants)
        }
        _ => abort_call_site!("Only enum are supported"),
    }
}

fn get_variant_generic_idents(data: &syn::DataEnum, variant_ident: &syn::Ident) -> Vec<syn::Ident> {
    let mut result = Vec::new();
    for v in data.variants.iter() {
        if v.ident == *variant_ident.to_string() {
            if let syn::Fields::Unnamed(f) = v.fields.clone() {
                for u in f.unnamed.iter() {
                    if let syn::Type::Path(tp) = u.ty.clone() {
                        result.push(tp.path.get_ident().unwrap().clone())
                    }
                }
                break;
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

fn add_default_trait_if_needed(predicate: &mut syn::WherePredicate, generic_ident: &syn::Ident) {
    if let syn::WherePredicate::Type(pt) = predicate {
        if let syn::Type::Path(tp) = pt.bounded_ty.clone() {
            if tp.path.is_ident(generic_ident) {
                let default_type: syn::TraitBound = syn::parse_str("Default").unwrap();
                let default_trait = syn::TypeParamBound::Trait(default_type);
                pt.bounds.push(default_trait);
            }
        }
    }
}

fn impl_edefault2(
    name: &syn::Ident,
    generics: &syn::Generics,
    variant: &syn::Ident,
    generic_idents: &[syn::Ident],
) -> TokenStream {
    let (impl_generics, ty_generics, where_clause_orig) = generics.split_for_impl();
    let where_clause_new = if let Some(where_clause) = where_clause_orig {
        let mut where_clause = where_clause.clone();
        use quote::ToTokens;
        for g in generic_idents.iter() {
            for p in where_clause.predicates.iter_mut() {
                add_default_trait_if_needed(p, g);
            }
        }
        where_clause.to_token_stream()
    } else {
        let mut where_clause = "where ".to_string();
        for gi in generic_idents.iter() {
            where_clause.push_str(&gi.to_string());
            where_clause.push_str(": Default,");
        }
        where_clause.parse::<proc_macro2::TokenStream>().unwrap()
    };
    let mut init_fields = "".to_string();
    for gi in generic_idents.iter() {
        init_fields.push_str(&gi.to_string());
        init_fields.push_str("::default(),");
    }
    let init_fields = init_fields.strip_suffix(',').unwrap();

    let init_fields: proc_macro2::TokenStream = init_fields.parse().unwrap();
    let result = quote! {
        impl#impl_generics ::std::default::Default for #name#ty_generics
            #where_clause_new
        {
            fn default() -> Self {
                Self::#variant(#init_fields)
            }
        }
    };
    //println!("{}", result);
    result.into()
}

fn impl_evariants(
    name: &syn::Ident,
    generics: &syn::Generics,
    _variants: &[syn::Ident],
) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let mut init = "[".to_string();
    for v in _variants.iter() {
        init.push('"');
        init.push_str(&v.to_string());
        init.push_str("\".to_string(),");
    }
    init.pop().unwrap();
    init.push(']');

    let init: proc_macro2::TokenStream = init.parse().unwrap();

    let result = quote! {
        impl#impl_generics ::enumacro::EnumVariantsVec for #name#ty_generics #where_clause
        {
            fn get_variants() -> Vec<String> {
                vec!#init
            }
        }
    };
    //println!("{}", result);
    result.into()
}
