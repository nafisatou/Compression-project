pub fn compress(data: &[u8]) -> Vec<u8> {
    let mut out = vec![];
    let mut i = 0;

    while i < data.len() {
        let byte = data[i];
        let mut count = 1;


        while i + count < data.len() && data[i + count] == byte && count < 255 {
            count += 1;
        }

        out.push(byte);
        out.push(count.try_into().unwrap());
        i += count;
    }

    out
}


pub fn decompress(data: &[u8]) -> Vec<u8> {
    let mut out = vec![];

    for chunk in data.chunks(2) {
        if chunk.len() == 2 {
            let byte = chunk[0];
            let count = chunk[1];
            out.extend(std::iter::repeat(byte).take(count as usize));
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rle_roundtrip() {
        let input = b"AAABBBCCCCCDDDDE";
        let compressed = compress(input);
        let decompressed = decompress(&compressed);
        assert_eq!(input.to_vec(), decompressed);
    }
}
