// AUTOGENERATED FROM index-iso-8859-3.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index-iso-8859-3.txt see the Encoding Standard
// http://encoding.spec.whatwg.org/

static FORWARD_TABLE: &'static [u16] = &[
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142,
    143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157,
    158, 159, 160, 294, 728, 163, 164, 65535, 292, 167, 168, 304, 350, 286,
    308, 173, 65535, 379, 176, 295, 178, 179, 180, 181, 293, 183, 184, 305,
    351, 287, 309, 189, 65535, 380, 192, 193, 194, 65535, 196, 266, 264, 199,
    200, 201, 202, 203, 204, 205, 206, 207, 65535, 209, 210, 211, 212, 288,
    214, 215, 284, 217, 218, 219, 220, 364, 348, 223, 224, 225, 226, 65535,
    228, 267, 265, 231, 232, 233, 234, 235, 236, 237, 238, 239, 65535, 241,
    242, 243, 244, 289, 246, 247, 285, 249, 250, 251, 252, 365, 349, 729,
];

#[inline]
pub fn forward(code: u8) -> u16 {
    FORWARD_TABLE[code as uint]
}

#[inline]
pub fn backward(code: u16) -> u8 {
    match code {
        128 => 0, 129 => 1, 130 => 2, 131 => 3, 132 => 4, 133 => 5, 134 => 6,
        135 => 7, 136 => 8, 137 => 9, 138 => 10, 139 => 11, 140 => 12,
        141 => 13, 142 => 14, 143 => 15, 144 => 16, 145 => 17, 146 => 18,
        147 => 19, 148 => 20, 149 => 21, 150 => 22, 151 => 23, 152 => 24,
        153 => 25, 154 => 26, 155 => 27, 156 => 28, 157 => 29, 158 => 30,
        159 => 31, 160 => 32, 294 => 33, 728 => 34, 163 => 35, 164 => 36,
        292 => 38, 167 => 39, 168 => 40, 304 => 41, 350 => 42, 286 => 43,
        308 => 44, 173 => 45, 379 => 47, 176 => 48, 295 => 49, 178 => 50,
        179 => 51, 180 => 52, 181 => 53, 293 => 54, 183 => 55, 184 => 56,
        305 => 57, 351 => 58, 287 => 59, 309 => 60, 189 => 61, 380 => 63,
        192 => 64, 193 => 65, 194 => 66, 196 => 68, 266 => 69, 264 => 70,
        199 => 71, 200 => 72, 201 => 73, 202 => 74, 203 => 75, 204 => 76,
        205 => 77, 206 => 78, 207 => 79, 209 => 81, 210 => 82, 211 => 83,
        212 => 84, 288 => 85, 214 => 86, 215 => 87, 284 => 88, 217 => 89,
        218 => 90, 219 => 91, 220 => 92, 364 => 93, 348 => 94, 223 => 95,
        224 => 96, 225 => 97, 226 => 98, 228 => 100, 267 => 101, 265 => 102,
        231 => 103, 232 => 104, 233 => 105, 234 => 106, 235 => 107, 236 => 108,
        237 => 109, 238 => 110, 239 => 111, 241 => 113, 242 => 114, 243 => 115,
        244 => 116, 289 => 117, 246 => 118, 247 => 119, 285 => 120, 249 => 121,
        250 => 122, 251 => 123, 252 => 124, 365 => 125, 349 => 126, 729 => 127,
        _ => 255
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
