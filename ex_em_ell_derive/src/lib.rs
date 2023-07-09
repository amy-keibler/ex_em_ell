#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(FromXmlDocument, attributes(ex_em_ell))]
pub fn decode_derive_document(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    todo!()
}

#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(FromXmlElement, attributes(ex_em_ell))]
pub fn decode_derive_element(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    todo!()
}

#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(ToXmlDocument, attributes(ex_em_ell))]
pub fn enecode_derive_document(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    todo!()
}

#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(ToXmlElement, attributes(ex_em_ell))]
pub fn enecode_derive_element(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    todo!()
}
