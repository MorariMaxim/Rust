


fn ceil_division(dividend: u32, divisor: u32) -> u32 {
    (dividend + divisor - 1) / divisor
}
fn chunk_to_u32(chunk: &[u8]) -> u32 {
    if chunk.len() > 4 {
        panic!("chunk size > 4")
    };
    let mut val = 0u32;

    for b in chunk.iter() {
        val <<= 8;
        val += *b as u32;
    }
    val <<= 8 * (4 - chunk.len());
    val
}

/// Encoding a slice of bytes to a base 64 string
///
/// # Example
/// ```
/// use base64::encode;
/// let bytes = b"light work.";
/// let string = encode(bytes);
/// assert_eq!(string, "bGlnaHQgd29yay4=");
/// ```
///
pub fn encode(input: &[u8]) -> String {
    let mut convert_table = Vec::<char>::with_capacity(64);
    for i in 0..=25u8 {
        convert_table.push((65 + i) as char);
    }
    for i in 0..=25u8 {
        convert_table.push((97 + i) as char);
    }
    for i in 0..=9u8 {
        convert_table.push((48 + i) as char);
    }

    convert_table.push('+');
    convert_table.push('/');

    let mut output_string =
        String::with_capacity((ceil_division(input.len() as u32, 3) * 4) as usize);

    for chunk in input.chunks_exact(3) {
        let mut temp = chunk_to_u32(chunk);
        let mut x: usize;
        for _i in 0..4 {
            x = ((temp & 0b111_111_0000_0000_0000_0000_0000_0000_00) >> 26) as usize;
            //println!("{x}");
            output_string.push(convert_table[x] as char);
            temp <<= 6;
        }
    }
    let len = input.len();
    let next = (input.len() / 3) * 3;
    let diff = len - next;
    if diff == 1 {
        let mut temp = chunk_to_u32(&input[next..next + 1]);
        let mut x: usize;
        for _i in 0..2 {
            x = ((temp & 0b111_111_0000_0000_0000_0000_0000_0000_00) >> 26) as usize;
            //println!("{x}");
            output_string.push(convert_table[x] as char);
            temp <<= 6;
        }
        output_string.push_str("==");
    } else if diff == 2 {
        let mut temp = chunk_to_u32(&input[next..next + 2]);
        let mut x: usize;
        for _i in 0..3 {
            x = ((temp & 0b111_111_0000_0000_0000_0000_0000_0000_00) >> 26) as usize;
            //println!("{x}");
            output_string.push(convert_table[x] as char);
            temp <<= 6;
        }
        output_string.push_str("=");
    }
    output_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_no_padding() {
        let bytes = b"light wor";
        let string = encode(bytes);
        assert_eq!(string, "bGlnaHQgd29y");
    }

    #[test]
    fn test_encode_padding_1padding() {
        let bytes = b"light wo";
        let string = encode(bytes);
        assert_eq!(string, "bGlnaHQgd28=");
    }

    #[test]
    fn test_encode_padding_2padding() {
        let bytes = b"light w";
        let string = encode(bytes);
        assert_eq!(string, "bGlnaHQgdw==");
    }
}
