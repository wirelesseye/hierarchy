use std::collections::HashMap;

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, FnArg, Ident, Visibility};

use crate::classinfo::{ClassField, ClassInfo, Extend, FnDecl};

pub fn class_impl(input: TokenStream) -> TokenStream {
    let classinfo = parse_macro_input!(input as ClassInfo);

    let struct_output = build_struct(&classinfo);
    let impl_output = build_impl(&classinfo);
    let trait_output = build_trait(&classinfo);
    let extend_output = build_extend(&classinfo);
    let impls_output = build_impls(&classinfo);

    let output = quote!(
        #struct_output
        #impl_output
        #trait_output
        #extend_output
        #impls_output
    );

    output.into()
}

fn build_struct(classinfo: &ClassInfo) -> proc_macro2::TokenStream {
    let ClassInfo {
        visibility,
        name,
        extend,
        fields,
        fn_decls: _,
        overrides: _,
        impls: _,
    } = classinfo;

    let super_field = if let Some(extend) = extend {
        let super_name = &extend.name;
        let super_field_name = to_snake_case(super_name);
        quote!(
            pub #super_field_name: #super_name,
        )
    } else {
        quote!()
    };
    let fields = build_fields(fields);

    quote!(
        #visibility struct #name {
            #super_field
            #fields
        }
    )
}

fn build_fields(fields: &Vec<ClassField>) -> proc_macro2::TokenStream {
    let mut output = quote!();

    for ClassField {
        visibility,
        name,
        ty,
    } in fields
    {
        output = quote!(
            #output
            #visibility #name : #ty,
        );
    }

    output.into()
}

fn build_impl(classinfo: &ClassInfo) -> proc_macro2::TokenStream {
    let ClassInfo {
        visibility: _,
        name,
        extend: _,
        fields: _,
        fn_decls,
        overrides: _,
        impls: _,
    } = classinfo;

    let fns = build_fns(fn_decls, true, |fn_decl| {
        matches!(fn_decl.visibility, Visibility::Inherited) || fn_decl.is_static()
    });

    quote!(
        impl #name {
            #fns
        }
    )
}

fn build_params(params: &Vec<FnArg>) -> proc_macro2::TokenStream {
    let mut output = quote!();

    for param in params {
        if output.is_empty() {
            output = param.into_token_stream()
        } else {
            let ts = param.into_token_stream();
            output = quote!(#output, #ts)
        }
    }

    output
}

fn build_trait(classinfo: &ClassInfo) -> proc_macro2::TokenStream {
    let ClassInfo {
        visibility,
        name,
        extend,
        fields: _,
        fn_decls,
        overrides: _,
        impls,
    } = classinfo;

    let name_snake = to_snake_case(name);
    let trait_name = format_ident!("{}Trait", name);
    let get_struct_name = format_ident!("get_{}_struct", name_snake);
    let dependencies = build_trait_dependencies(extend, impls);
    let fns = build_fns(fn_decls, false, |fn_decl| {
        matches!(fn_decl.visibility, Visibility::Public(_)) && !fn_decl.is_static()
    });

    quote!(
        #visibility trait #trait_name #dependencies {
            fn #get_struct_name(&self) -> &#name;
            fn get_super(&self) -> &dyn #trait_name;
            #fns
        }

        impl #trait_name for #name {
            fn #get_struct_name(&self) -> &#name {
                self
            }
            fn get_super(&self) -> &dyn #trait_name {
                self
            }
        }
    )
}

fn build_trait_dependencies(extend: &Option<Extend>, impls: &HashMap<Ident, Vec<FnDecl>>) -> proc_macro2::TokenStream {
    let mut output = quote!();
    
    if extend.is_some() {
        let mut extend = extend.as_ref().unwrap();
        
        loop {
            let super_trait_name = format_ident!("{}Trait", extend.name);
    
            if output.is_empty() {
                output = quote!(: #super_trait_name)
            } else {
                output = quote!(#output + #super_trait_name)
            }
    
            if (extend.extend.is_some()) {
                extend = extend.extend.as_ref().unwrap();
            } else {
                break;
            }
        }
    }

    for (impl_name, _) in impls {
        if output.is_empty() {
            output = quote!(: #impl_name)
        } else {
            output = quote!(#output + #impl_name)
        }
    }

    output
}

fn build_fns(
    fn_decls: &Vec<FnDecl>,
    include_visibility: bool,
    predicate: fn(&FnDecl) -> bool,
) -> proc_macro2::TokenStream {
    let mut output = quote!();

    for fn_decl in fn_decls {
        let FnDecl {
            visibility,
            name,
            params,
            return_type,
            body,
        } = fn_decl;

        let visibility = if include_visibility {
            quote!(#visibility)
        } else {
            quote!()
        };
        if predicate(fn_decl) {
            let params_output = build_params(params);
            output = quote!(
                #output
                #visibility fn #name (#params_output) #return_type #body
            )
        }
    }

    output
}

fn build_extend(classinfo: &ClassInfo) -> proc_macro2::TokenStream {
    let ClassInfo {
        visibility: _,
        name,
        extend,
        fields: _,
        fn_decls: _,
        overrides,
        impls: _,
    } = classinfo;

    if extend.is_none() {
        return quote!();
    }

    let mut extend = extend.as_ref().unwrap();
    let super_field_name = to_snake_case(&extend.name);
    let mut ref_chain = quote!();
    let mut output = quote!();

    loop {
        let extend_name = &extend.name;
        let extend_snake_name = to_snake_case(extend_name);
        let get_extend_name = format_ident!("get_{}_struct", extend_snake_name);
        let extend_trait_name = format_ident!("{}Trait", extend_name);

        if ref_chain.is_empty() {
            ref_chain = quote!(#extend_snake_name)
        } else {
            ref_chain = quote!(#ref_chain . #extend_snake_name)
        }

        let override_fns = if let Some(fn_decls) = overrides.get(extend_name) {
            build_fns(fn_decls, false, |_| true)
        } else {
            quote!()
        };

        output = quote!(
            #output
            impl #extend_trait_name for #name {
                fn #get_extend_name(&self) -> &#extend_name {
                    &self . #ref_chain
                }
                fn get_super(&self) -> &dyn #extend_trait_name {
                    &self.#super_field_name
                }
                #override_fns
            }
        );

        if extend.extend.is_some() {
            extend = extend.extend.as_ref().unwrap()
        } else {
            break;
        }
    }

    output
}

fn build_impls(classinfo: &ClassInfo) -> proc_macro2::TokenStream {
    let ClassInfo {
        visibility,
        name,
        extend,
        fields,
        fn_decls,
        overrides,
        impls,
    } = classinfo;
    let mut output = quote!();

    for (impl_name, fn_decls) in impls {
        let fns = build_fns(fn_decls, false, |_| true);
        output = quote!(
            #output
            impl #impl_name for #name {
                #fns
            }
        );
    }

    output
}

fn to_snake_case(ident: &Ident) -> Ident {
    let snake_case = ident.to_string().to_case(Case::Snake);
    syn::Ident::new(&snake_case, ident.span())
}
