use crate::blobs::DecoderBlob;
use crate::blobs::DecoderResultBlob;
use crate::blobs::ListBlob;
use crate::blobs::StringPtrBlob;
use crate::containers::ExternalList as List;
use crate::containers::ExternalStringPtr as StringPtr;
use crate::containers::IntoBlob;
use crate::source::DecoderResult;
use crate::source::InputError;
use crate::source::MaybeDecoder;
use crate::source::MaybeDecoderAPI;

#[cfg(test)]
impl Default for DecoderBlob {
    fn default() -> Self {
        let bytes: [u8; std::mem::size_of::<Self>()] = [0; std::mem::size_of::<Self>()];
        Self { bytes }
    }
}

/// Custom decoder, a wrapper around a function
#[repr(C)]
pub struct Decoder {
    pub(crate) blob: DecoderBlob,
}

extern "C" {
    fn lib_ruby_parser__external__decoder__call(
        blob: *mut DecoderBlob,
        encoding: StringPtrBlob,
        input: ListBlob,
    ) -> DecoderResultBlob;
    fn lib_ruby_parser__external__decoder_drop(blob: *mut DecoderBlob);
}

impl Drop for Decoder {
    fn drop(&mut self) {
        unsafe { lib_ruby_parser__external__decoder_drop(&mut self.blob) }
    }
}

impl Decoder {
    pub(crate) fn call(&mut self, encoding: StringPtr, input: List<u8>) -> DecoderResult {
        DecoderResult::from_blob(unsafe {
            lib_ruby_parser__external__decoder__call(
                &mut self.blob,
                encoding.into_blob(),
                input.into_blob(),
            )
        })
    }

    #[allow(dead_code)]
    pub(crate) fn from_blob(blob: DecoderBlob) -> Self {
        Self { blob }
    }
}

impl std::fmt::Debug for Decoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Decoder").finish()
    }
}

pub fn decode_input(input: List<u8>, enc: StringPtr, decoder: &mut MaybeDecoder) -> DecoderResult {
    match enc.to_uppercase().as_str() {
        "UTF-8" | "ASCII-8BIT" | "BINARY" => {
            return DecoderResult::new_ok(input.into());
        }
        _ => {
            if let Some(decoder) = decoder.as_decoder_mut() {
                decoder.call(enc, input)
            } else {
                DecoderResult::new_err(InputError::new_unsupported_encoding(enc))
            }
        }
    }
}
