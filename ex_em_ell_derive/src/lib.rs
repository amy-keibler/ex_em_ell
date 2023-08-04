use heck::ToLowerCamelCase;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics};

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
    let input = parse_macro_input!(input as DeriveInput);

    let write_xml_document = generate_write_xml_document(&input);

    let name = input.ident;

    // Add a bound `T: FromXmlElement` to every type parameter T.
    let generics = add_to_xml_element_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let expanded = quote! {
        // The generated impl.
        impl #impl_generics ex_em_ell::traits::ToXmlDocument for #name #ty_generics #where_clause {
            fn to_xml_document<W: std::io::Write>(self: &Self, writer: &mut ex_em_ell::xml::EventWriter<W>) -> Result<(), ex_em_ell::errors::XmlWriteError>
            {
                #write_xml_document

                Ok(())
            }
        }
    };

    eprintln!("{}", expanded);

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(ToXmlElement, attributes(ex_em_ell))]
pub fn enecode_derive_element(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    todo!()
}

// Add a bound `T: ToXmlElement` to every type parameter T.
fn add_to_xml_element_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(ex_em_ell::ToXmlElement));
        }
    }
    generics
}

fn generate_write_xml_document(input: &DeriveInput) -> TokenStream {
    let tag_name = input.ident.to_string().to_lower_camel_case();
    quote! {
        ex_em_ell::xml_utils::write_simple_tag(writer, &#tag_name, "TODO")?;
    }
}
