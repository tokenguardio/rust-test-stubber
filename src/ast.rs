/// This module contains functions that allows to create test stubs.
use quote::quote;
use syn::{export::Span};
use syn::Ident;
use regex::Regex;

pub fn process_string(content: String) -> Result<String, syn::Error> {
    let mut file = syn::parse_file(&content)?;
    let mut items = remove_old_mod(file.items.clone());
    items.push(syn::Item::Mod(traverse_items(&file.items)));
    file.items.clear();
    items.into_iter().for_each(|it| file.items.push(it));
    Ok(format!("{}", quote!(#file)))
}

fn remove_old_mod(vec: Vec<syn::Item>) -> Vec<syn::Item> {
    vec.into_iter()
        .filter(|it| match it {
            syn::Item::Mod(m) if m.ident.to_string() == "should" => false,
            _ => true
        })
        .collect()
}

/// Traverses file item by item, generating test module that contains method stubs
/// and mocks.
pub fn traverse_items(items: &Vec<syn::Item>) -> syn::ItemMod {
    let (structs, impls): (Vec<syn::ItemStruct>, Vec<syn::ItemImpl>) = items
        .clone()
        .into_iter()
        .filter_map(|it| match it {
            syn::Item::Trait(t) => Some(gen_trait_stub(t)),
            _ => None,
        })
        .collect::<Vec<_>>()
        .iter()
        .cloned()
        .unzip();

    let fn_stubs: Vec<syn::ItemFn> = items
        .clone()
        .into_iter()
        .filter_map(|it| match it {
            syn::Item::Fn(f) => Some(gen_method_stub(&f.sig.ident.to_string())),
            _ => None
        })
        .collect();

    let impl_stubs: Vec<syn::ItemFn> = items
        .clone()
        .into_iter()
        .filter_map(|it| match it {
            syn::Item::Impl(ipl) if ipl.trait_.is_none() => Some(ipl),
            _ => None
        })
        .flat_map(|ipl| {
            let ty = *ipl.self_ty;
            ipl.items
                .into_iter()
                .filter_map(move |it| match it {
                    syn::ImplItem::Method(m) => {
                        let demangled_ty = {
                            let re = Regex::new("[^a-zA-Z]+")
                                .expect("regex should be valid");
                            let var_name = format!("{}", quote!(#ty));
                            let demangled = re.replace_all(&var_name, "");
                            format!("{}", demangled)
                        };
                        let fmted = 
                            format!("{}_{}", 
                                demangled_ty, 
                                &m.sig.ident.to_string());
                        Some(gen_method_stub(&fmted))
                    },
                    _ => None
                })
        })
        .collect();

    syn::parse_quote! {
        #[cfg(test)]
        mod should {
            #(#structs)*
            #(#impls)*
            #(#fn_stubs)*
            #(#impl_stubs)*
        }
    }
}

fn gen_method_stub(name: &str) -> syn::ItemFn {
    let fmt_name = format!("test_{}", name);
    let id = syn::Ident::new(&fmt_name, Span::call_site());
    syn::parse_quote! {
        #[test]
        fn #id() {
            // TODO
        }
    }
}

fn gen_trait_stub(item_trait: syn::ItemTrait) -> (syn::ItemStruct, syn::ItemImpl) {
    let fake_struct_name = format!("Fake{}", item_trait.ident.to_string());
    let fake_struct_id = Ident::new(&fake_struct_name, Span::call_site());

    let stubs: Vec<syn::ImplItemMethod> = item_trait
        .items
        .into_iter()
        .filter_map(|it| match it {
            syn::TraitItem::Method(method) => 
                Some(syn::ImplItemMethod {
                    attrs: method.attrs, 
                    vis: syn::Visibility::Inherited,
                    defaultness: None,
                    sig: method.sig,
                    block: syn::parse_quote! {
                        {}
                    }
                }),
            _ => None,
        })
        .collect();
    
    let ast_struct: syn::ItemStruct = syn::parse_quote! {
        struct #fake_struct_id;
    };
    // trace_macros!(true);
    let trait_name = item_trait.ident;
    let ast_impl: syn::ItemImpl = syn::parse_quote! {
        impl #trait_name for #fake_struct_id {
            #(#stubs)*
        }
    };

    (ast_struct, ast_impl)
}