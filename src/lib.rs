extern crate proc_macro;

use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    *,
};

// trait TestCollection = impl<T: TestBound> for (TestType<T>)
struct HeterogeneousVec {
    pub vis: Visibility,
    pub _trait_token: Token![trait],
    pub trait_ident: Ident,
    pub _equals_token: Token![=],
    pub _impl_token: Token![impl],
    pub _lt_token: Token![<],
    pub type_param: TypeParam,
    pub _gt_token: Token![>],
    pub _for_token: Token![for],
    pub _paren_token: syn::token::Paren,
    pub item_type: Type,
}

impl Parse for HeterogeneousVec {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            vis: input.parse()?,
            _trait_token: input.parse()?,
            trait_ident: input.parse()?,
            _equals_token: input.parse()?,
            _impl_token: input.parse()?,
            _lt_token: input.parse()?,
            type_param: input.parse()?,
            _gt_token: input.parse()?,
            _for_token: input.parse()?,
            _paren_token: parenthesized!(content in input),
            item_type: content.parse()?,
        })
    }
}

fn substitute<T: quote::ToTokens + Parse>(replace: &Ident, with: &Ident, item: &T) -> T {
    use proc_macro2::*;
    let replace = replace.to_string();
    let with = quote! { #with }.into_iter().next().unwrap();
    let tokens = quote! { #item };
    let tokens: TokenStream = tokens
        .into_iter()
        .map(move |token| match token {
            TokenTree::Ident(ref found) if found.to_string() == replace => with.clone(),
            other => other,
        })
        .collect();
    parse2(tokens).unwrap()
}

// https://doc.rust-lang.org/reference/procedural-macros.html
#[proc_macro]
pub fn heterogeneous_vec(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: HeterogeneousVec = parse_macro_input!(input);

    let vis = &input.vis;
    let trait_ident = &input.trait_ident;
    let type_param = &input.type_param;
    let type_ident = &type_param.ident;
    let item_type = &input.item_type;

    let type_ident_0 = Ident::new("T0", type_ident.span());
    let type_param_0 = substitute(&type_ident, &type_ident_0, type_param);

    let type_ident_1 = Ident::new("T1", type_ident.span());
    let type_param_1 = substitute(&type_ident, &type_ident_1, type_param);

    let type_ident_2 = Ident::new("T2", type_ident.span());
    let type_param_2 = substitute(&type_ident, &type_ident_2, type_param);

    let functor_ident = Ident::new(&(trait_ident.to_string() + "Functor"), trait_ident.span());

    let expanded = quote! {
        #vis trait #functor_ident {
            type Result<#type_param>;
            fn apply<#type_param>(&mut self, v: #item_type) -> Self::Result<#type_ident>;
        }

        #vis trait #trait_ident {
            type Push<#type_param>: #trait_ident;
            fn push<#type_param>(self, v: #item_type) -> Self::Push<#type_ident>;

            type Map<F: #functor_ident>;
            fn map<F: #functor_ident>(self, f: &mut F) -> Self::Map<F>;
        }

        impl #trait_ident for () {
            type Push<#type_param> = (#type_ident,);
            fn push<#type_param>(self, v: #item_type) -> Self::Push<#type_ident> {
                (v,)
            }

            type Map<F: #functor_ident> = ();
            fn map<F: #functor_ident>(self, f: &mut F) -> Self::Map<F> {
                ()
            }
        }

        impl<#type_param_0> #trait_ident for
            (#type_ident_0,) {
            type Push<#type_param> = (#type_ident_0, #type_ident);
            fn push<#type_param>(self, v: #item_type) -> Self::Push<#type_ident> {
                (self.0, v)
            }

            type Map<F: #functor_ident> = (F::Result<#type_ident_0>,);
            fn map<F: #functor_ident>(self, f: &mut F) -> Self::Map<F> {
                (f.apply(self.0),)
            }
        }

        impl<#type_param_0, #type_param_1> #trait_ident for
            (#type_ident_0, #type_ident_1) {
            type Push<#type_param> = (#type_ident_0, #type_ident_1, #type_ident);
            fn push<#type_param>(self, v: #item_type) -> Self::Push<#type_ident> {
                (self.0, self.1, v)
            }

            type Map<F: #functor_ident> =
                (F::Result<#type_ident_0>, F::Result<#type_ident_1>);
            fn map<F: #functor_ident>(self, f: &mut F) -> Self::Map<F> {
                (f.apply(self.0), f.apply(self.1))
            }
        }

        impl<#type_param_0, #type_param_1, #type_param_2> #trait_ident for
            (#type_ident_0, #type_ident_1, #type_ident_2) {
            type Push<#type_param> = ();
            fn push<#type_param>(self, v: #item_type) -> Self::Push<#type_ident> { unimplemented!(); }

            type Map<F: #functor_ident> =
                (F::Result<#type_ident_0>, F::Result<#type_ident_1>, F::Result<#type_ident_2>);
            fn map<F: #functor_ident>(self, f: &mut F) -> Self::Map<F> {
                (f.apply(self.0), f.apply(self.1), f.apply(self.2))
            }
        }
    };

    expanded.into()
}

#[test]
pub fn basic() {
    macrotest::expand("test/basic.rs");
}
