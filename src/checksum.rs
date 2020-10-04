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
pub type Length = u8;

const BLAKE3_CODE: Code = 0x1e;
const BLAKE3_LEN: Length = 0x20;

pub type Byteset = [u8; OUT_LEN];
pub type Hex = ArrayString<[u8; 64]>;

#[derive(Debug, Clone, PartialEq)]
pub struct Checksum {
    /// The algorithm code https://github.com/multiformats/multicodec/blob/master/table.csv
    code: Code,
    /// The hash length https://github.com/multiformats/multihash
    len: Length,
    hash: Hash,
}

impl Checksum {
    /// Hashes the given bytes.
    ///
    /// If you need to cast a `Byteset` as a `Checksum` use the `From` implementations.
    ///
    /// ## Examples
    ///
    /// ```
    /// use onelo_backend::checksum::Checksum;
    ///
    /// let chksum = Checksum::new(b"onelo");
    /// let expected = "1e20cabe0427e7fdaa13ec1d49de58a6179a2ecb6dd6fd674261421949fab0acc525";
    ///
    /// assert_eq!(chksum.to_string(), expected);
    /// ```
    pub fn new(input: &[u8]) -> Self {
        Self::wrap(blake3::hash(input))
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        self.hash.as_bytes()
    }

    pub fn to_hex(&self) -> Hex {
        self.hash.to_hex()
    }

    fn wrap(hash: Hash) -> Self {
        Self {
            code: BLAKE3_CODE,
            len: BLAKE3_LEN,
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
        write!(f, "{:0x}{:0x}{}", self.code, self.len, self.to_hex())
    }
}

impl FromStr for Checksum {
    type Err = ChecksumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hash_bytes = hex::decode(s)?;
        let code: u8 = hash_bytes[0];
        let len: u8 = hash_bytes[1];

        if code != BLAKE3_CODE {
            return Err(ChecksumError::UnknownCode(code));
        }

        if len != BLAKE3_LEN {
            return Err(ChecksumError::InconsistentLength(code, len));
        }

        let hash_array: [u8; OUT_LEN] = hash_bytes[2..].try_into()?;
        let hash: Hash = hash_array.into();

        Ok(Checksum::wrap(hash))
    }
}

#[derive(Debug)]
pub enum ChecksumError {
    Bad,
    UnexpectedLength,
    UnknownCode(Code),
    InconsistentLength(Code, Length),
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
            ChecksumError::UnknownCode(code) => {
                write!(f, "The given checksum has an unknown code `{}`", code)
            }
            ChecksumError::InconsistentLength(code, len) => write!(
                f,
                "The given checksum has a code `{}` with an inconsistent length `{}`",
                code, len
            ),
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
        let expected = "1e20cabe0427e7fdaa13ec1d49de58a6179a2ecb6dd6fd674261421949fab0acc525";

        assert_eq!(actual.to_string(), expected);
    }

    #[test]
    fn parse() -> Result<(), ChecksumError> {
        let hex = "1e20cabe0427e7fdaa13ec1d49de58a6179a2ecb6dd6fd674261421949fab0acc525";
        let actual: Checksum = hex.parse()?;
        let expected = Checksum::new(b"onelo");

        assert_eq!(actual, expected);

        Ok(())
    }
}
