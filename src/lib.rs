#![allow(unreachable_patterns)]
#![allow(non_snake_case)]

pub mod decode;
pub mod encode;

#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OBIVersion {
    One,
}

impl OBIVersion {
    pub fn header_size(&self) -> u32 {
        match self {
            OBIVersion::One => 26,
        }
    }
}

#[non_exhaustive]
pub struct FileHeader {
    pub file_size: u32,
    pub data_offset: u32,
    pub version: u16,
}

impl FileHeader {
    pub fn new(version: OBIVersion, data_size: u32) -> Self {
        let header_size = version.header_size();
        Self {
            version: match version {
                OBIVersion::One => 1u16,
                _ => unreachable!("New version has been added!"),
            },
            file_size: header_size + data_size,
            data_offset: header_size,
        }
    }
}

#[non_exhaustive]
pub struct ImageInfoHeader {
    pub width: u32,
    pub height: u32,
    pub compression_type: u32,
    pub post_compression_size: u32,
}

impl ImageInfoHeader {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            compression_type: 0u32,
            post_compression_size: 0u32,
        }
    }
}

#[non_exhaustive]
pub enum CompressionType {
    RLE,
    Kosinki,
    None,
}

impl CompressionType {
    pub fn from_u32(kind: u32) -> Self {
        match kind {
            0 => CompressionType::None,
            1 => CompressionType::RLE,
            10 => CompressionType::Kosinki,
            _ => panic!("Invalid compression algorithm"),
        }
    }
}

pub struct Image {
    pub file_header: FileHeader,
    pub image_info_header: ImageInfoHeader,
    pub data: Vec<bool>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        // round to the nearest multiple of 8
        // convert to number of bytes by dividing by 8
        let mut data_size = width * height + 7;
        data_size = data_size - (data_size % 8);
        let data = vec![false; data_size as usize];

        Self {
            file_header: FileHeader::new(OBIVersion::One, data_size / 8),
            image_info_header: ImageInfoHeader::new(width, height),
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn size_of_image_info_header() {
        let file_header_size = size_of::<ImageInfoHeader>();
        assert_eq!(16, file_header_size);
    }

    #[test]
    fn encode_decode() {
        let img = Image::new(100, 80);
        let encoded = encode::encode_image(img).unwrap();
        assert_eq!(encoded.len(), 1026);
    }
}
