use std::io::Write;

use xml::{writer, EventWriter};

use crate::errors::XmlWriteError;

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

pub fn to_xml_write_error(
    element: impl AsRef<str>,
) -> impl FnOnce(xml::writer::Error) -> XmlWriteError {
    let element = element.as_ref().to_owned();
    |error| XmlWriteError::XmlElementWriteError { error, element }
}
