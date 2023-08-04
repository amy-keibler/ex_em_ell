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

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum XmlReadError {}
