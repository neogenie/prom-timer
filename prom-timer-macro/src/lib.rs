//! Macros for [`prom-timer`].
//!
//! [`prom-timer`]: https://crates.io/crates/prom-timer

use proc_macro::TokenStream;
use quote::quote;
use std::collections::vec_deque::VecDeque;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Expr, ItemFn, Token};

struct Args {
    metric: Expr,
    tags: Vec<Expr>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut params: VecDeque<Expr> = Punctuated::<Expr, Token![,]>::parse_terminated(input)?
            .into_iter()
            .collect();

        let metric = params.pop_front().unwrap();

        Ok(Args {
            metric,
            tags: params.into_iter().collect(),
        })
    }
}


///
/// Decorator for synchronous and asynchronous functions to measure their execution time with
/// RAII [`prom-timer`]
///
/// As the error message says, handler function needs to be async.
///
/// ```
/// use prometheus::{self, HistogramVec, histogram_opts};
///
/// let timer = HistogramVec::new(
///             histogram_opts!("timer", "Timer")
///                 .namespace("api_v2")
///                 .const_labels(labels.clone())
///                 .buckets(
///                     [
///                         0.001, 0.0025, 0.005, 0.01, 0.025, 0.05, 0.1, 0.2, 0.3, 0.4, 0.5, 1.0, 2.0,
///                        3.0, 4.0, 5.0, 10.0,
///                     ]
///                     .into(),
///                 ),
///             &["tag"],
///         )
///         .unwrap();
///
/// registry.register(Box::new(timer.clone())).unwrap();
///
/// #[timer(timer, "f")]
/// async fn f() {
///     // ... Some work here.
/// }
///
/// ```
/// `f()` execution time will be stored as timer histogram entry with tag `f`
///
#[proc_macro_attribute]
pub fn timer(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let args = parse_macro_input!(args as Args);

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;

    let metric = args.metric;
    let tags = args.tags;
    let stmts = &block.stmts;

    let expanded = quote! {
        #(#attrs)* #vis #sig {
            let _t = prom_timer::Timer::new(#metric, &[#(#tags)*]);
            #(#stmts)*
        }
    };

    TokenStream::from(expanded)
}
