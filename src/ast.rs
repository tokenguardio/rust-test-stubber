use std::{fs, io, io::Read, io::Write};
use syn::export::Span;
extern crate quote;
use quote::*;

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    ParseError(syn::Error)
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn process_file(in_path: &str, out_path: &str) -> Result<()> {
    let mut file = load_file(in_path)?;
    prepend_test_mod(&mut file);
    let mut out = fs::File::create(out_path).map_err(|e| Error::IOError(e))?;
    out.write_fmt(format_args!("{}", quote::quote!(#file))).map_err(|e| Error::IOError(e))?;
    Ok(())
}

pub fn process_string(contents: String) -> Result<String> {
    let mut file = syn::parse_file(&contents).map_err(|e| Error::ParseError(e))?;
    prepend_test_mod(&mut file);
    Ok(format!("{}", quote::quote!(#file)))
}

/// Loads file in a Result.
fn load_file(path: &str) -> Result<syn::File> {
    fn get_contents(path: &str) -> io::Result<String> {
        let mut file = fs::File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        Ok(buf)
    }
    let contents = get_contents(path).map_err(|e| Error::IOError(e))?;
    let ast = syn::parse_file(&contents).map_err(|e| Error::ParseError(e))?;
    Ok(ast)
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
    let name = item_trait.ident;
    let fake_struct_name = format!("Fake{}", name.to_string());
    let fake_struct_id = syn::Ident::new(&fake_struct_name, Span::call_site());

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
                        {

                        }
                    }
                }),
            _ => None,
        })
        .collect();
    
    let ast_struct: syn::ItemStruct = syn::parse_quote! {
        struct #fake_struct_id;
    };
    let ast_impl: syn::ItemImpl = syn::parse_quote! {
        impl #name for #fake_struct_id {
            #(#stubs)*
        }
    };

    (ast_struct, ast_impl)
}

fn traverse_items(items: Vec<syn::Item>) -> syn::ItemMod {
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
            let ty = ipl.self_ty;
            ipl.items
                .into_iter()
                .filter_map(move |it| match it {
                    syn::ImplItem::Method(m) => {
                        let fmted = 
                            format!("{}_{}", 
                                quote!(#ty), 
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

fn prepend_test_mod(file: &mut syn::File) {
    file.items.push(syn::Item::Mod(traverse_items(file.items.clone())));
}