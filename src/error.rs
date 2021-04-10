use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("RTP header size insufficient")]
    errHeaderSizeInsufficient,
    #[error("RTP header size insufficient for extension")]
    errHeaderSizeInsufficientForExtension,
    #[error("buffer too small")]
    errTooSmall,
    #[error("extension not enabled")]
    errHeaderExtensionsNotEnabled,
    #[error("extension not found")]
    errHeaderExtensionNotFound,

    #[error("header extension id must be between 1 and 14 for RFC 5285 extensions")]
    errRFC8285OneByteHeaderIDRange,
    #[error("header extension payload must be 16bytes or less for RFC 5285 one byte extensions")]
    errRFC8285OneByteHeaderSize,

    #[error("header extension id must be between 1 and 255 for RFC 5285 extensions")]
    errRFC8285TwoByteHeaderIDRange,
    #[error("header extension payload must be 255bytes or less for RFC 5285 two byte extensions")]
    errRFC8285TwoByteHeaderSize,

    #[error("header extension id must be 0 for none RFC 5285 extensions")]
    errRFC3550HeaderIDRange,

    #[error("extension_payload must be in 32-bit words")]
    HeaderExtensionPayloadNot32BitWords,
    #[error("audio level overflow")]
    AudioLevelOverflow,
    #[error("payload is not large enough")]
    PayloadIsNotLargeEnough,
    #[error("STAP-A declared size({0}) is larger than buffer({1})")]
    StapASizeLargerThanBuffer(usize, usize),
    #[error("nalu type {0} is currently not handled")]
    NaluTypeIsNotHandled(u8),
    #[error("SystemTimeError: {0}")]
    SystemTime(#[from] std::time::SystemTimeError),
    #[error("IoError: {0}")]
    Io(#[from] std::io::Error),
}
