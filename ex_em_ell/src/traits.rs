use std::io::{Read, Write};
use xml::{
    attribute::OwnedAttribute, name::OwnedName, namespace::Namespace, EventReader, EventWriter,
};

use crate::{
    errors::{XmlReadError, XmlWriteError},
    xml_utils::{read_simple_tag, write_simple_tag},
};

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
        write_simple_tag(writer, tag, self)
    }
}

impl FromXmlElement for String {
    fn from_xml_element<R: Read>(
        reader: &mut EventReader<R>,
        element_name: &OwnedName,
        _element_attributes: &[OwnedAttribute],
        _element_namespace: &Namespace,
    ) -> Result<Self, XmlReadError>
    where
        Self: Sized,
    {
        read_simple_tag(reader, element_name)
    }
}

impl ToXmlElement for bool {
    fn to_xml_element<W: Write>(
        self: &Self,
        writer: &mut EventWriter<W>,
        tag: &str,
    ) -> Result<(), XmlWriteError> {
        write_simple_tag(writer, tag, &self.to_string())
    }
}

impl FromXmlElement for bool {
    fn from_xml_element<R: Read>(
        reader: &mut EventReader<R>,
        element_name: &OwnedName,
        _element_attributes: &[OwnedAttribute],
        _element_namespace: &Namespace,
    ) -> Result<Self, XmlReadError>
    where
        Self: Sized,
    {
        read_simple_tag(reader, element_name).and_then(|value| {
            let value = value.as_ref();
            match value {
                "true" | "1" => Ok(true),
                "false" | "0" => Ok(false),
                _ => Err(XmlReadError::InvalidParseError {
                    value: value.to_string(),
                    data_type: "xs:boolean".to_string(),
                    element: element_name.to_string(),
                }),
            }
        })
    }
}
