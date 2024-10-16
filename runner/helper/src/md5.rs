const S: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9,
    14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15,
    21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

const K: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

const A: u32 = 0x67452301;
const B: u32 = 0xefcdab89;
const C: u32 = 0x98badcfe;
const D: u32 = 0x10325476;

pub struct MD5 {}

struct Payload<'a> {
    payload: Option<&'a [u8]>,
    bytes: [u8; 64],
    chunks: [u32; 16],
    len: [u8; 8],
}

impl<'a> Payload<'a> {
    fn new(payload: &'a [u8]) -> Self {
        let len = payload.len() as u64 * 8;
        Self {
            payload: Some(payload),
            bytes: [0; 64],
            chunks: [0; 16],
            len: len.to_le_bytes(),
        }
    }

    fn fill(&mut self) -> bool {
        if let Some(bytes) = self.payload.as_mut() {
            if bytes.len() >= 64 {
                self.bytes.copy_from_slice(&bytes[0..64]);
                *bytes = &bytes[64..];
            } else {
                self.bytes.as_mut_slice()[..bytes.len()].copy_from_slice(bytes);
                self.bytes[bytes.len()] = 0x80;
                let last = bytes.len() + 1;
                if last <= 56 {
                    self.bytes.as_mut_slice()[last..56]
                        .iter_mut()
                        .for_each(|b| *b = 0);
                    self.bytes.as_mut_slice()[56..].copy_from_slice(&self.len);
                    self.payload = None;
                } else {
                    self.bytes.as_mut_slice()[last..]
                        .iter_mut()
                        .for_each(|b| *b = 0);
                    *bytes = &bytes[bytes.len()..];
                }
            }

            for i in 0..16 {
                self.chunks[i] =
                    u32::from_le_bytes(self.bytes[i * 4..(i + 1) * 4].try_into().unwrap());
            }

            true
        } else {
            false
        }
    }
}

impl MD5 {
    pub fn digest(bytes: &[u8]) -> [u8; 16] {
        let mut a0 = A;
        let mut b0 = B;
        let mut c0 = C;
        let mut d0 = D;
        let mut payload = Payload::new(bytes);
        while payload.fill() {
            let mut a = a0;
            let mut b = b0;
            let mut c = c0;
            let mut d = d0;

            macro_rules! tail {
                ($f:ident, $g:ident, $i:ident) => {{
                    let f = $f
                        .wrapping_add(a)
                        .wrapping_add(K[$i])
                        .wrapping_add(payload.chunks[$g]);
                    a = d;
                    d = c;
                    c = b;
                    b = b.wrapping_add(f.rotate_left(S[$i]));
                }};
            }

            for i in 0..16 {
                let f = (b & c) | (!b & d);
                let g = i;
                tail!(f, g, i);
            }
            for i in 16..32 {
                let f = (d & b) | (!d & c);
                let g = (5 * i + 1) % 16;
                tail!(f, g, i);
            }
            for i in 32..48 {
                let f = b ^ c ^ d;
                let g = (3 * i + 5) % 16;
                tail!(f, g, i);
            }
            for i in 48..64 {
                let f = c ^ (b | !d);
                let g = (7 * i) % 16;
                tail!(f, g, i);
            }

            a0 = a0.wrapping_add(a);
            b0 = b0.wrapping_add(b);
            c0 = c0.wrapping_add(c);
            d0 = d0.wrapping_add(d);
        }

        let mut res = [0; 16];
        res[0..4].copy_from_slice(&a0.to_le_bytes());
        res[4..8].copy_from_slice(&b0.to_le_bytes());
        res[8..12].copy_from_slice(&c0.to_le_bytes());
        res[12..16].copy_from_slice(&d0.to_le_bytes());
        res
    }
}

#[cfg(test)]
mod test {
    use crate::md5::MD5;

    #[test]
    fn md5() {
        let tests = [
            ("1", "c4ca4238a0b923820dcc509a6f75849b"),
            (
                "The quick brown fox jumps over the lazy dog",
                "9e107d9d372bb6826bd81d3542a419d6",
            ),
            (
                "The quick brown fox jumps over the lazy dog.",
                "e4d909c290d0fb1ca068ffaddf22cbd0",
            ),
            ("", "d41d8cd98f00b204e9800998ecf8427e"),
        ];
        for (s, e) in tests.iter() {
            use std::fmt::Write;
            let digest = MD5::digest(s.as_bytes());
            let digest_str: String = digest.iter().fold(String::new(), |mut s, b| {
                let _ = write!(s, "{b:02x}");
                s
            });
            assert_eq!(digest_str, *e, "{s:?} did not producted expected MD5");
        }
    }
}
