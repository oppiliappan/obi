use std::borrow::Borrow;

use bitvec::{prelude::*, vec::BitVec};
use byteorder::{LittleEndian, WriteBytesExt};

use crate::error::{OBIError, OBIResult};
use crate::Image;

pub fn encode_image<I>(obi_image: I) -> OBIResult<Vec<u8>>
where
    I: Borrow<Image>,
{
    let obi_image = obi_image.borrow();
    let mut obi_data = Vec::with_capacity(obi_image.file_header.file_size as usize);

    // file header
    let file_header = &obi_image.file_header;
    obi_data
        .write_u16::<LittleEndian>(file_header.version)
        .map_err(|_| OBIError::Encode)?;
    obi_data
        .write_u32::<LittleEndian>(file_header.file_size)
        .map_err(|_| OBIError::Encode)?;
    obi_data
        .write_u32::<LittleEndian>(file_header.data_offset)
        .map_err(|_| OBIError::Encode)?;

    // image info header
    let image_info_header = &obi_image.image_info_header;
    obi_data
        .write_u32::<LittleEndian>(image_info_header.width)
        .map_err(|_| OBIError::Encode)?;
    obi_data
        .write_u32::<LittleEndian>(image_info_header.height)
        .map_err(|_| OBIError::Encode)?;
    obi_data
        .write_u32::<LittleEndian>(image_info_header.compression_type)
        .map_err(|_| OBIError::Encode)?;
    obi_data
        .write_u32::<LittleEndian>(image_info_header.post_compression_size)
        .map_err(|_| OBIError::Encode)?;

    // pixmap data
    let pixmap = &obi_image.data;
    for byte in pixmap.chunks(8) {
        let mut bv = BitVec::<Lsb0, u8>::new();
        for &b in byte {
            bv.push(b);
        }
        obi_data
            .write_u8(bv.load::<u8>())
            .map_err(|_| OBIError::Encode)?;
    }

    return Ok(obi_data);
}
