use heck::ToLowerCamelCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics};

#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(FromXmlDocument, attributes(ex_em_ell))]
pub fn decode_derive_document(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let read_xml_document = generate_read_xml_document(&input);

    let name = input.ident;

    // Add a bound `T: FromXmlElement` to every type parameter T.
    let generics = add_from_xml_element_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let expanded = quote! {
        // The generated impl.
        impl #impl_generics ex_em_ell::traits::FromXmlDocument for #name #ty_generics #where_clause {
            fn from_xml_document<R: std::io::Read>(reader: &mut ex_em_ell::xml::EventReader<R>) -> Result<Self, ex_em_ell::errors::XmlReadError>
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

    let read_xml_element = generate_read_xml_element(&input);

    let name = input.ident;

    // Add a bound `T: FromXmlElement` to every type parameter T.
    let generics = add_from_xml_element_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let expanded = quote! {
        // The generated impl.
        impl #impl_generics ex_em_ell::traits::FromXmlElement for #name #ty_generics #where_clause {
            fn from_xml_element<R: std::io::Read>(reader: &mut ex_em_ell::xml::EventReader<R>, element_name: &ex_em_ell::xml::name::OwnedName, element_attributes: &[ex_em_ell::xml::attribute::OwnedAttribute], element_namespace: &ex_em_ell::xml::namespace::Namespace) -> Result<Self, ex_em_ell::errors::XmlReadError>
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

fn generate_read_xml_document(input: &DeriveInput) -> TokenStream {
    let tag_name = input.ident.to_string().to_lower_camel_case();

    let (variable_declarations, state_machine, required_variables, struct_fields): (
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
                                                                     reader,
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
                                                                             element: #tag_name.to_string(),
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
                        let next_element = reader
                            .next()
                            .map_err(ex_em_ell::xml_utils::to_xml_read_error(#tag_name))?;
                        match next_element {
                            #state_machine_arms_recurse
                            ex_em_ell::xml::reader::XmlEvent::EndElement { name } if &name.to_string() == #tag_name => {
                                got_end_tag = true;
                            }
                            unexpected => return Err(ex_em_ell::xml_utils::unexpected_element_error(#tag_name, unexpected)),
                        }
                    }

                };

                let required_variables: TokenStream =
                    required_variable_declarations_recurse.into_iter().collect();

                let struct_fields: TokenStream = struct_fields_recurse.into_iter().collect();

                (
                    variable_declarations,
                    state_machine,
                    required_variables,
                    struct_fields,
                )
            }
            Fields::Unnamed(ref fields) => unimplemented!(),
            Fields::Unit => unimplemented!(),
        },
        Data::Enum(_) => unimplemented!(),
        Data::Union(_) => unimplemented!(),
    };

    quote! {

            reader
                .next()
                .map_err(ex_em_ell::xml_utils::to_xml_read_error(#tag_name))
                .and_then(|event| match event {
                    ex_em_ell::xml::reader::XmlEvent::StartDocument { .. } => Ok(()),
                    unexpected => Err(ex_em_ell::xml_utils::unexpected_element_error(#tag_name, unexpected)),
                })?;

            reader
                .next()
                .map_err(ex_em_ell::xml_utils::to_xml_read_error(#tag_name))
                .and_then(|event| match event {
                    ex_em_ell::xml::reader::XmlEvent::StartElement {
                        name,
                        attributes,
                        namespace,
                    } if name.local_name == #tag_name => {
    Ok(())
                    }
                    unexpected => Err(ex_em_ell::xml_utils::unexpected_element_error(#tag_name, unexpected)),
                })?;

            #variable_declarations

            #state_machine

            reader
                .next()
                .map_err(ex_em_ell::xml_utils::to_xml_read_error(#tag_name))
                .and_then(|event| match event {
                    ex_em_ell::xml::reader::XmlEvent::EndDocument => Ok(()),
                    unexpected => Err(ex_em_ell::xml_utils::unexpected_element_error(#tag_name, unexpected)),
                })?;

            #required_variables

            Ok(Self {
                #struct_fields
            })
        }
}

fn generate_read_xml_element(input: &DeriveInput) -> TokenStream {
    let (variable_declarations, state_machine, required_variables, struct_fields): (
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
                                                                     reader,
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
                                                                             element: element_name.to_string(),
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
                        let next_element = reader
                            .next()
                            .map_err(ex_em_ell::xml_utils::to_xml_read_error(element_name.to_string()))?;
                        match next_element {
                            #state_machine_arms_recurse
                            ex_em_ell::xml::reader::XmlEvent::EndElement { name } if &name == element_name => {
                                got_end_tag = true;
                            }
                            unexpected => return Err(ex_em_ell::xml_utils::unexpected_element_error(element_name.to_string(), unexpected)),
                        }
                    }

                };

                let required_variables: TokenStream =
                    required_variable_declarations_recurse.into_iter().collect();

                let struct_fields: TokenStream = struct_fields_recurse.into_iter().collect();

                (
                    variable_declarations,
                    state_machine,
                    required_variables,
                    struct_fields,
                )
            }
            Fields::Unnamed(ref fields) => unimplemented!(),
            Fields::Unit => unimplemented!(),
        },
        Data::Enum(_) => unimplemented!(),
        Data::Union(_) => unimplemented!(),
    };

    quote! {
        #variable_declarations

        #state_machine

        #required_variables

        Ok(Self {
            #struct_fields
        })
    }
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

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(ToXmlElement, attributes(ex_em_ell))]
pub fn enecode_derive_element(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let write_xml_element = generate_write_xml_element(&input);

    let name = input.ident;

    // Add a bound `T: FromXmlElement` to every type parameter T.
    let generics = add_to_xml_element_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let expanded = quote! {
        // The generated impl.
        impl #impl_generics ex_em_ell::traits::ToXmlElement for #name #ty_generics #where_clause {
            fn to_xml_element<W: std::io::Write>(self: &Self, writer: &mut ex_em_ell::xml::EventWriter<W>, tag: &str) -> Result<(), ex_em_ell::errors::XmlWriteError>
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

fn generate_write_xml_document(input: &DeriveInput) -> TokenStream {
    let tag_name = input.ident.to_string().to_lower_camel_case();

    let field_writers: TokenStream = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f
                        .ident
                        .as_ref()
                        .expect("Named field should have an identifier");
                    let field_tag_name = name.to_string().to_lower_camel_case();

                    quote_spanned! { f.span() =>
                       ex_em_ell::traits::ToXmlElement::to_xml_element(&self.#name, writer, #field_tag_name)?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(ref fields) => unimplemented!(),
            Fields::Unit => unimplemented!(),
        },
        Data::Enum(_) => unimplemented!(),
        Data::Union(_) => unimplemented!(),
    };

    quote! {
        writer.write(ex_em_ell::xml::writer::XmlEvent::start_element(#tag_name)).map_err(ex_em_ell::xml_utils::to_xml_write_error(#tag_name))?;

        #field_writers

        writer.write(ex_em_ell::xml::writer::XmlEvent::end_element()).map_err(ex_em_ell::xml_utils::to_xml_write_error(#tag_name))?;

    }
}

// TODO: pass in an identifier rather than assuming one (and then simplify this with generate_write_xml_document, with the ability to handle namespaces)
fn generate_write_xml_element(input: &DeriveInput) -> TokenStream {
    let field_writers: TokenStream = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f
                        .ident
                        .as_ref()
                        .expect("Named field should have an identifier");
                    let field_tag_name = name.to_string().to_lower_camel_case();

                    quote_spanned! { f.span() =>
                       ex_em_ell::traits::ToXmlElement::to_xml_element(&self.#name, writer, #field_tag_name)?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(ref fields) => unimplemented!(),
            Fields::Unit => unimplemented!(),
        },
        Data::Enum(_) => unimplemented!(),
        Data::Union(_) => unimplemented!(),
    };

    quote! {
        writer.write(ex_em_ell::xml::writer::XmlEvent::start_element(tag)).map_err(ex_em_ell::xml_utils::to_xml_write_error(tag))?;

        #field_writers

        writer.write(ex_em_ell::xml::writer::XmlEvent::end_element()).map_err(ex_em_ell::xml_utils::to_xml_write_error(tag))?;

    }
}
