use std::io::{Read, Result};

/// Read u8 values from a series of bytes.
pub fn read_u8<R: Read>(reader: &mut R) -> Result<u8> {
    let mut buf = [0; 1];
    reader.read_exact(&mut buf)?;

    Ok(buf[0])
}

/// Read u16 values from a series of bytes.
///
/// All the numeric values in the replays are little endian, so that is what we use here.
pub fn read_u16<R: Read>(reader: &mut R) -> Result<u16> {
    let mut buf = [0; 2];
    reader.read_exact(&mut buf)?;

    let val = u16::from_le_bytes(buf);

    Ok(val)
}

/// Read u32 values from a series of bytes.
///
/// All the numeric values in the replays are little endian, so that is what we use here.
pub fn read_u32<R: Read>(reader: &mut R) -> Result<u32> {
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;

    let val = u32::from_le_bytes(buf);

    Ok(val)
}

/// Read u128 values from a series of bytes.
///
/// All the numeric values in the replays are little endian, so that is what we use here.
pub fn read_u128<R: Read>(reader: &mut R) -> Result<u128> {
    let mut buf = [0; 16];
    reader.read_exact(&mut buf)?;

    let val = u128::from_le_bytes(buf);

    Ok(val)
}

/// Read f32 values from a series of bytes.
///
/// All the numeric values in the replays are little endian, so that is what we use here.
pub fn read_f32<R: Read>(reader: &mut R) -> Result<f32> {
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;

    let val = f32::from_bits(u32::from_le_bytes(buf));

    Ok(val)
}

/// Performs a case-insensitive string comparison.
pub fn string_eq_ignore_case(a: &str, b: &str) -> bool {
    a == b || a.to_lowercase() == b.to_lowercase()
}
