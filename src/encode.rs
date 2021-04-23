use std::borrow::Borrow;

use bitvec::{prelude::*, vec::BitVec};
use byteorder::{LittleEndian, WriteBytesExt};

use crate::{
    error::{OBIError, OBIResult},
    rle::RLE,
    CompressionType, Image,
};

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

    let write_pixel_data = |pixels: &Vec<bool>, obi_data: &mut Vec<u8>| -> OBIResult<()> {
        for byte in pixels.chunks(8) {
            let mut bv = BitVec::<Lsb0, u8>::new();
            for &b in byte {
                bv.push(b)
            }
            obi_data
                .write_u8(bv.load::<u8>())
                .map_err(|_| OBIError::Encode)?;
        }
        Ok(())
    };

    // pixmap data
    let pixmap = &obi_image.data;
    match CompressionType::from_u32(obi_image.image_info_header.compression_type) {
        CompressionType::RLE => {
            let (data_points, lengths): (Vec<_>, Vec<_>) = pixmap.compress().into_iter().unzip();
            for l in lengths {
                obi_data
                    .write_u32::<LittleEndian>(l as u32)
                    .map_err(|_| OBIError::Encode)?;
            }
            // end length sequence with zero
            obi_data
                .write_u32::<LittleEndian>(0)
                .map_err(|_| OBIError::Encode)?;
            // begin data point sequence
            write_pixel_data(&data_points, &mut obi_data)?;
        }
        _ => write_pixel_data(pixmap, &mut obi_data)?,
    };

    Ok(obi_data)
}
