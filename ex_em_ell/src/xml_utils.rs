use itertools::Itertools;
use std::io::{Read, Write};

use xml::{name::OwnedName, reader, writer, EventReader, EventWriter};

use crate::{
    errors::{XmlReadError, XmlWriteError},
    FromXmlElement,
};

/// Write a tag that is of the form `<tag>content</tag>`
pub fn write_simple_tag<W: Write>(
    writer: &mut EventWriter<W>,
    tag: &str,
    content: &str,
) -> Result<(), XmlWriteError> {
    writer
        .write(writer::XmlEvent::start_element(tag))
        .map_err(to_xml_write_error(tag))?;

    writer
        .write(writer::XmlEvent::characters(content))
        .map_err(to_xml_write_error(tag))?;

    writer
        .write(writer::XmlEvent::end_element())
        .map_err(to_xml_write_error(tag))?;
    Ok(())
}

pub fn read_simple_tag<R: Read>(
    event_reader: &mut EventReader<R>,
    element: &OwnedName,
) -> Result<String, XmlReadError> {
    let element_display = element.to_string();
    let content = event_reader
        .next()
        .map_err(to_xml_read_error(&element_display))
        .and_then(inner_text_or_error(&element_display))?;

    event_reader
        .next()
        .map_err(to_xml_read_error(&element_display))
        .and_then(closing_tag_or_error(element))?;

    Ok(content)
}

pub fn read_list_tag<R: Read, T: FromXmlElement>(
    event_reader: &mut EventReader<R>,
    element_name: &OwnedName,
    inner_element_tag: &str,
) -> Result<Vec<T>, XmlReadError> {
    let mut items = Vec::new();

    let mut got_end_tag = false;
    while !got_end_tag {
        let next_element = event_reader
            .next()
            .map_err(to_xml_read_error(&element_name.local_name))?;
        match next_element {
            reader::XmlEvent::StartElement {
                name,
                attributes,
                namespace,
                ..
            } if name.local_name == inner_element_tag => {
                items.push(T::from_xml_element(
                    event_reader,
                    &name,
                    &attributes,
                    &namespace,
                )?);
            }
            reader::XmlEvent::EndElement { name } if &name == element_name => {
                got_end_tag = true;
            }
            unexpected => {
                return Err(unexpected_element_with_known_values_error(
                    element_name,
                    vec![inner_element_tag.to_string()],
                    unexpected,
                ))
            }
        }
    }

    Ok(items)
}

pub fn inner_text_or_error(
    element_name: impl AsRef<str>,
) -> impl FnOnce(xml::reader::XmlEvent) -> Result<String, XmlReadError> {
    let element_name = element_name.as_ref().to_owned();
    |event| match event {
        reader::XmlEvent::Characters(s) | reader::XmlEvent::CData(s) => Ok(s),
        unexpected => Err(unexpected_element_error(element_name, unexpected)),
    }
}

pub fn closing_tag_or_error(
    element: &OwnedName,
) -> impl FnOnce(xml::reader::XmlEvent) -> Result<(), XmlReadError> {
    let element = element.clone();
    move |event| match event {
        reader::XmlEvent::EndElement { name } if name == element => Ok(()),
        unexpected => Err(unexpected_element_error(&element, unexpected)),
    }
}

pub fn to_xml_write_error(
    element: impl AsRef<str>,
) -> impl FnOnce(xml::writer::Error) -> XmlWriteError {
    let element = element.as_ref().to_owned();
    |error| XmlWriteError::XmlElementWriteError { error, element }
}

pub fn to_xml_read_error(
    element_name: impl AsRef<str>,
) -> impl FnOnce(xml::reader::Error) -> XmlReadError {
    let element_name = element_name.as_ref().to_owned();
    |error| XmlReadError::ElementReadError {
        error,
        element: element_name,
    }
}

pub fn unexpected_element_error(
    element: impl ToString,
    unexpected: xml::reader::XmlEvent,
) -> XmlReadError {
    XmlReadError::UnexpectedElementReadError {
        error: format!("Got unexpected element {:?}", unexpected),
        element: element.to_string(),
    }
}

pub fn unexpected_element_with_known_values_error(
    element: impl ToString,
    valid_elements: Vec<String>,
    unexpected: xml::reader::XmlEvent,
) -> XmlReadError {
    XmlReadError::UnexpectedElementReadError {
        error: format!(
            "Got unexpected element {:?}, expected one of: {}",
            unexpected,
            valid_elements.iter().join(", ")
        ),
        element: element.to_string(),
    }
}
