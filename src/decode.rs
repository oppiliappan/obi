use std::io::{Cursor, Read};

use bitvec::prelude::*;
use byteorder::{LittleEndian, ReadBytesExt};

use crate::error::{OBIError, OBIResult};
use crate::{CompressionType, FileHeader, Image, ImageInfoHeader};

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
        file_size,
        version,
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

    let data: Vec<bool> = match CompressionType::from_u32(compression_type) {
        CompressionType::RLE => {
            let mut rest = vec![];
            let mut lengths = vec![];
            loop {
                let l = obi_data
                    .read_u32::<LittleEndian>()
                    .map_err(|_| OBIError::Encode)?;
                if l == 0 {
                    break;
                }
                lengths.push(l);
            }
            obi_data
                .read_to_end(&mut rest)
                .map_err(|_| OBIError::Decode)?;
            rest.iter()
                .map(|&b| {
                    BitVec::<Lsb0, u8>::from_element(b)
                        .into_iter()
                        .map(|e| e as bool)
                        .collect::<Vec<bool>>()
                })
                .flatten()
                .into_iter()
                .zip(lengths)
                .map(|(d, l)| vec![d; l as usize])
                .flatten()
                .collect::<Vec<bool>>()
        }
        _ => {
            let mut rest = vec![];
            obi_data
                .read_to_end(&mut rest)
                .map_err(|_| OBIError::Decode)?;
            rest.iter()
                .map(|&b| {
                    BitVec::<Lsb0, u8>::from_element(b)
                        .into_iter()
                        .map(|e| e as bool)
                        .collect::<Vec<bool>>()
                })
                .flatten()
                .collect::<Vec<_>>()
        }
    };
    Ok(Image {
        file_header,
        image_info_header,
        data,
    })
}
