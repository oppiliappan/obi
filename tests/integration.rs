use std::io::Cursor;

use obi::{error::OBIResult, Image};

#[test]
fn encode_modify_decode() {
    let mut img = Image::new(50, 5);
    img.set(2, 3, true).expect("Indexing error");
    let encoded = img.encode().unwrap();
    let mut cursor = Cursor::new(encoded);
    let decoded = Image::decode(&mut cursor).unwrap();
    assert!(decoded.get(2, 3).unwrap());
}

#[test]
fn encode_modify_decode_padded_byte_edge_case() -> OBIResult<()> {
    let mut img = Image::new(1, 4);
    img.set(0, 2, true)?;
    let encoded = img.encode()?;
    let mut cursor = Cursor::new(encoded);
    let decoded = Image::decode(&mut cursor)?;
    assert!(!decoded.get(0, 0)?);
    assert!(!decoded.get(0, 1)?);
    assert!(decoded.get(0, 2)?);
    assert!(!decoded.get(0, 3)?);
    Ok(())
}

#[test]
fn encode_modify_decode_padded_byte_edge_case_big_image() -> OBIResult<()> {
    let mut img = Image::new(25, 10);
    img.set(24, 9, true)?;
    let encoded = img.encode()?;
    let mut cursor = Cursor::new(encoded);
    let decoded = Image::decode(&mut cursor)?;
    assert!(decoded.get(24, 9)?);
    Ok(())
}
