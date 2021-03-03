use std::{error, fmt};

pub type OBIResult<T> = Result<T, OBIError>;

#[derive(Debug)]
pub enum OBIError {
    Encode,
    Decode,
    Image,
}

impl fmt::Display for OBIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OBIError::Encode => writeln!(f, "Encoding error"),
            OBIError::Decode => writeln!(f, "Decoding error"),
            OBIError::Image => writeln!(f, "Image manipulation error"),
        }
    }
}

impl error::Error for OBIError {}

#[derive(Debug)]
enum Encode {
    Metadata(MetadataError),
    ChunkError(u32),
}

#[derive(Debug)]
enum Decode {
    Metadata(MetadataError),
    ChunkError(u32),
}

#[derive(Debug)]
enum MetadataError {
    VersionError,
    FileSizeError,
    DataOffsetError,
    WidthError,
    HeightError,
    CompressionTypeError,
    PostCompressionSizeError,
}
