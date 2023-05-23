use proc_macro::TokenStream;
use syn::{
  parse_macro_input, Ident,
  __private::{Span, ToTokens},
};

/// Pre-compiled conversion from identifier to &'static str
///
/// ```no_run
/// use strife_macros::shouting_snake_case_str;
///
/// assert_eq!(shouting_snake_case_str!(HelloWorld), "HELLO_WORLD");
/// assert_eq!(shouting_snake_case_str!(helloWorld), "HELLO_WORLD");
/// ```
#[proc_macro]
pub fn shouting_snake_case_str(input: TokenStream) -> TokenStream {
  // Parse the identifier or somewhere else there
  let name = parse_macro_input!(input as Ident).to_string();
  let content = heck::AsShoutySnakeCase(name).to_string();

  syn::LitStr::new(&content, Span::call_site())
    .into_token_stream()
    .into()
}
