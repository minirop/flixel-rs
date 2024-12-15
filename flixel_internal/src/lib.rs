#![allow(unused)]
use convert_case::*;
use proc_macro::TokenTree;
use syn::parse::Parser;
use proc_macro::Span;
use syn::*;
use quote::quote;
use proc_macro::TokenStream;

#[proc_macro]
pub fn bindffi(tokens: TokenStream) -> TokenStream {
    let signature = parse_macro_input!(tokens as Signature);
    let name = signature.ident.to_string();
    let snake_name = name.to_case(Case::Snake);

    let mut args = vec![];
    let mut named_args = vec![];
    signature.inputs.iter().map(|a| {
        let arg = quote!(#a).to_string();
        let name = arg.split(':').map(|e| e.trim().to_string()).collect::<Vec<_>>();
        named_args.push(name.clone());
        args.push(arg.clone());
    }).collect::<()>();

    let ret = signature.output;
    let mut output = format!("pub fn {snake_name}({}) {} ", args.join(", "), quote!(#ret).to_string());
    output.push_str("{\n");
    output.push_str(&format!("unsafe {{ {name}("));
    for (i, a) in named_args.iter().enumerate() {
        if i > 0 { output.push_str(", "); }

        if a[1] == "& str" {
            output.push_str(&format!("rl_str!({})", a[0]));
        } else {
            output.push_str(&format!("{}", a[0]));
        }
    }
    output.push_str(") }\n");
    output.push_str("}\n");

    output.parse().unwrap()
}
