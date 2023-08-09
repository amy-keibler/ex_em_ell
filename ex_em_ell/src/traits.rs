use std::io::{Read, Write};
use xml::{
    attribute::OwnedAttribute, name::OwnedName, namespace::Namespace, EventReader, EventWriter,
};

use crate::errors::{XmlReadError, XmlWriteError};

pub trait ToXmlDocument {
    fn to_xml_document<W: Write>(
        self: &Self,
        writer: &mut EventWriter<W>,
    ) -> Result<(), XmlWriteError>;
}

pub trait ToXmlElement {
    fn to_xml_element<W: Write>(
        self: &Self,
        writer: &mut EventWriter<W>,
        tag: &str,
    ) -> Result<(), XmlWriteError>;

    fn will_write(self: &Self) -> bool {
        true
    }
}

pub trait FromXmlDocument {
    fn from_xml_document<R: Read>(reader: &mut EventReader<R>) -> Result<Self, XmlReadError>
    where
        Self: Sized;
}

pub trait FromXmlElement {
    fn from_xml_element<R: Read>(
        reader: &mut EventReader<R>,
        element_name: &OwnedName,
        element_attributes: &[OwnedAttribute],
        element_namespace: &Namespace,
    ) -> Result<Self, XmlReadError>
    where
        Self: Sized;
}

impl ToXmlElement for String {
    fn to_xml_element<W: Write>(
        self: &Self,
        writer: &mut EventWriter<W>,
        tag: &str,
    ) -> Result<(), XmlWriteError> {
        crate::xml_utils::write_simple_tag(writer, tag, self)
    }
}
