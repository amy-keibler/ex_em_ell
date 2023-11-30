use heck::ToLowerCamelCase;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields};

pub(crate) fn generate_read_xml_document(
    input: &DeriveInput,
    reader_variable: &Ident,
) -> TokenStream {
    let tag_name = input.ident.to_string().to_lower_camel_case();

    // Borrow the tag OwnedName so it can be consistent with what's passed to FromXmlElement
    let tag_name_variable = format_ident!("_{}", "tag_name");
    let tag_name_borrowed_variable = format_ident!("_{}_borrowed", tag_name_variable);

    let (code, return_expression) =
        generate_read(input, reader_variable, &tag_name_borrowed_variable);

    quote! {
        #reader_variable
                .next()
                .map_err(ex_em_ell::xml_utils::to_xml_read_error(#tag_name))
                .and_then(|event| match event {
                    ex_em_ell::xml::reader::XmlEvent::StartDocument { .. } => Ok(()),
                    unexpected => Err(ex_em_ell::xml_utils::unexpected_element_error(#tag_name, unexpected)),
                })?;

            let #tag_name_variable = #reader_variable
                .next()
                .map_err(ex_em_ell::xml_utils::to_xml_read_error(#tag_name))
                .and_then(|event| match event {
                    ex_em_ell::xml::reader::XmlEvent::StartElement {
                        name,
                        attributes,
                        namespace,
                    } if name.local_name == #tag_name => {
    Ok(name)
                    }
                    unexpected => Err(ex_em_ell::xml_utils::unexpected_element_error(#tag_name, unexpected)),
                })?;
            let #tag_name_borrowed_variable = &#tag_name_variable;

            #code

            #reader_variable
                .next()
                .map_err(ex_em_ell::xml_utils::to_xml_read_error(#tag_name))
                .and_then(|event| match event {
                    ex_em_ell::xml::reader::XmlEvent::EndDocument => Ok(()),
                    unexpected => Err(ex_em_ell::xml_utils::unexpected_element_error(#tag_name, unexpected)),
                })?;

            #return_expression
        }
}

pub(crate) fn generate_read_xml_element(
    input: &DeriveInput,
    reader_variable: &Ident,
    tag_name_variable: &Ident,
) -> TokenStream {
    let (code, return_expression) = generate_read(input, reader_variable, tag_name_variable);

    quote! {
        #code

        #return_expression
    }
}

fn generate_read(
    input: &DeriveInput,
    reader_variable: &Ident,
    tag_name_variable: &Ident,
) -> (TokenStream, TokenStream) {
    let (variable_declarations, state_machine, required_variables, output): (
        TokenStream,
        TokenStream,
        TokenStream,
        TokenStream,
    ) = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let (
                    variable_declarations_recurse,
                    state_machine_arms_recurse,
                    required_variable_declarations_recurse,
                    struct_fields_recurse,
                ): (
                    Vec<TokenStream>,
                    Vec<TokenStream>,
                    Vec<TokenStream>,
                    Vec<TokenStream>,
                ) = itertools::multiunzip(fields.named.iter().map(|f| {
                    let name = &f
                        .ident
                        .as_ref()
                        .expect("Named field should have an identifier");

                    let variable = format_ident!("_{}", name);

                    let variable_type = &f.ty;

                    let field_tag_name = name.to_string().to_lower_camel_case();

                    let variable_declaration = quote_spanned! { f.span() =>
                                     let mut #variable : Option<#variable_type> = None;
                    };

                    let state_machine_arm = quote_spanned! { f.span() =>
                                                             ex_em_ell::xml::reader::XmlEvent::StartElement {
                                                                 name, attributes, namespace, ..
                                                             } if name.local_name == #field_tag_name => {
                                                                 #variable = Some(ex_em_ell::traits::FromXmlElement::from_xml_element(
                                                                     #reader_variable,
                                                                     &name,
                                                                     &attributes,
                                                                     &namespace,
                                                                 )?)
                                                             }
                    };

                    let required_variable = format_ident!("{}_required", variable);

                    let required_variable_declaration = quote_spanned! { f.span() =>
                                                                         let #required_variable: #variable_type = #variable.ok_or_else(|| ex_em_ell::errors::XmlReadError::RequiredDataMissing {
                                                                             required_field: #field_tag_name.to_string(),
                                                                             element: #tag_name_variable.to_string(),
                                                                         })?;
                    };

                    let struct_field = quote_spanned! { f.span() =>
                                                        #name: #required_variable,
                    };

                    (
                        variable_declaration,
                        state_machine_arm,
                        required_variable_declaration,
                        struct_field,
                    )
                }));
                let variable_declarations: TokenStream =
                    variable_declarations_recurse.into_iter().collect();

                let state_machine_arms_recurse: TokenStream =
                    state_machine_arms_recurse.into_iter().collect();

                let state_machine = quote! {
                    let mut got_end_tag = false;
                    while !got_end_tag {
                        let next_element = #reader_variable
                            .next()
                            .map_err(ex_em_ell::xml_utils::to_xml_read_error(#tag_name_variable.to_string()))?;
                        match next_element {
                            #state_machine_arms_recurse
                            ex_em_ell::xml::reader::XmlEvent::EndElement { name } if &name == #tag_name_variable => {
                                got_end_tag = true;
                            }
                            unexpected => return Err(ex_em_ell::xml_utils::unexpected_element_error(#tag_name_variable.to_string(), unexpected)),
                        }
                    }

                };

                let required_variables: TokenStream =
                    required_variable_declarations_recurse.into_iter().collect();

                let struct_fields: TokenStream = struct_fields_recurse.into_iter().collect();

                let output: TokenStream = quote! {
                    Self {
                        #struct_fields
                    }
                };

                (
                    variable_declarations,
                    state_machine,
                    required_variables,
                    output,
                )
            }
            Fields::Unnamed(ref fields) => {
                if fields.unnamed.len() != 1 {
                    panic!("Currently only single element tuple structs are supported");
                }
                let field = fields
                    .unnamed
                    .first()
                    .expect("Expected a field on the tuple struct");

                todo!();
            }
            Fields::Unit => unimplemented!(),
        },
        Data::Enum(_) => unimplemented!(),
        Data::Union(_) => unimplemented!(),
    };

    (
        quote! {
        #variable_declarations

        #state_machine

        #required_variables
        },
        quote! {

        Ok(#output)},
    )
}
