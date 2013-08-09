// AUTOGENERATED FROM index-windows-1252.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index-windows-1252.txt see the Encoding Standard
// http://encoding.spec.whatwg.org/

static FORWARD_TABLE: &'static [u16] = &[
    8364, 129, 8218, 402, 8222, 8230, 8224, 8225, 710, 8240, 352, 8249, 338,
    141, 381, 143, 144, 8216, 8217, 8220, 8221, 8226, 8211, 8212, 732, 8482,
    353, 8250, 339, 157, 382, 376, 160, 161, 162, 163, 164, 165, 166, 167, 168,
    169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183,
    184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198,
    199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213,
    214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228,
    229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243,
    244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255,
];

#[inline]
pub fn forward(code: u8) -> u16 {
    FORWARD_TABLE[code as uint]
}

#[inline]
pub fn backward(code: u16) -> u8 {
    match code {
        8364 => 0, 129 => 1, 8218 => 2, 402 => 3, 8222 => 4, 8230 => 5,
        8224 => 6, 8225 => 7, 710 => 8, 8240 => 9, 352 => 10, 8249 => 11,
        338 => 12, 141 => 13, 381 => 14, 143 => 15, 144 => 16, 8216 => 17,
        8217 => 18, 8220 => 19, 8221 => 20, 8226 => 21, 8211 => 22, 8212 => 23,
        732 => 24, 8482 => 25, 353 => 26, 8250 => 27, 339 => 28, 157 => 29,
        382 => 30, 376 => 31, 160 => 32, 161 => 33, 162 => 34, 163 => 35,
        164 => 36, 165 => 37, 166 => 38, 167 => 39, 168 => 40, 169 => 41,
        170 => 42, 171 => 43, 172 => 44, 173 => 45, 174 => 46, 175 => 47,
        176 => 48, 177 => 49, 178 => 50, 179 => 51, 180 => 52, 181 => 53,
        182 => 54, 183 => 55, 184 => 56, 185 => 57, 186 => 58, 187 => 59,
        188 => 60, 189 => 61, 190 => 62, 191 => 63, 192 => 64, 193 => 65,
        194 => 66, 195 => 67, 196 => 68, 197 => 69, 198 => 70, 199 => 71,
        200 => 72, 201 => 73, 202 => 74, 203 => 75, 204 => 76, 205 => 77,
        206 => 78, 207 => 79, 208 => 80, 209 => 81, 210 => 82, 211 => 83,
        212 => 84, 213 => 85, 214 => 86, 215 => 87, 216 => 88, 217 => 89,
        218 => 90, 219 => 91, 220 => 92, 221 => 93, 222 => 94, 223 => 95,
        224 => 96, 225 => 97, 226 => 98, 227 => 99, 228 => 100, 229 => 101,
        230 => 102, 231 => 103, 232 => 104, 233 => 105, 234 => 106, 235 => 107,
        236 => 108, 237 => 109, 238 => 110, 239 => 111, 240 => 112, 241 => 113,
        242 => 114, 243 => 115, 244 => 116, 245 => 117, 246 => 118, 247 => 119,
        248 => 120, 249 => 121, 250 => 122, 251 => 123, 252 => 124, 253 => 125,
        254 => 126, 255 => 127, _ => 255
    }
}

#[cfg(test)]
mod tests {
    use std::u8;
    use super::{forward, backward};

    #[test]
    fn test_correct_table() {
        for u8::range(0, 128) |i| {
            let j = forward(i);
            if j != 0xffff { assert_eq!(backward(j), i); }
        }
    }
}
