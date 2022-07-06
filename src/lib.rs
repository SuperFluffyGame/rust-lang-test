use proc_macro::TokenStream;

#[proc_macro]
pub fn time(tok: TokenStream) -> TokenStream {
    let mut tok_iter = tok.into_iter();

    let ident = tok_iter.next().unwrap();

    format!("let {ident}_start = std::time::Instant::now();")
        .parse()
        .unwrap()
}

#[proc_macro]
pub fn time_end(tok: TokenStream) -> TokenStream {
    let mut tok_iter = tok.into_iter();

    let ident = tok_iter.next().unwrap();

    format!("let {ident}_elapsed = std::time::Instant::now() - {ident}_start;")
        .parse()
        .unwrap()
}

#[proc_macro]
pub fn time_print(tok: TokenStream) -> TokenStream {
    let mut tok_iter = tok.into_iter();

    let ident = tok_iter.next().unwrap().to_string();

    format!("println!(\"{ident} time: {{:.3}}ms\", {ident}_elapsed.as_secs_f64() * 1000.0);")
        .parse()
        .unwrap()
}
