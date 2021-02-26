#![allow(unreachable_patterns)]
#![allow(non_snake_case)]
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Pixel {
    On,
    Off,
}

#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OBIVersion {
    One,
}

impl OBIVersion {
    pub fn header_size(&self) -> u32 {
        match self {
            One => 26,
        }
    }
}

#[non_exhaustive]
pub struct FileHeader {
    pub version: u16,
    pub file_size: u32,
    pub data_offset: u32,
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
    pub data: Vec<Pixel>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        let data_size = width * height;
        let data = vec![Pixel::Off; data_size as usize];
        Self {
            file_header: FileHeader::new(OBIVersion::One, data_size),
            image_info_header: ImageInfoHeader::new(width, height),
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
