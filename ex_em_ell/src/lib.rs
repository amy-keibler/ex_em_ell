#![doc = include_str!("../README.md")]

pub mod errors;
pub mod traits;
pub mod xml_utils;

use std::io::Read;

use errors::{XmlReadError, XmlWriteError};
use xml::{EmitterConfig, EventReader, EventWriter, ParserConfig};

#[cfg(feature = "derive")]
pub use ex_em_ell_derive::{FromXmlDocument, FromXmlElement, ToXmlDocument, ToXmlElement};

pub use traits::{FromXmlDocument, FromXmlElement, ToXmlDocument, ToXmlElement};
pub extern crate xml;

pub fn to_string<T: ToXmlDocument>(value: &T) -> Result<String, XmlWriteError> {
    to_string_with_config(value, EmitterConfig::default())
}

pub fn to_string_pretty<T: ToXmlDocument>(value: &T) -> Result<String, XmlWriteError> {
    let config = EmitterConfig::default().perform_indent(true);
    to_string_with_config(value, config)
}

fn to_string_with_config<T: ToXmlDocument>(
    value: &T,
    config: EmitterConfig,
) -> Result<String, XmlWriteError> {
    let mut output = Vec::new();
    let mut event_writer = EventWriter::new_with_config(&mut output, config);

    value.to_xml_document(&mut event_writer)?;

    let output = String::from_utf8_lossy(&output).to_string();
    Ok(output)
}

pub fn from_reader<T: FromXmlDocument, R: Read>(reader: R) -> Result<T, XmlReadError> {
    let config = ParserConfig::new().trim_whitespace(true);
    let mut event_reader = EventReader::new_with_config(reader, config);
    T::from_xml_document(&mut event_reader)
}
