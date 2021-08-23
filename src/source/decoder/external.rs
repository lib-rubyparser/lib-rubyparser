use crate::containers::ExternalList as List;
use crate::containers::ExternalStringPtr as StringPtr;
use crate::source::InputError;

/// Decoder is what is used if input source has encoding
/// that is not supported out of the box.
///
/// Supported encoding are:
/// 1. UTF-8
/// 2. ASCII-8BIT (or BINARY, it's an alias)
///
/// So if your source looks like this:
///
/// ```text
/// # encoding: koi8-r
/// \xFF = 42
/// ```
///
/// you need to provide a decoder that converts this byte sequence
/// into UTF-8 bytes.
///
/// Decoding function
///
/// Takes encoding name and initial input as arguments
/// and returns `Ok(decoded)` vector of bytes or `Err(error)` that will be returned
/// in the `ParserResult::diagnostics` vector.
pub type DecoderFn = dyn Fn(StringPtr, List<u8>) -> DecoderResult;

/// Custom decoder, a wrapper around a function
pub struct Decoder {
    f: Box<DecoderFn>,
}

impl Decoder {
    /// Constructs a rewriter based on a given function
    pub fn new(f: Box<DecoderFn>) -> Self {
        Self { f }
    }

    pub(crate) fn call(&self, encoding: StringPtr, input: List<u8>) -> DecoderResult {
        let f = &*self.f;
        f(encoding, input)
    }
}

impl std::fmt::Debug for Decoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Decoder").finish()
    }
}

/// Result that is returned from decoding function
#[repr(C)]
#[derive(Debug)]
pub enum DecoderResult {
    /// Ok + decoded bytes
    Ok(List<u8>),

    /// Err + reason
    Err(InputError),
}

impl DecoderResult {
    pub(crate) fn to_result(self) -> Result<List<u8>, InputError> {
        match self {
            Self::Ok(value) => Ok(value),
            Self::Err(err) => Err(err),
        }
    }
}

pub fn decode_input(input: List<u8>, enc: StringPtr, decoder: &Option<Decoder>) -> DecoderResult {
    match enc.to_uppercase().as_str() {
        "UTF-8" | "ASCII-8BIT" | "BINARY" => {
            return DecoderResult::Ok(input.into());
        }
        _ => {
            if let Some(decoder) = decoder.as_ref() {
                decoder.call(enc, input)
            } else {
                DecoderResult::Err(InputError::new_unsupported_encoding(enc))
            }
        }
    }
}
