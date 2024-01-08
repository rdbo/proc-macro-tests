use proc_macro::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use std::{fs::read_to_string, path::PathBuf};
use syn::{
    parse::Parse, parse::ParseStream, parse_macro_input, Expr, ExprAssign, Ident, ItemStruct,
    LitStr, Result,
};

#[proc_macro_attribute]
pub fn donothing(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn template(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ExprAssign);

    let err_path_tokstr: TokenStream = quote! {
        compile_error!("Missing required 'path = \"<PATH>\"'");
    }
    .into();

    if args.left.to_token_stream().to_string() != "path" {
        return err_path_tokstr;
    }

    let path: String = args.right.to_token_stream().to_string();
    let pathbuf = PathBuf::from("templates").join(&path[1..path.len() - 1]);
    let filepath = pathbuf.to_string_lossy().to_string();
    let path: TokenStream = filepath.to_token_stream().into();
    let path = parse_macro_input!(path as LitStr);

    let content =
        read_to_string(&filepath).expect(&format!("Template not found: \"{}\"", filepath));

    let input = parse_macro_input!(input as ItemStruct);
    let ident = &input.ident;

    quote! {
        #input

        impl TemplateFile for #ident {
            const PATH: &'static str = #path;
            const CONTENT: &'static str = #content;
        }
    }
    .into()
}
