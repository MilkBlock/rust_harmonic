
// static W:LazyRef<TokenStream>= LazyRef::new(||{ quote!(wrap) });
// sync/send global lazylock reference
// impl<T:ToTokens+Clone> ToTokens for LazyRef<T> {
//     fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
//         T::to_tokens(&self.t.clone(), tokens);
//     }
// }
