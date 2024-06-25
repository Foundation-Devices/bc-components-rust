use std::str::FromStr;
use dcbor::prelude::*;
use url::Url;
use anyhow::{ bail, Result, Error };

use crate::tags;

/// A URI.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct URI(String);

impl URI {
    /// Creates a new `URI` from a string.
    ///
    /// No validation is performed on the string.
    pub fn new(uri: impl Into<String>) -> Result<Self> {
        let uri = uri.into();
        if Url::parse(&uri).is_ok() {
            Ok(Self(uri))
        } else {
            bail!("Invalid URI")
        }
    }
}

impl FromStr for URI {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl AsRef<str> for URI {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<String> for URI {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl AsRef<URI> for URI {
    fn as_ref(&self) -> &URI {
        self
    }
}

impl CBORTagged for URI {
    fn cbor_tags() -> Vec<Tag> {
        vec![tags::URI]
    }
}

impl From<URI> for CBOR {
    fn from(value: URI) -> Self {
        value.tagged_cbor()
    }
}

impl CBORTaggedEncodable for URI {
    fn untagged_cbor(&self) -> CBOR {
        self.0.clone().into()
    }
}

impl TryFrom<CBOR> for URI {
    type Error = Error;

    fn try_from(cbor: CBOR) -> Result<Self, Self::Error> {
        Self::from_tagged_cbor(cbor)
    }
}

impl CBORTaggedDecodable for URI {
    fn from_untagged_cbor(cbor: CBOR) -> Result<Self> {
        let uri: String = cbor.try_into()?;
        Self::new(uri)
    }
}

impl std::fmt::Display for URI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Convert from a string to a URI.
impl From<&str> for URI {
    fn from(uri: &str) -> Self {
        Self::new(uri).unwrap()
    }
}

// Convert from a string to a URI.
impl From<String> for URI {
    fn from(uri: String) -> Self {
        Self::new(uri).unwrap()
    }
}

// Convert from a URI to a string.
impl From<URI> for String {
    fn from(uri: URI) -> Self {
        uri.0
    }
}

// Convert from a URI to a string.
impl From<&URI> for String {
    fn from(uri: &URI) -> Self {
        uri.0.clone()
    }
}
