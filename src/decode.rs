use std::io::{Cursor, Read};

use bitvec::prelude::*;
use byteorder::{LittleEndian, ReadBytesExt};

use crate::error::{OBIError, OBIResult};
use crate::{FileHeader, Image, ImageInfoHeader};

pub fn decode_image(obi_data: &mut Cursor<Vec<u8>>) -> OBIResult<Image> {
    // file header
    let version = obi_data
        .read_u16::<LittleEndian>()
        .map_err(|_| OBIError::Decode)?;
    let file_size = obi_data
        .read_u32::<LittleEndian>()
        .map_err(|_| OBIError::Decode)?;
    let data_offset = obi_data
        .read_u32::<LittleEndian>()
        .map_err(|_| OBIError::Decode)?;
    let file_header = FileHeader {
        version,
        file_size,
        data_offset,
    };

    // image info header
    let width = obi_data
        .read_u32::<LittleEndian>()
        .map_err(|_| OBIError::Decode)?;
    let height = obi_data
        .read_u32::<LittleEndian>()
        .map_err(|_| OBIError::Decode)?;
    let compression_type = obi_data
        .read_u32::<LittleEndian>()
        .map_err(|_| OBIError::Decode)?;
    let post_compression_size = obi_data
        .read_u32::<LittleEndian>()
        .map_err(|_| OBIError::Decode)?;
    let image_info_header = ImageInfoHeader {
        width,
        height,
        compression_type,
        post_compression_size,
    };

    // pixmap data
    let mut data_bytes = vec![];
    obi_data
        .read_to_end(&mut data_bytes)
        .map_err(|_| OBIError::Decode)?;
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
