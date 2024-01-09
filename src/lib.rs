use proc_macro::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use std::{fs::read_to_string, path::PathBuf};
use syn::{
    parse::Parse, parse::ParseStream, parse_macro_input, punctuated::Punctuated, Expr, ExprAssign,
    Ident, ItemStruct, LitStr, Result,
};

#[proc_macro_attribute]
pub fn donothing(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

fn template(args: TokenStream, input: TokenStream) -> TokenStream {
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
        // #input

        impl TemplateFile for #ident {
            const PATH: &'static str = #path;
            const CONTENT: &'static str = #content;
        }
    }
    .into()
}

#[proc_macro_derive(Template, attributes(template))]
pub fn derive_template(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input.clone()).unwrap();

    let mut template_arg = None;

    let struct_tokstream = if let syn::Data::Struct(data_struct) = ast.data {
        let struct_tokens = data_struct.struct_token.to_token_stream();
        let ident = ast.ident.to_token_stream();
        let fields = data_struct.fields.to_token_stream();

        quote! {
            #struct_tokens #ident #fields
        }
    } else {
        return quote! {
            compile_error!("Missing struct");
        }
        .into();
    };

    println!("DATA STRUCT: {}", struct_tokstream.to_token_stream());

    for attr in &ast.attrs {
        println!("ATTR: {}", attr.to_token_stream());

        if !attr.path().is_ident("template") {
            continue;
        }

        template_arg = Some(
            attr.parse_args_with(Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)
                .expect("Failed to parse arguments from #[template]"),
        );
    }

    let template_arg = template_arg.expect("Missing #[template(path = \"<PATH>\")]");

    template(
        template_arg.to_token_stream().into(),
        struct_tokstream.into(),
    )
    .into()
}
