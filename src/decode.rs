use std::io::{self, Cursor, Read};

use bitvec::prelude::*;
use byteorder::{LittleEndian, ReadBytesExt};

use crate::{FileHeader, Image, ImageInfoHeader};

pub fn decode_image(obi_data: &mut Cursor<Vec<u8>>) -> io::Result<Image> {
    // file header
    let version = obi_data.read_u16::<LittleEndian>()?;
    let file_size = obi_data.read_u32::<LittleEndian>()?;
    let data_offset = obi_data.read_u32::<LittleEndian>()?;
    let file_header = FileHeader {
        version,
        file_size,
        data_offset,
    };

    // image info header
    let width = obi_data.read_u32::<LittleEndian>()?;
    let height = obi_data.read_u32::<LittleEndian>()?;
    let compression_type = obi_data.read_u32::<LittleEndian>()?;
    let post_compression_size = obi_data.read_u32::<LittleEndian>()?;
    let image_info_header = ImageInfoHeader {
        width,
        height,
        compression_type,
        post_compression_size,
    };

    // pixmap data
    let mut data_bytes = vec![];
    obi_data.read_to_end(&mut data_bytes)?;
    let data = data_bytes
        .iter()
        .map(|&b| {
            BitVec::<Lsb0, u8>::from_element(b)
                .into_iter()
                .map(|e| e as bool)
                .collect::<Vec<bool>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    return Ok(Image {
        file_header,
        image_info_header,
        data,
    });
}
