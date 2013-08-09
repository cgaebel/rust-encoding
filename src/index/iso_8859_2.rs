// AUTOGENERATED FROM index-iso-8859-2.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index-iso-8859-2.txt see the Encoding Standard
// http://encoding.spec.whatwg.org/

static FORWARD_TABLE: &'static [u16] = &[
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142,
    143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157,
    158, 159, 160, 260, 728, 321, 164, 317, 346, 167, 168, 352, 350, 356, 377,
    173, 381, 379, 176, 261, 731, 322, 180, 318, 347, 711, 184, 353, 351, 357,
    378, 733, 382, 380, 340, 193, 194, 258, 196, 313, 262, 199, 268, 201, 280,
    203, 282, 205, 206, 270, 272, 323, 327, 211, 212, 336, 214, 215, 344, 366,
    218, 368, 220, 221, 354, 223, 341, 225, 226, 259, 228, 314, 263, 231, 269,
    233, 281, 235, 283, 237, 238, 271, 273, 324, 328, 243, 244, 337, 246, 247,
    345, 367, 250, 369, 252, 253, 355, 729,
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
        159 => 31, 160 => 32, 260 => 33, 728 => 34, 321 => 35, 164 => 36,
        317 => 37, 346 => 38, 167 => 39, 168 => 40, 352 => 41, 350 => 42,
        356 => 43, 377 => 44, 173 => 45, 381 => 46, 379 => 47, 176 => 48,
        261 => 49, 731 => 50, 322 => 51, 180 => 52, 318 => 53, 347 => 54,
        711 => 55, 184 => 56, 353 => 57, 351 => 58, 357 => 59, 378 => 60,
        733 => 61, 382 => 62, 380 => 63, 340 => 64, 193 => 65, 194 => 66,
        258 => 67, 196 => 68, 313 => 69, 262 => 70, 199 => 71, 268 => 72,
        201 => 73, 280 => 74, 203 => 75, 282 => 76, 205 => 77, 206 => 78,
        270 => 79, 272 => 80, 323 => 81, 327 => 82, 211 => 83, 212 => 84,
        336 => 85, 214 => 86, 215 => 87, 344 => 88, 366 => 89, 218 => 90,
        368 => 91, 220 => 92, 221 => 93, 354 => 94, 223 => 95, 341 => 96,
        225 => 97, 226 => 98, 259 => 99, 228 => 100, 314 => 101, 263 => 102,
        231 => 103, 269 => 104, 233 => 105, 281 => 106, 235 => 107, 283 => 108,
        237 => 109, 238 => 110, 271 => 111, 273 => 112, 324 => 113, 328 => 114,
        243 => 115, 244 => 116, 337 => 117, 246 => 118, 247 => 119, 345 => 120,
        367 => 121, 250 => 122, 369 => 123, 252 => 124, 253 => 125, 355 => 126,
        729 => 127, _ => 255
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
