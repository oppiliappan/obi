use std::io;

use bitvec::prelude::*;
use bitvec::slice::BitSlice;
use byteorder::{LittleEndian, WriteBytesExt};

use crate::Image;

pub fn encode_image(obi_image: Image) -> io::Result<Vec<u8>> {
    let mut obi_data = Vec::with_capacity(obi_image.file_header.file_size as usize);

    // file header
    let file_header = obi_image.file_header;
    obi_data.write_u16::<LittleEndian>(file_header.version)?;
    obi_data.write_u32::<LittleEndian>(file_header.file_size)?;
    obi_data.write_u32::<LittleEndian>(file_header.data_offset)?;

    // image info header
    let image_info_header = obi_image.image_info_header;
    obi_data.write_u32::<LittleEndian>(image_info_header.width)?;
    obi_data.write_u32::<LittleEndian>(image_info_header.height)?;
    obi_data.write_u32::<LittleEndian>(image_info_header.compression_type)?;
    obi_data.write_u32::<LittleEndian>(image_info_header.post_compression_size)?;

    // pixmap data
    let pixmap = obi_image.data;
    for byte in pixmap.chunks(8) {
        let bits_as_u8 = byte.iter().map(|&e| e as u8).collect::<Vec<_>>();
        let slice = BitSlice::<Lsb0, _>::from_slice(&bits_as_u8).unwrap();
        obi_data.write_u8(slice.load::<u8>())?;
    }

    return Ok(obi_data);
}
