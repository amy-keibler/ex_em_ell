pub mod errors;
pub mod traits;
pub mod xml_utils;

use errors::XmlWriteError;
use xml::{EmitterConfig, EventWriter};

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
