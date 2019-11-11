#[macro_use] extern crate lazy_static;
extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};


lazy_static! {
  static ref TEST_VEC: Vec<&'static str> = vec!["Test"];
}

#[proc_macro_attribute]
pub fn test_macro(_args: TokenStream, input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as ItemFn);

  let macro_args = input.attrs[0].tts.to_string();
  let mut macro_args_chars = macro_args.chars();
  macro_args_chars.next();
  macro_args_chars.next();
  let offset = macro_args_chars.as_str().find("\"").unwrap();

  let route_type = input.attrs[0].path.segments[0].ident.to_string();
  let route_path = &macro_args[2..offset+2];
  println!("{}, {}", route_type, route_path);

  //println!("{}", TEST_VEC[0]);
  //TEST_VEC.push(route_path);

  TokenStream::from(quote!(#input))
}