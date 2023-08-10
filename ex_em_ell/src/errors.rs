use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum XmlWriteError {
    #[error("Failed to serialize XML while writing {element}: {error}")]
    XmlElementWriteError {
        #[source]
        error: xml::writer::Error,
        element: String,
    },
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum XmlReadError {
    #[error("Failed to deserialize XML while reading {element}: {error}")]
    ElementReadError {
        #[source]
        error: xml::reader::Error,
        element: String,
    },
    #[error("Got unexpected XML element when reading {element}: {error}")]
    UnexpectedElementReadError { error: String, element: String },

    #[error("Ended element {element} without data for required field {required_field}")]
    RequiredDataMissing {
        required_field: String,
        element: String,
    },

    #[error("Could not parse {value} as {data_type} on {element}")]
    InvalidParseError {
        value: String,
        data_type: String,
        element: String,
    },

    #[error(
        "Expected document to be in the form {expected_namespace}, but received {}", .actual_namespace.as_ref().unwrap_or(&"no namespace".to_string())
    )]
    InvalidNamespaceError {
        expected_namespace: String,
        actual_namespace: Option<String>,
    },
}
