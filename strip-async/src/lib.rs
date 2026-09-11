#![doc = include_str!("../README.md")]

extern crate proc_macro;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

/// Strip `async` and `.await` tokens
#[proc_macro_attribute]
pub fn strip_async(attr: TokenStream, input: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return gen_compile_error("strip_async::strip_async macro doesn't accept any arguments");
    }
    strip_async_inner(input)
}

/// Same as `#[cfg_attr(..., ::strip_async::strip_async)]`
#[proc_macro_attribute]
pub fn strip_async_if(attr: TokenStream, input: TokenStream) -> TokenStream {
    std::iter::empty()
        .chain(gen_cfg_attr(attr, &["strip_async", "strip_async"]).into_iter())
        .chain(input.into_iter())
        .collect()
}

/// Paste the following tokens without any changes
#[proc_macro_attribute]
pub fn keep(attr: TokenStream, input: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return gen_compile_error("strip_async::keep macro doesn't accept any arguments");
    }
    input
}

/// Same as `#[cfg_attr(not(...), ::strip_async::skip)]`
#[proc_macro_attribute]
pub fn keep_if(attr: TokenStream, input: TokenStream) -> TokenStream {
    std::iter::empty()
        .chain(gen_cfg_attr(gen_not(attr), &["strip_async", "skip"]).into_iter())
        .chain(input.into_iter())
        .collect()
}

/// Skip any following tokens
#[proc_macro_attribute]
pub fn skip(attr: TokenStream, _input: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return gen_compile_error("strip_async::skip macro doesn't accept any arguments");
    }
    TokenStream::new()
}

/// Same as `#[cfg_attr(..., ::strip_async::skip)]`
#[proc_macro_attribute]
pub fn skip_if(attr: TokenStream, input: TokenStream) -> TokenStream {
    std::iter::empty()
        .chain(gen_cfg_attr(attr, &["strip_async", "skip"]).into_iter())
        .chain(input.into_iter())
        .collect()
}

fn strip_async_inner(input: TokenStream) -> TokenStream {
    let is_await = |tok: &TokenTree| matches!(tok, TokenTree::Ident(i) if i.to_string() == "await");

    let mut res = TokenStream::new();
    let mut iter = input.into_iter().peekable();
    while let Some(tok) = iter.next() {
        match tok {
            // "async"
            TokenTree::Ident(i) if i.to_string() == "async" => {}
            // ".await"
            TokenTree::Punct(p) if p.as_char() == '.' && iter.next_if(is_await).is_some() => {}
            // recurse
            TokenTree::Group(g) => {
                let mut new_g = Group::new(g.delimiter(), strip_async_inner(g.stream()));
                new_g.set_span(g.span());
                res.extend([TokenTree::Group(new_g)]);
            }
            _ => res.extend([tok]),
        }
    }
    res
}

/// Generate `compile_error!("{err}")`
fn gen_compile_error(err: &str) -> TokenStream {
    [
        TokenTree::Ident(Ident::new("compile_error", Span::call_site())),
        TokenTree::Punct(Punct::new('!', proc_macro::Spacing::Joint)),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::from_iter([TokenTree::Literal(Literal::string(err))]),
        )),
        TokenTree::Punct(Punct::new(';', proc_macro::Spacing::Joint)),
    ]
    .into_iter()
    .collect()
}

/// Generate `not({attr})`
fn gen_not(attr: TokenStream) -> TokenStream {
    [
        TokenTree::Ident(Ident::new("not", Span::call_site())),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, attr)),
    ]
    .into_iter()
    .collect()
}

/// Generate `#[cfg_attr({attr}, ::{path})]`
fn gen_cfg_attr(attr: TokenStream, path: &[&str]) -> TokenStream {
    [
        TokenTree::Punct(Punct::new('#', proc_macro::Spacing::Joint)),
        TokenTree::Group(Group::new(
            Delimiter::Bracket,
            TokenStream::from_iter([
                TokenTree::Ident(Ident::new("cfg_attr", Span::call_site())),
                TokenTree::Group(Group::new(
                    Delimiter::Parenthesis,
                    TokenStream::from_iter(
                        attr.into_iter()
                            .chain([TokenTree::Punct(Punct::new(',', Spacing::Alone))].into_iter())
                            .chain(
                                path.iter()
                                    .map(|segment| {
                                        [
                                            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                                            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                                            TokenTree::Ident(Ident::new(
                                                segment,
                                                Span::call_site(),
                                            )),
                                        ]
                                        .into_iter()
                                    })
                                    .flatten(),
                            ),
                    ),
                )),
            ]),
        )),
    ]
    .into_iter()
    .collect()
}
