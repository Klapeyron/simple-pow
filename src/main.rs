use hex::FromHexError;
use sha2::{digest::Output, Digest, Sha256};
use std::array::TryFromSliceError;
use std::convert::TryInto;
use thiserror::Error;

#[derive(Error)]
pub enum PrefixSearchError {
    #[error("Required at least one argument with 64-byte encoded string")]
    NotEnoughCommandLineArguments,
    #[error("Hex-encoded input should be 64-bytes long")]
    InputInvalidLength(#[from] TryFromSliceError),
    #[error("Unable to decode hexdecimal encoded input: {0}")]
    HexDecodeError(#[from] FromHexError),
    #[error("Prefix matching given predicate not found")]
    PrefixNotFound,
}

/// Standard library is printing main error via [`Debug`] not [`Display`] (see [RFC 1937](https://github.com/rust-lang/rfcs/pull/1937)).
///
/// In order to print more human readable errors, we print [`Debug`] as [`Display`]
impl std::fmt::Debug for PrefixSearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

#[derive(Debug, PartialEq)]
struct Input {
    checksum: [u8; 64],
}

/// Iterator instance over sha256 checksums created from combination of all 4 byte permutations
/// with input byte array
struct PrefixedChecksumIterator {
    prefix: std::ops::Range<u32>,
    hasher: Sha256,
    input: Input,
}

struct Chunk {
    /// Result of `sha256([prefix] + [input])` operation
    checksum: Output<Sha256>,
    /// Prefix used to calculate sha256
    prefix: [u8; 4],
}

impl IntoIterator for Input {
    type Item = Chunk;
    type IntoIter = PrefixedChecksumIterator;

    fn into_iter(self) -> Self::IntoIter {
        PrefixedChecksumIterator {
            prefix: (0..u32::MAX),
            hasher: sha2::Sha256::new(),
            input: self,
        }
    }
}

impl Iterator for PrefixedChecksumIterator {
    type Item = Chunk;

    fn next(&mut self) -> Option<Self::Item> {
        let prefix = self.prefix.next()?.to_be_bytes();
        self.hasher.update(prefix);
        self.hasher.update(&self.input.checksum[..]);

        let checksum = self.hasher.finalize_reset();

        Some(Chunk { checksum, prefix })
    }
}

fn main() -> Result<(), PrefixSearchError> {
    let input = std::env::args()
        .into_iter()
        .nth(1) // skip name of the app @ argv[0]
        .ok_or(PrefixSearchError::NotEnoughCommandLineArguments)?;

    let input = Input {
        checksum: hex::decode(input)?[..].try_into()?,
    };

    let Chunk { checksum, prefix } = input
        .into_iter()
        .find(|Chunk { checksum, .. }| checksum.ends_with(&[0xca, 0xfe]))
        .ok_or(PrefixSearchError::PrefixNotFound)?;

    println!("{}", hex::encode(checksum));
    println!("{}", hex::encode(prefix));

    Ok(())
}
