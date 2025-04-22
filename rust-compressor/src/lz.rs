pub fn compress(data: &[u8]) -> Vec<u8> {
    let mut out = vec![];
    let window = 20;
    let mut i = 0;

    while i < data.len() {
        let mut match_offset = 0;
        let mut match_len = 0;

        for j in (i.saturating_sub(window))..i {
            let mut k = 0;
            while i + k < data.len() && data[j + k] == data[i + k] && k < 255 {
                k += 1;
            }

            if k > match_len {
                match_len = k;
                match_offset = i - j;
            }
        }

        if match_len >= 3 {
            out.push(0x01);
            out.push(match_offset as u8);
            out.push(match_len as u8);
            i += match_len;
        } else {
            out.push(0x00);
            out.push(data[i]);
            i += 1;
        }
    }

    out
}

pub fn decompress(data: &[u8]) -> Vec<u8> {
    let mut out = vec![];
    let mut i = 0;

    while i < data.len() {
        match data[i] {
            0x00 => {
                i += 1;
                out.push(data[i]);
                i += 1;
            },
            0x01 => {
                let offset = data[i + 1] as usize;
                let length = data[i + 2] as usize;
                let start = out.len() - offset;
                for j in 0..length {
                    out.push(out[start + j]);
                }
                i += 3;
            },
            _ => break,
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lz_roundtrip() {
        let input = b"ABABABABABAB";
        let compressed = compress(input);
        let decompressed = decompress(&compressed);
        assert_eq!(input.to_vec(), decompressed);
    }
}
