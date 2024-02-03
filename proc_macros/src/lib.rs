use std::collections::HashMap;

use proc_macro::TokenStream;
use syn::{braced, parse::{Parse, ParseStream}, parse_macro_input, Block, Ident, Stmt, Token};
use quote::{quote, format_ident};
use convert_case::{Case, Casing};

struct InheritFrom {
    super_name: Ident,
    inherit_from: Option<Box<InheritFrom>>,
}

struct InheritInfo {
    struct_name: Ident,
    inherit_from: InheritFrom,
    overrides: HashMap<Ident, Vec<Stmt>>
}

impl Parse for InheritInfo {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let struct_name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let inherit_from: InheritFrom = parse_inherit_from(input)?;
        let mut overrides = HashMap::new();

        input.parse::<Token![,]>()?;
        while input.peek(Token![override]) {
            input.parse::<Token![override]>()?;
            let override_name: Ident = input.parse()?;

            let content;
            braced!(content in input);
            let stmts = content.call(Block::parse_within)?;
            
            overrides.entry(override_name).or_insert(Vec::new()).extend(stmts);
        }

        Ok (Self {
            struct_name,
            inherit_from,
            overrides
        })
    }
}

fn parse_inherit_from(input: ParseStream) -> syn::Result<InheritFrom> {
    let super_name: Ident = input.parse()?;

    let inherit_from: Option<Box<InheritFrom>> = if input.peek(Token![<]) {
        input.parse::<Token![<]>()?;
        Some(Box::new(parse_inherit_from(input)?))
    } else {
        None
    };

    Ok(InheritFrom {
        super_name,
        inherit_from
    })
}

fn to_snake_case(ident: &Ident) -> Ident {
    let snake_case = ident.to_string().to_case(Case::Snake);
    syn::Ident::new(&snake_case, ident.span())
}

#[proc_macro]
pub fn inherit(input: TokenStream) -> TokenStream {
    let inherif_info = parse_macro_input!(input as InheritInfo);
    let struct_name = inherif_info.struct_name;
    let mut output = quote!();

    let mut ref_chain = quote!();
    let mut inherit_from = inherif_info.inherit_from;
    loop {
        let super_name = inherit_from.super_name;
        let trait_name = format_ident!("{}Trait", &super_name);
        let super_sname_name = to_snake_case(&super_name);
        let fn_name = format_ident!("get_{}", super_sname_name);
        
        if ref_chain.is_empty() {
            ref_chain = quote!(#super_sname_name)
        } else {
            ref_chain = quote!(#ref_chain . #super_sname_name)
        }

        let override_fns = if let Some(stmts) = inherif_info.overrides.get(&super_name) {
            let mut fns = quote!();
            for stmt in stmts {
                fns = quote!(
                    #fns
                    #stmt
                )
            }
            fns
        } else {
            quote!()
        };

        output = quote! {
            #output
            impl #trait_name for #struct_name {
                fn #fn_name(&self) -> &#super_name {
                    &self . #ref_chain
                }

                #override_fns
            }
        };

        if inherit_from.inherit_from.is_some() {
            inherit_from = *inherit_from.inherit_from.unwrap();
        } else {
            break;
        }
    }

    output.into()
}