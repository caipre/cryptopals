use itertools::Itertools;

const BASE64: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
    'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'
];


#[derive(Debug)]
struct ByteArray(Vec<u8>);

impl PartialEq for ByteArray {
    fn eq(&self, other: &ByteArray) -> bool {
        if self.0.len() != other.0.len() {
            return false
        }
        for i in 0..self.0.len() {
            if self.0[i] != other.0[i] {
                return false
            }
        }
        true
    }
}

impl ByteArray {
    fn from_hex(s: &str) -> ByteArray {
        ByteArray(
            s.bytes()
             .chunks_lazy(2).into_iter()
                .map(|chunk| {
                    chunk
                        .map(|byte| {
                            match byte as char {
                                '0'...'9' => byte - '0' as u8,
                                'A'...'F' => byte - 'A' as u8 + 10,
                                'a'...'f' => byte - 'a' as u8 + 10,
                                _         => unreachable!(),
                            }})
                        .enumerate()
                        .fold(0, |buf, (idx, elem)| { buf << (idx * 4) | elem })
                })
                .collect()
        )
    }

    fn to_base64(&self) -> String {
        let padding = match self.0.len() % 3 {
            0 => vec![],
            1 => vec![0, 0],
            2 => vec![0],
            _ => unreachable!(),
        };

        self.0.iter()
            .chain(padding.iter())
            .chunks_lazy(3).into_iter()
            .map(|chunk| { chunk.fold(0u32, |buf, &elem| { buf << 8 | elem as u32 }) })
            .fold(Vec::new(), |mut vec, chunk| {
                vec.extend(
                    [18u32, 12, 6, 0]
                        .iter()
                        .map(|shift| { BASE64[((chunk >> shift) & 0x3f) as usize] })
                );
                vec
            })
            .into_iter()
            .rev().skip(padding.len()).rev()
            .into_iter()
            .chain(padding.iter().map(|_| '='))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::ByteArray;

    #[test]
    fn test_bytearray_from_hex() {
        assert_eq!(ByteArray::from_hex("49276d206b69"),
                   ByteArray(vec![0x49, 0x27, 0x6d, 0x20, 0x6b, 0x69]));
    }

    #[test]
    fn test_bytearray_to_base64() {
        assert_eq!(ByteArray(vec![0x49, 0x27, 0x6d, 0x20, 0x6b, 0x69]).to_base64(),
                   "SSdtIGtp");
    }

    #[test]
    fn test_base64_padding() {
        assert_eq!(ByteArray::from_hex("a").to_base64(), "Cg==");
        assert_eq!(ByteArray::from_hex("a0").to_base64(), "oA==");
        assert_eq!(ByteArray::from_hex("a0b").to_base64(), "oAs=");
    }

    #[test]
    fn test_hex_to_base64() {
        let string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        assert_eq!(ByteArray::from_hex(string).to_base64(), "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }
}
