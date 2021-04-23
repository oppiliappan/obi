use crate::{
    error::{OBIError, OBIResult},
    CompressionType, Image,
};

use std::io::Write;

use png::{BitDepth, ColorType, Encoder, Writer};

pub fn to_png<W: Write>(writer: W, img: &Image) -> Writer<W> {
    let mut encoder = Encoder::new(writer, img.width(), img.height());
    encoder.set_color(ColorType::RGBA);
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer
        .write_image_data(
            &img.data
                .iter()
                .take((img.width() * img.height()) as usize)
                .map(|x| if *x { vec![255; 4] } else { vec![0, 0, 0, 255] })
                .flatten()
                .collect::<Vec<_>>()[..],
        )
        .unwrap();
    writer
}
