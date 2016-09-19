use std::iter;
use std::iter::FromIterator;
use std::ops::BitXor;

use num::traits::*;
use itertools::Itertools;

const BASE64: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
    'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'
];


#[derive(Debug, Clone)]
pub struct ByteArray(pub Vec<u8>);

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

impl IntoIterator for ByteArray {
    type Item = u8;
    type IntoIter = ::std::vec::IntoIter<u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<u8> for ByteArray {
    fn from_iter<I: IntoIterator<Item=u8>>(iter: I) -> ByteArray {
        ByteArray(iter.into_iter().collect())
    }
}

impl ByteArray {
    pub fn new() -> ByteArray {
        ByteArray(vec![0])
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn from_hex(s: &str) -> ByteArray {
        s.chars().map(|c| c.to_digit(16).unwrap() )
            .chunks_lazy(2).into_iter()
            .map(|bytepair| bytepair.bitfold::<u8>(4))
            .collect()
    }

    pub fn to_hex(&self) -> String {
        self.0.clone().into_iter()
            .flat_map(|byte| {
                vec![4, 0].into_iter()
                    .map(move|shift| format!("{:x}", ((byte >> shift) & 0xf)))
            })
            .collect()
    }

    pub fn to_base64(&self) -> String {
        let padlen = (!self.0.len() % 3 + 3) % 3;

        self.0.clone().into_iter()
            .pad_using(self.0.len() + padlen, |_| 0 )
            .chunks_lazy(3).into_iter()
            .map(|triple| triple.bitfold::<u32>(8) )
            .flat_map(|triple| {
                vec![18u32, 12, 6, 0].into_iter()
                    .map(move|shift| BASE64[((triple >> shift) & 0x3f) as usize])
            })
            .collect_vec()
            .into_iter()
            .dropping_back(padlen)
            .chain(iter::repeat('=').take(padlen))
            .collect()
    }

    pub fn to_ascii(&self) -> String {
        self.0.clone().into_iter()
            .map(|byte| format!("{}", byte as char))
            .collect()
    }
}

impl BitXor for ByteArray {
    type Output = ByteArray;

    fn bitxor(self, rhs: ByteArray) -> ByteArray {
        self.0.into_iter().zip(rhs.0.into_iter())
            .map(|(lbyte, rbyte)| lbyte ^ rbyte)
            .collect()
    }
}

trait BitFold: Iterator {
    fn bitfold<T>(self, shift: usize) -> T
        where Self: Sized,
              Self::Item: ToPrimitive,
              T: Zero + PrimInt
    {
        self.fold(T::zero(), |buf, bits| { buf << shift | T::from(bits).unwrap() })
    }
}

// Iterator<Item=PrimInt>
impl<T: ?Sized> BitFold for T where T: Iterator {}

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

    #[test]
    fn test_bytearray_bitxor() {
        let a = ByteArray::from_hex("1c0111001f010100061a024b53535009181c");
        let b = ByteArray::from_hex("686974207468652062756c6c277320657965");
        assert_eq!((a ^ b).to_hex(), "746865206b696420646f6e277420706c6179")
    }
}
