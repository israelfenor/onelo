//! This module is concerned with the artefact checksums.

use arrayvec::ArrayString;
use blake3::{self, Hash, OUT_LEN};
use std::array::TryFromSliceError;
use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

/// The multihash code.
pub type Code = u8;
const BLAKE3: Code = 0x1e;

pub type Byteset = [u8; OUT_LEN];
pub type Hex = ArrayString<[u8; 64]>;

#[derive(Debug, Clone, PartialEq)]
pub struct Checksum {
    code: Code,
    hash: Hash,
}

impl Checksum {
    /// Hashes the given bytes.
    ///
    /// If you need to cast a `Byteset` as a `Checksum` use the `From` implementations.
    ///
    /// ## Examples
    pub fn new(input: &[u8]) -> Self {
        Self {
            code: BLAKE3,
            hash: blake3::hash(input),
        }
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        self.hash.as_bytes()
    }

    pub fn to_hex(&self) -> Hex {
        self.hash.to_hex()
    }

    fn wrap(hash: Hash) -> Self {
        Self {
            code: BLAKE3,
            hash: hash.into(),
        }
    }

    pub fn unwrap(&self) -> Hash {
        self.hash
    }
}

impl From<Byteset> for Checksum {
    #[inline]
    fn from(bytes: [u8; OUT_LEN]) -> Self {
        Self::wrap(bytes.into())
    }
}

impl From<Checksum> for Byteset {
    #[inline]
    fn from(checksum: Checksum) -> Self {
        checksum.hash.into()
    }
}

impl fmt::Display for Checksum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0x}{}", self.code, self.to_hex())
    }
}

impl FromStr for Checksum {
    type Err = ChecksumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hash_bytes = hex::decode(s)?;
        // let hash_array: [u8; OUT_LEN] = hash_bytes[..].try_into()?;
        let code: u8 = hash_bytes[0];
        dbg!(&code);
        let hash_array: [u8; OUT_LEN] = hash_bytes[1..].try_into()?;
        let hash: Hash = hash_array.into();

        Ok(Checksum::wrap(hash))
    }
}

#[derive(Debug)]
pub enum ChecksumError {
    Bad,
    UnexpectedLength,
    Hex(hex::FromHexError),
}

impl fmt::Display for ChecksumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChecksumError::Bad => write!(f, "bad"),
            ChecksumError::Hex(err) => write!(f, "{}", err),
            ChecksumError::UnexpectedLength => {
                write!(f, "The given slice cannot be casted as a checksum")
            }
        }
    }
}

impl Error for ChecksumError {}

impl From<hex::FromHexError> for ChecksumError {
    fn from(err: hex::FromHexError) -> Self {
        ChecksumError::Hex(err)
    }
}

impl From<TryFromSliceError> for ChecksumError {
    fn from(_err: TryFromSliceError) -> Self {
        ChecksumError::UnexpectedLength
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blake3_checksum() {
        let actual = Checksum::new(b"onelo");
        let expected = "1ecabe0427e7fdaa13ec1d49de58a6179a2ecb6dd6fd674261421949fab0acc525";

        assert_eq!(actual.to_string(), expected);
    }

    #[test]
    fn parse() -> Result<(), ChecksumError> {
        let hex = "1ecabe0427e7fdaa13ec1d49de58a6179a2ecb6dd6fd674261421949fab0acc525";
        let actual: Checksum = hex.parse()?;
        let expected = Checksum::new(b"onelo");

        assert_eq!(actual, expected);

        Ok(())
    }
}
