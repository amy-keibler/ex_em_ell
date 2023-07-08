use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum XmlWriteError {}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum XmlReadError {}
