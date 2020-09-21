//! This module is concerned with the unique content found in sources.

// TODO: Promote to a Type that encodes the hashing algorithm e.g. Blake3
pub type Checksum = String;

// TODO: We want to keep raw bytes here, so we can delay UTF-8 enforcement to when it's needed.
/// A sequence of bytes for a piece of content.
pub type Blob = Vec<u8>;

/// A piece of content.
///
/// A piece of content is the unprocessed blob obtained from, for example, a file found in a source directory.
///
/// If you need to understand the type of content e.g. `ContentType`, check the `SourceEntry`.
#[derive(Debug)]
pub struct Content {
    id: Checksum,
    blob: Blob,
}

impl Content {
    /// A convenient constructor for when both checksum and blob are known.
    pub fn new(id: Checksum, blob: Blob) -> Self {
        Content { id, blob }
    }

    pub fn checksum(&self) -> &Checksum {
        &self.id
    }
}
