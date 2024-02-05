use std::collections::HashMap;

use syn::{
    braced, parenthesized, parse::{Parse, ParseBuffer}, Block, FnArg, Ident, ReturnType, Token, Type, TypeTuple, Visibility
};

pub struct Extend {
    pub name: Ident,
    pub extend: Option<Box<Extend>>,
}

pub struct ClassInfo {
    pub visibility: Visibility,
    pub name: Ident,
    pub extend: Option<Extend>,
    pub fields: Vec<ClassField>,
    pub fn_decls: Vec<FnDecl>,
    pub overrides: HashMap<Ident, Vec<FnDecl>>,
    pub impls: HashMap<Ident, Vec<FnDecl>>,
}

pub struct ClassField {
    pub visibility: Visibility,
    pub name: Ident,
    pub ty: Type,
}

pub struct FnDecl {
    pub visibility: Visibility,
    pub name: Ident,
    pub params: Vec<FnArg>,
    pub return_type: ReturnType,
    pub body: Block,
}

impl FnDecl {
    pub fn is_static(&self) -> bool {
        if let Some(first) = self.params.first() {
            if matches!(first, FnArg::Receiver(_)) {
                false
            } else {
                true
            }
        } else {
            true
        }
    }
}

impl Parse for ClassInfo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let visibility: Visibility = input.parse()?;
        let name: Ident = input.parse()?;

        let extend = if input.peek(syn::Ident) {
            let ident: Ident = input.parse()?;
            if ident == "extends" {
                let extend = parse_extend(input)?;
                Some(extend)
            } else {
                panic!("expect `extends` but get `{}`", ident)
            }
        } else {
            None
        };

        let class_content;
        braced!(class_content in input);

        let mut fields: Vec<ClassField> = Vec::new();
        let mut fn_decls: Vec<FnDecl> = Vec::new();
        let mut overrides: HashMap<Ident, Vec<FnDecl>> = HashMap::new();
        let mut impls: HashMap<Ident, Vec<FnDecl>> = HashMap::new();

        while !class_content.is_empty() {
            if class_content.peek(Token![override]) {
                class_content.parse::<Token![override]>()?;
                let name: Ident = class_content.parse()?;
                let override_content;
                braced!(override_content in class_content);
                let fn_decls = parse_fns(&override_content)?;
                overrides.entry(name).or_insert(Vec::new()).extend(fn_decls);
            } else if class_content.peek(Token![impl]) {
                class_content.parse::<Token![impl]>()?;
                let name: Ident = class_content.parse()?;
                let impl_content;
                braced!(impl_content in class_content);
                let fn_decls = parse_fns(&impl_content)?;
                impls.entry(name).or_insert(Vec::new()).extend(fn_decls);
            } else if class_content.peek(Token![pub]) {
                if class_content.peek2(Token![let]) {
                    fields.push(parse_field(&class_content)?);
                } else {
                    fn_decls.push(parse_fn_decl(&class_content)?);
                }
            } else if class_content.peek(Token![let]) {
                fields.push(parse_field(&class_content)?);
            } else {
                fn_decls.push(parse_fn_decl(&class_content)?);
            }
        }

        Ok(ClassInfo {
            visibility,
            name,
            extend,
            fields,
            fn_decls,
            overrides,
            impls,
        })
    }
}

fn parse_field(input: &ParseBuffer) -> syn::Result<ClassField> {
    let visibility: Visibility = input.parse()?;
    input.parse::<Token![let]>()?;
    let name: Ident = input.parse()?;
    input.parse::<Token![:]>()?;
    let ty: Type = input.parse()?;
    input.parse::<Token![;]>()?;

    Ok(ClassField {
        visibility,
        name,
        ty,
    })
}

fn parse_fn_decl(input: &ParseBuffer) -> syn::Result<FnDecl> {
    let visibility = if input.peek(Token![pub]) {
        let token = input.parse::<Token![pub]>()?;
        Visibility::Public(token)
    } else {
        Visibility::Inherited
    };

    input.parse::<Token![fn]>()?;
    let name: Ident = input.parse()?;

    let params_content;
    parenthesized!(params_content in input);
    let params = parse_params(&params_content)?;

    let return_type: ReturnType = input.parse()?;

    let body: Block = input.parse()?;

    Ok(FnDecl {
        visibility,
        name,
        params,
        return_type,
        body,
    })
}

fn parse_params(input: &ParseBuffer) -> syn::Result<Vec<FnArg>> {
    let mut params = Vec::new();

    while !input.is_empty() {
        let fn_arg: FnArg = input.parse()?;
        params.push(fn_arg);
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }
    }

    Ok(params)
}

fn parse_fns(input: &ParseBuffer) -> syn::Result<Vec<FnDecl>> {
    let mut fn_decls = Vec::new();
    while !input.is_empty() {
        fn_decls.push(parse_fn_decl(input)?)
    }
    Ok(fn_decls)
}

fn parse_extend(input: &ParseBuffer) -> syn::Result<Extend> {
    let name: Ident = input.parse()?;
    let extend = if input.peek(Token![<]) {
        input.parse::<Token![<]>()?;
        Some(Box::new(parse_extend(input)?))
    } else {
        None
    };

    Ok(Extend {
        name,
        extend,
    })
}