#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

fn encode(value: u32) -> Vec<u8> {
    let mut bytes = Vec::new();
    for i in (0..5).rev() {
        let mut buffer: u8 = 0;
        buffer |= ((value >> (i * 7)) & 0b01111111) as u8;
        // Skip leading zeros
        if i > 0 && buffer == 0 && bytes.is_empty() {
            continue;
        }
        // Set the continuation bit if there are more bytes to come
        if i != 0 {
            buffer |= 0b10000000;
        }
        bytes.push(buffer);
    }
    bytes
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(|&value| encode(value)).collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut decoded_numbers: Vec<u32> = Vec::new();
    let mut value: u32 = 0;
    let mut is_complete = true;
    for &byte in bytes {
        value = (value << 7) | ((byte & 0b01111111) as u32);
        is_complete = false;
        if (byte & 0b10000000) == 0 {
            decoded_numbers.push(value);
            value = 0;
            is_complete = true;
        }
    }
    if is_complete {
        Ok(decoded_numbers)
    } else {
        Err(Error::IncompleteNumber)
    }
}
