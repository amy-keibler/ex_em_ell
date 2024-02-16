use darling::FromMeta;
use heck::ToLowerCamelCase;
use itertools::Itertools;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Expr, Fields};

pub(crate) fn generate_write_xml_document(
    input: &DeriveInput,
    writer_variable: &Ident,
) -> TokenStream {
    let tag_name = input.ident.to_string().to_lower_camel_case();

    let tag_name_variable = format_ident!("_{}", "tag_name");

    let writer = generate_write_xml_element(input, &writer_variable, &tag_name_variable);

    quote! {
        let #tag_name_variable = #tag_name;

        #writer
    }
}

pub(crate) fn generate_write_xml_element(
    input: &DeriveInput,
    writer_variable: &Ident,
    tag_name_variable: &Ident,
) -> TokenStream {
    let field_writers: TokenStream = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f
                        .ident
                        .as_ref()
                        .expect("Named field should have an identifier");

                    let write_attrs: WriteAttrs = f.attrs.iter().find_map(|attr| FromMeta::from_meta(&attr.meta).ok()).unwrap_or_default();
                    let field_tag_name = write_attrs.rename.unwrap_or_else(|| name.to_string().to_lower_camel_case());

                    quote_spanned! { f.span() =>
                       ex_em_ell::traits::ToXmlElement::to_xml_element(&self.#name, #writer_variable, #field_tag_name)?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(_) => unimplemented!(),
            Fields::Unit => unimplemented!(),
        },
        Data::Enum(_) => unimplemented!(),
        Data::Union(_) => unimplemented!(),
    };

    quote! {
        #writer_variable.write(ex_em_ell::xml::writer::XmlEvent::start_element(#tag_name_variable)).map_err(ex_em_ell::xml_utils::to_xml_write_error(#tag_name_variable))?;

        #field_writers

        #writer_variable.write(ex_em_ell::xml::writer::XmlEvent::end_element()).map_err(ex_em_ell::xml_utils::to_xml_write_error(#tag_name_variable))?;

    }
}

#[derive(Debug, Default, FromMeta)]
struct WriteAttrs {
    #[darling(default)]
    rename: Option<String>,
}
