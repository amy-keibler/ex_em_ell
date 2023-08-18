use quote::{format_ident, quote};
use syn::{parse_macro_input, parse_quote, DeriveInput, GenericParam, Generics};

mod read;
mod write;

use read::{generate_read_xml_document, generate_read_xml_element};
use write::{generate_write_xml_document, generate_write_xml_element};

#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(FromXmlDocument, attributes(ex_em_ell))]
pub fn decode_derive_document(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let reader_variable = format_ident!("_{}", "reader");

    let read_xml_document = generate_read_xml_document(&input, &reader_variable);

    let name = input.ident;

    // Add a bound `T: FromXmlElement` to every type parameter T.
    let generics = add_from_xml_element_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let expanded = quote! {
        // The generated impl.
        impl #impl_generics ex_em_ell::traits::FromXmlDocument for #name #ty_generics #where_clause {
            fn from_xml_document<R: std::io::Read>(#reader_variable: &mut ex_em_ell::xml::EventReader<R>) -> Result<Self, ex_em_ell::errors::XmlReadError>
            {
                #read_xml_document
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(FromXmlElement, attributes(ex_em_ell))]
pub fn decode_derive_element(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let reader_variable = format_ident!("_{}", "reader");
    let tag_name_variable = format_ident!("_{}", "tag_name");

    let read_xml_element = generate_read_xml_element(&input, &reader_variable, &tag_name_variable);

    let name = input.ident;

    // Add a bound `T: FromXmlElement` to every type parameter T.
    let generics = add_from_xml_element_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let expanded = quote! {
        // The generated impl.
        impl #impl_generics ex_em_ell::traits::FromXmlElement for #name #ty_generics #where_clause {
            fn from_xml_element<R: std::io::Read>(#reader_variable: &mut ex_em_ell::xml::EventReader<R>, #tag_name_variable: &ex_em_ell::xml::name::OwnedName, element_attributes: &[ex_em_ell::xml::attribute::OwnedAttribute], element_namespace: &ex_em_ell::xml::namespace::Namespace) -> Result<Self, ex_em_ell::errors::XmlReadError>
            {
                #read_xml_element
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

// Add a bound `T: FromXmlElement` to every type parameter T.
fn add_from_xml_element_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(ex_em_ell::FromXmlElement));
        }
    }
    generics
}

#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(ToXmlDocument, attributes(ex_em_ell))]
pub fn enecode_derive_document(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let writer_variable = format_ident!("_{}", "writer");
    let write_xml_document = generate_write_xml_document(&input, &writer_variable);

    let name = input.ident;

    // Add a bound `T: FromXmlElement` to every type parameter T.
    let generics = add_to_xml_element_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let expanded = quote! {
        // The generated impl.
        impl #impl_generics ex_em_ell::traits::ToXmlDocument for #name #ty_generics #where_clause {
            fn to_xml_document<W: std::io::Write>(self: &Self, #writer_variable: &mut ex_em_ell::xml::EventWriter<W>) -> Result<(), ex_em_ell::errors::XmlWriteError>
            {
                #write_xml_document

                Ok(())
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(ToXmlElement, attributes(ex_em_ell))]
pub fn enecode_derive_element(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let writer_variable = format_ident!("_{}", "writer");
    let tag_name_variable = format_ident!("_{}", "tag_name");

    let write_xml_element =
        generate_write_xml_element(&input, &writer_variable, &tag_name_variable);

    let name = input.ident;

    // Add a bound `T: FromXmlElement` to every type parameter T.
    let generics = add_to_xml_element_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let expanded = quote! {
        // The generated impl.
        impl #impl_generics ex_em_ell::traits::ToXmlElement for #name #ty_generics #where_clause {
            fn to_xml_element<W: std::io::Write>(self: &Self, #writer_variable: &mut ex_em_ell::xml::EventWriter<W>, #tag_name_variable: &str) -> Result<(), ex_em_ell::errors::XmlWriteError>
            {
                #write_xml_element

                Ok(())
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
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
