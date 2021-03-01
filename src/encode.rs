use std::io;

use bitvec::{prelude::*, vec::BitVec};
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
        let mut bv = BitVec::<Lsb0, u8>::new();
        for &b in byte {
            bv.push(b);
        }
        obi_data.write_u8(bv.load::<u8>())?;
    }

    return Ok(obi_data);
}
