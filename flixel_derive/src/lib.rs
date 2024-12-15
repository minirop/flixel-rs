use syn::parse::Parser;
use proc_macro::Span;
use syn::*;
use quote::quote;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn state(args: TokenStream, tokens: TokenStream) -> TokenStream {
    template_proc_macro(args, tokens, "FlxState")
}

#[proc_macro_attribute]
pub fn widget(args: TokenStream, tokens: TokenStream) -> TokenStream {
    template_proc_macro(args, tokens, "FlxWidget")
}

fn template_proc_macro(args: TokenStream, tokens: TokenStream, parent: &str) -> TokenStream {
    let _ = parse_macro_input!(args as parse::Nothing);

    let parent = Ident::new(parent, Span::call_site().into());

    let mut fields_to_init = vec![];

    let mut item_struct = parse_macro_input!(tokens as ItemStruct);
    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        for f in &fields.named {
            if let Some(fname) = &f.ident {
                fields_to_init.push(fname.clone());
            }
        }
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { parent: #parent })
                .unwrap(),
        );
    }

    let struct_name = &item_struct.ident;

    let state_func = if parent == "FlxState" {
        quote!{
            impl ConcreteState for #struct_name {
                fn state(&self) -> &FlxState {
                    &self
                }
            }
        }
    } else {
        quote!{}
    };

    let output = quote! {
        #item_struct

        impl std::default::Default for #struct_name {
            fn default() -> Self {
                Self {
                    parent: #parent::default(),
                    #( #fields_to_init: Default::default() ),*
                }
            }
        }

        impl #struct_name {
            pub fn new() -> Box<Self> {
                Box::new(Self::default())
            }
        }

        #state_func

        impl std::ops::Deref for #struct_name {
            type Target = #parent;

            fn deref(&self) -> &Self::Target {
                &self.parent
            }
        }

        impl std::ops::DerefMut for #struct_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.parent
            }
        }
    };
    output.into()
}
