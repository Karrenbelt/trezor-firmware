//! This file is generated by core/tools/codegen/gen_font.py
#![cfg_attr(any(), rustfmt::skip)]
#![allow(non_upper_case_globals)]
// Each glyph:
//   - first two bytes: width, height
//   - next three bytes: advance, bearingX, bearingY
//   - rest is packed 4-bit glyph data

use crate::ui::display::font::FontInfo;

// NOTE: glyphs of uppercase and special characters removed to save space as they are not used

/// ' ' (ASCII 32)
const Font_TTSatoshi_DemiBold_42_glyph_32: [u8; 5] = [ 0, 0, 11, 0, 0 ];

/// 'a' (ASCII 97)
const Font_TTSatoshi_DemiBold_42_glyph_97: [u8; 235] = [ 20, 23, 23, 1, 22, 0, 0, 0, 147, 237, 239, 157, 4, 0, 0, 0, 0, 162, 255, 255, 255, 255, 223, 3, 0, 0, 64, 255, 255, 255, 255, 255, 255, 79, 0, 0, 225, 255, 255, 255, 255, 255, 255, 255, 2, 0, 251, 255, 255, 107, 116, 252, 255, 255, 11, 32, 255, 255, 127, 0, 0, 160, 255, 255, 31, 96, 255, 255, 12, 0, 0, 16, 254, 255, 95, 0, 0, 0, 0, 0, 0, 0, 251, 255, 127, 0, 0, 0, 0, 0, 0, 0, 249, 255, 159, 0, 0, 113, 219, 255, 255, 255, 255, 255, 159, 0, 128, 255, 255, 255, 255, 255, 255, 255, 159, 0, 250, 255, 255, 255, 255, 255, 255, 255, 159, 96, 255, 255, 255, 206, 187, 187, 253, 255, 159, 208, 255, 255, 61, 0, 0, 0, 249, 255, 159, 242, 255, 255, 3, 0, 0, 0, 251, 255, 159, 243, 255, 255, 0, 0, 0, 0, 254, 255, 159, 243, 255, 255, 0, 0, 0, 96, 255, 255, 159, 241, 255, 255, 6, 0, 0, 228, 255, 255, 159, 208, 255, 255, 143, 52, 165, 255, 255, 255, 159, 64, 255, 255, 255, 255, 255, 255, 255, 255, 159, 0, 249, 255, 255, 255, 255, 255, 251, 255, 159, 0, 112, 255, 255, 255, 255, 111, 240, 255, 159, 0, 0, 145, 237, 239, 107, 1, 0, 0, 0 ];

/// 'b' (ASCII 98)
const Font_TTSatoshi_DemiBold_42_glyph_98: [u8; 365] = [ 23, 30, 26, 2, 29, 241, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 241, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 241, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 241, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 241, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 241, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 241, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 241, 255, 255, 0, 114, 236, 223, 107, 1, 0, 0, 0, 241, 255, 255, 112, 255, 255, 255, 255, 110, 0, 0, 0, 241, 255, 255, 250, 255, 255, 255, 255, 255, 9, 0, 0, 241, 255, 255, 255, 255, 255, 255, 255, 255, 143, 0, 0, 241, 255, 255, 255, 255, 207, 255, 255, 255, 255, 4, 0, 241, 255, 255, 255, 61, 0, 48, 253, 255, 255, 12, 0, 241, 255, 255, 191, 0, 0, 0, 176, 255, 255, 79, 0, 241, 255, 255, 47, 0, 0, 0, 16, 255, 255, 159, 0, 241, 255, 255, 9, 0, 0, 0, 0, 248, 255, 223, 0, 241, 255, 255, 5, 0, 0, 0, 0, 244, 255, 255, 0, 241, 255, 255, 2, 0, 0, 0, 0, 242, 255, 255, 0, 241, 255, 255, 0, 0, 0, 0, 0, 240, 255, 255, 1, 241, 255, 255, 2, 0, 0, 0, 0, 242, 255, 255, 0, 241, 255, 255, 5, 0, 0, 0, 0, 244, 255, 255, 0, 241, 255, 255, 9, 0, 0, 0, 0, 248, 255, 223, 0, 241, 255, 255, 47, 0, 0, 0, 16, 255, 255, 159, 0, 241, 255, 255, 191, 0, 0, 0, 176, 255, 255, 79, 0, 241, 255, 255, 255, 61, 0, 48, 253, 255, 255, 12, 0, 241, 255, 255, 255, 255, 207, 255, 255, 255, 255, 4, 0, 241, 255, 255, 255, 255, 255, 255, 255, 255, 143, 0, 0, 241, 255, 175, 251, 255, 255, 255, 255, 255, 9, 0, 0, 241, 255, 111, 144, 255, 255, 255, 255, 110, 0, 0, 0, 0, 0, 0, 0, 130, 237, 239, 107, 1, 0, 0, 0 ];

/// 'c' (ASCII 99)
const Font_TTSatoshi_DemiBold_42_glyph_99: [u8; 258] = [ 22, 23, 24, 1, 22, 0, 0, 0, 64, 217, 254, 206, 56, 0, 0, 0, 0, 0, 64, 253, 255, 255, 255, 255, 27, 0, 0, 0, 0, 248, 255, 255, 255, 255, 255, 239, 3, 0, 0, 144, 255, 255, 255, 255, 255, 255, 255, 30, 0, 0, 246, 255, 255, 255, 206, 254, 255, 255, 207, 0, 0, 254, 255, 255, 43, 0, 48, 252, 255, 255, 6, 112, 255, 255, 159, 0, 0, 0, 192, 255, 255, 12, 208, 255, 255, 13, 0, 0, 0, 32, 255, 255, 47, 241, 255, 255, 5, 0, 0, 0, 0, 101, 102, 38, 244, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 245, 255, 239, 0, 0, 0, 0, 0, 0, 0, 0, 246, 255, 207, 0, 0, 0, 0, 0, 0, 0, 0, 245, 255, 239, 0, 0, 0, 0, 0, 0, 0, 0, 244, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 241, 255, 255, 4, 0, 0, 0, 0, 135, 136, 56, 208, 255, 255, 13, 0, 0, 0, 32, 255, 255, 31, 112, 255, 255, 143, 0, 0, 0, 192, 255, 255, 12, 0, 254, 255, 255, 43, 0, 48, 252, 255, 255, 6, 0, 246, 255, 255, 255, 206, 254, 255, 255, 191, 0, 0, 144, 255, 255, 255, 255, 255, 255, 255, 30, 0, 0, 0, 248, 255, 255, 255, 255, 255, 239, 3, 0, 0, 0, 64, 253, 255, 255, 255, 255, 27, 0, 0, 0, 0, 0, 64, 217, 254, 206, 56, 0, 0, 0 ];

/// 'd' (ASCII 100)
const Font_TTSatoshi_DemiBold_42_glyph_100: [u8; 335] = [ 22, 30, 26, 1, 29, 0, 0, 0, 0, 0, 0, 0, 0, 245, 255, 223, 0, 0, 0, 0, 0, 0, 0, 0, 245, 255, 223, 0, 0, 0, 0, 0, 0, 0, 0, 245, 255, 223, 0, 0, 0, 0, 0, 0, 0, 0, 245, 255, 223, 0, 0, 0, 0, 0, 0, 0, 0, 245, 255, 223, 0, 0, 0, 0, 0, 0, 0, 0, 245, 255, 223, 0, 0, 0, 0, 0, 0, 0, 0, 245, 255, 223, 0, 0, 0, 130, 236, 223, 107, 0, 245, 255, 223, 0, 0, 145, 255, 255, 255, 255, 78, 245, 255, 223, 0, 16, 253, 255, 255, 255, 255, 255, 250, 255, 223, 0, 208, 255, 255, 255, 255, 255, 255, 255, 255, 223, 0, 250, 255, 255, 255, 222, 255, 255, 255, 255, 223, 32, 255, 255, 255, 42, 0, 96, 255, 255, 255, 223, 160, 255, 255, 127, 0, 0, 0, 226, 255, 255, 223, 224, 255, 255, 12, 0, 0, 0, 96, 255, 255, 223, 242, 255, 255, 4, 0, 0, 0, 0, 253, 255, 223, 245, 255, 255, 0, 0, 0, 0, 0, 250, 255, 223, 246, 255, 223, 0, 0, 0, 0, 0, 247, 255, 223, 246, 255, 207, 0, 0, 0, 0, 0, 245, 255, 223, 245, 255, 223, 0, 0, 0, 0, 0, 247, 255, 223, 244, 255, 255, 0, 0, 0, 0, 0, 250, 255, 223, 241, 255, 255, 4, 0, 0, 0, 0, 253, 255, 223, 208, 255, 255, 12, 0, 0, 0, 96, 255, 255, 223, 128, 255, 255, 127, 0, 0, 0, 226, 255, 255, 223, 16, 255, 255, 255, 26, 0, 80, 255, 255, 255, 223, 0, 248, 255, 255, 255, 206, 255, 255, 255, 255, 223, 0, 192, 255, 255, 255, 255, 255, 255, 255, 255, 223, 0, 16, 252, 255, 255, 255, 255, 255, 247, 255, 223, 0, 0, 144, 255, 255, 255, 255, 94, 176, 255, 223, 0, 0, 0, 130, 236, 239, 124, 1, 0, 0, 0 ];

/// 'e' (ASCII 101)
const Font_TTSatoshi_DemiBold_42_glyph_101: [u8; 258] = [ 22, 23, 24, 1, 22, 0, 0, 0, 64, 217, 254, 206, 39, 0, 0, 0, 0, 0, 64, 253, 255, 255, 255, 255, 26, 0, 0, 0, 0, 248, 255, 255, 255, 255, 255, 239, 3, 0, 0, 144, 255, 255, 255, 255, 255, 255, 255, 47, 0, 0, 246, 255, 255, 159, 70, 199, 255, 255, 223, 0, 16, 255, 255, 207, 2, 0, 0, 246, 255, 255, 7, 128, 255, 255, 30, 0, 0, 0, 128, 255, 255, 14, 208, 255, 255, 4, 0, 0, 0, 0, 253, 255, 79, 241, 255, 255, 0, 0, 0, 0, 0, 249, 255, 143, 244, 255, 223, 119, 119, 119, 119, 119, 251, 255, 175, 246, 255, 255, 255, 255, 255, 255, 255, 255, 255, 207, 246, 255, 255, 255, 255, 255, 255, 255, 255, 255, 207, 245, 255, 255, 255, 255, 255, 255, 255, 255, 255, 175, 244, 255, 207, 0, 0, 0, 0, 0, 0, 0, 0, 241, 255, 255, 1, 0, 0, 0, 0, 0, 0, 0, 192, 255, 255, 7, 0, 0, 0, 0, 0, 0, 0, 96, 255, 255, 47, 0, 0, 0, 80, 238, 238, 13, 0, 254, 255, 239, 4, 0, 0, 245, 255, 255, 8, 0, 245, 255, 255, 191, 87, 199, 255, 255, 239, 1, 0, 144, 255, 255, 255, 255, 255, 255, 255, 63, 0, 0, 0, 248, 255, 255, 255, 255, 255, 255, 4, 0, 0, 0, 64, 253, 255, 255, 255, 255, 26, 0, 0, 0, 0, 0, 64, 217, 254, 206, 56, 0, 0, 0 ];

/// 'f' (ASCII 102)
const Font_TTSatoshi_DemiBold_42_glyph_102: [u8; 215] = [ 14, 30, 15, 1, 30, 0, 0, 16, 201, 255, 255, 63, 0, 0, 227, 255, 255, 255, 63, 0, 0, 253, 255, 255, 255, 63, 0, 48, 255, 255, 255, 255, 63, 0, 96, 255, 255, 191, 120, 23, 0, 128, 255, 255, 12, 0, 0, 0, 128, 255, 255, 9, 0, 0, 0, 128, 255, 255, 9, 0, 0, 0, 128, 255, 255, 9, 0, 0, 255, 255, 255, 255, 255, 255, 63, 255, 255, 255, 255, 255, 255, 63, 255, 255, 255, 255, 255, 255, 63, 255, 255, 255, 255, 255, 255, 63, 220, 237, 255, 255, 223, 221, 61, 0, 128, 255, 255, 9, 0, 0, 0, 128, 255, 255, 9, 0, 0, 0, 128, 255, 255, 9, 0, 0, 0, 128, 255, 255, 9, 0, 0, 0, 128, 255, 255, 9, 0, 0, 0, 128, 255, 255, 9, 0, 0, 0, 128, 255, 255, 9, 0, 0, 0, 128, 255, 255, 9, 0, 0, 0, 128, 255, 255, 9, 0, 0, 0, 128, 255, 255, 9, 0, 0, 0, 128, 255, 255, 9, 0, 0, 0, 128, 255, 255, 9, 0, 0, 0, 128, 255, 255, 9, 0, 0, 0, 128, 255, 255, 9, 0, 0, 0, 128, 255, 255, 9, 0, 0, 0, 128, 255, 255, 9, 0, 0 ];

/// 'g' (ASCII 103)
const Font_TTSatoshi_DemiBold_42_glyph_103: [u8; 346] = [ 22, 31, 25, 1, 22, 0, 0, 0, 148, 237, 223, 90, 0, 0, 0, 0, 0, 0, 194, 255, 255, 255, 255, 45, 241, 255, 127, 0, 64, 255, 255, 255, 255, 255, 239, 249, 255, 127, 0, 243, 255, 255, 255, 255, 255, 255, 255, 255, 127, 0, 253, 255, 255, 239, 187, 254, 255, 255, 255, 127, 96, 255, 255, 239, 5, 0, 64, 254, 255, 255, 127, 208, 255, 255, 31, 0, 0, 0, 225, 255, 255, 127, 241, 255, 255, 6, 0, 0, 0, 96, 255, 255, 127, 244, 255, 255, 0, 0, 0, 0, 0, 255, 255, 127, 246, 255, 223, 0, 0, 0, 0, 0, 252, 255, 127, 246, 255, 207, 0, 0, 0, 0, 0, 251, 255, 127, 245, 255, 239, 0, 0, 0, 0, 0, 254, 255, 127, 243, 255, 255, 2, 0, 0, 0, 16, 255, 255, 127, 240, 255, 255, 10, 0, 0, 0, 160, 255, 255, 127, 160, 255, 255, 127, 0, 0, 0, 246, 255, 255, 127, 48, 255, 255, 255, 75, 17, 180, 255, 255, 255, 127, 0, 249, 255, 255, 255, 255, 255, 255, 255, 255, 127, 0, 192, 255, 255, 255, 255, 255, 255, 255, 255, 127, 0, 0, 251, 255, 255, 255, 255, 207, 252, 255, 127, 0, 0, 80, 253, 255, 255, 255, 7, 250, 255, 127, 0, 0, 0, 32, 134, 120, 4, 0, 250, 255, 127, 0, 0, 0, 0, 0, 0, 0, 0, 251, 255, 127, 0, 0, 0, 0, 0, 0, 0, 0, 253, 255, 95, 192, 255, 255, 8, 0, 0, 0, 16, 255, 255, 79, 144, 255, 255, 30, 0, 0, 0, 160, 255, 255, 15, 48, 255, 255, 223, 3, 0, 0, 249, 255, 255, 10, 0, 251, 255, 255, 207, 137, 235, 255, 255, 255, 3, 0, 226, 255, 255, 255, 255, 255, 255, 255, 143, 0, 0, 48, 254, 255, 255, 255, 255, 255, 255, 9, 0, 0, 0, 145, 255, 255, 255, 255, 255, 93, 0, 0, 0, 0, 0, 114, 219, 255, 222, 89, 0, 0, 0 ];

/// 'h' (ASCII 104)
const Font_TTSatoshi_DemiBold_42_glyph_104: [u8; 324] = [ 21, 29, 25, 2, 29, 241, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 241, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 241, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 241, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 241, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 241, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 241, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 241, 255, 255, 0, 147, 237, 223, 74, 0, 0, 0, 241, 255, 255, 161, 255, 255, 255, 255, 44, 0, 0, 241, 255, 255, 253, 255, 255, 255, 255, 239, 2, 0, 241, 255, 255, 255, 255, 255, 255, 255, 255, 13, 0, 241, 255, 255, 255, 255, 254, 255, 255, 255, 127, 0, 241, 255, 255, 255, 40, 0, 212, 255, 255, 239, 0, 241, 255, 255, 63, 0, 0, 0, 252, 255, 255, 2, 241, 255, 255, 9, 0, 0, 0, 242, 255, 255, 5, 241, 255, 255, 3, 0, 0, 0, 208, 255, 255, 6, 241, 255, 255, 0, 0, 0, 0, 176, 255, 255, 7, 241, 255, 255, 0, 0, 0, 0, 160, 255, 255, 7, 241, 255, 255, 0, 0, 0, 0, 160, 255, 255, 7, 241, 255, 255, 0, 0, 0, 0, 160, 255, 255, 7, 241, 255, 255, 0, 0, 0, 0, 160, 255, 255, 7, 241, 255, 255, 0, 0, 0, 0, 160, 255, 255, 7, 241, 255, 255, 0, 0, 0, 0, 160, 255, 255, 7, 241, 255, 255, 0, 0, 0, 0, 160, 255, 255, 7, 241, 255, 255, 0, 0, 0, 0, 160, 255, 255, 7, 241, 255, 255, 0, 0, 0, 0, 160, 255, 255, 7, 241, 255, 255, 0, 0, 0, 0, 160, 255, 255, 7, 241, 255, 255, 0, 0, 0, 0, 160, 255, 255, 7, 241, 255, 255, 0, 0, 0, 0, 160, 255, 255, 7 ];

/// 'i' (ASCII 105)
const Font_TTSatoshi_DemiBold_42_glyph_105: [u8; 125] = [ 7, 30, 11, 2, 30, 247, 255, 255, 0, 247, 255, 255, 0, 247, 255, 255, 0, 247, 255, 255, 0, 198, 204, 204, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 244, 255, 207, 0, 244, 255, 207, 0, 244, 255, 207, 0, 244, 255, 207, 0, 244, 255, 207, 0, 244, 255, 207, 0, 244, 255, 207, 0, 244, 255, 207, 0, 244, 255, 207, 0, 244, 255, 207, 0, 244, 255, 207, 0, 244, 255, 207, 0, 244, 255, 207, 0, 244, 255, 207, 0, 244, 255, 207, 0, 244, 255, 207, 0, 244, 255, 207, 0, 244, 255, 207, 0, 244, 255, 207, 0, 244, 255, 207, 0, 244, 255, 207, 0 ];

/// 'j' (ASCII 106)
const Font_TTSatoshi_DemiBold_42_glyph_106: [u8; 239] = [ 12, 39, 14, 0, 30, 0, 0, 112, 255, 255, 15, 0, 0, 112, 255, 255, 15, 0, 0, 112, 255, 255, 15, 0, 0, 112, 255, 255, 15, 0, 0, 96, 204, 204, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 255, 255, 12, 0, 0, 64, 255, 255, 12, 0, 0, 64, 255, 255, 12, 0, 0, 64, 255, 255, 12, 0, 0, 64, 255, 255, 12, 0, 0, 64, 255, 255, 12, 0, 0, 64, 255, 255, 12, 0, 0, 64, 255, 255, 12, 0, 0, 64, 255, 255, 12, 0, 0, 64, 255, 255, 12, 0, 0, 64, 255, 255, 12, 0, 0, 64, 255, 255, 12, 0, 0, 64, 255, 255, 12, 0, 0, 64, 255, 255, 12, 0, 0, 64, 255, 255, 12, 0, 0, 64, 255, 255, 12, 0, 0, 64, 255, 255, 12, 0, 0, 64, 255, 255, 12, 0, 0, 64, 255, 255, 12, 0, 0, 64, 255, 255, 12, 0, 0, 64, 255, 255, 12, 0, 0, 64, 255, 255, 12, 0, 0, 64, 255, 255, 12, 0, 0, 80, 255, 255, 12, 0, 0, 161, 255, 255, 12, 240, 255, 255, 255, 255, 10, 240, 255, 255, 255, 255, 7, 240, 255, 255, 255, 239, 1, 240, 255, 255, 255, 78, 0, 176, 187, 187, 105, 0, 0 ];

/// 'k' (ASCII 107)
const Font_TTSatoshi_DemiBold_42_glyph_107: [u8; 324] = [ 21, 29, 22, 2, 29, 241, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 241, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 241, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 241, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 241, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 241, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 241, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 241, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 241, 255, 255, 0, 0, 0, 96, 255, 255, 159, 0, 241, 255, 255, 0, 0, 0, 244, 255, 255, 11, 0, 241, 255, 255, 0, 0, 32, 254, 255, 223, 0, 0, 241, 255, 255, 0, 0, 209, 255, 255, 46, 0, 0, 241, 255, 255, 0, 0, 252, 255, 255, 3, 0, 0, 241, 255, 255, 0, 160, 255, 255, 95, 0, 0, 0, 241, 255, 255, 0, 247, 255, 255, 7, 0, 0, 0, 241, 255, 255, 80, 255, 255, 159, 0, 0, 0, 0, 241, 255, 255, 243, 255, 255, 11, 0, 0, 0, 0, 241, 255, 255, 254, 255, 223, 0, 0, 0, 0, 0, 241, 255, 255, 255, 255, 127, 0, 0, 0, 0, 0, 241, 255, 255, 250, 255, 255, 4, 0, 0, 0, 0, 241, 255, 255, 192, 255, 255, 46, 0, 0, 0, 0, 241, 255, 255, 16, 253, 255, 223, 1, 0, 0, 0, 241, 255, 255, 0, 242, 255, 255, 12, 0, 0, 0, 241, 255, 255, 0, 64, 255, 255, 191, 0, 0, 0, 241, 255, 255, 0, 0, 246, 255, 255, 9, 0, 0, 241, 255, 255, 0, 0, 128, 255, 255, 111, 0, 0, 241, 255, 255, 0, 0, 0, 250, 255, 255, 4, 0, 241, 255, 255, 0, 0, 0, 192, 255, 255, 63, 0, 241, 255, 255, 0, 0, 0, 16, 254, 255, 239, 2 ];

/// 'l' (ASCII 108)
const Font_TTSatoshi_DemiBold_42_glyph_108: [u8; 121] = [ 7, 29, 11, 2, 29, 241, 255, 255, 0, 241, 255, 255, 0, 241, 255, 255, 0, 241, 255, 255, 0, 241, 255, 255, 0, 241, 255, 255, 0, 241, 255, 255, 0, 241, 255, 255, 0, 241, 255, 255, 0, 241, 255, 255, 0, 241, 255, 255, 0, 241, 255, 255, 0, 241, 255, 255, 0, 241, 255, 255, 0, 241, 255, 255, 0, 241, 255, 255, 0, 241, 255, 255, 0, 241, 255, 255, 0, 241, 255, 255, 0, 241, 255, 255, 0, 241, 255, 255, 0, 241, 255, 255, 0, 241, 255, 255, 0, 241, 255, 255, 0, 241, 255, 255, 0, 241, 255, 255, 0, 241, 255, 255, 0, 241, 255, 255, 0, 241, 255, 255, 0 ];

/// 'm' (ASCII 109)
const Font_TTSatoshi_DemiBold_42_glyph_109: [u8; 357] = [ 32, 22, 36, 2, 22, 0, 0, 0, 16, 199, 238, 140, 1, 0, 0, 165, 253, 206, 23, 0, 0, 244, 255, 63, 229, 255, 255, 255, 111, 0, 211, 255, 255, 255, 239, 5, 0, 244, 255, 207, 255, 255, 255, 255, 255, 86, 255, 255, 255, 255, 255, 95, 0, 244, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 239, 1, 244, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 254, 255, 255, 255, 9, 244, 255, 255, 191, 2, 80, 254, 255, 255, 255, 42, 0, 246, 255, 255, 31, 244, 255, 255, 11, 0, 0, 243, 255, 255, 159, 0, 0, 80, 255, 255, 95, 244, 255, 255, 2, 0, 0, 176, 255, 255, 31, 0, 0, 0, 254, 255, 127, 244, 255, 239, 0, 0, 0, 144, 255, 255, 13, 0, 0, 0, 251, 255, 143, 244, 255, 223, 0, 0, 0, 112, 255, 255, 11, 0, 0, 0, 249, 255, 143, 244, 255, 207, 0, 0, 0, 112, 255, 255, 11, 0, 0, 0, 249, 255, 143, 244, 255, 207, 0, 0, 0, 112, 255, 255, 11, 0, 0, 0, 249, 255, 143, 244, 255, 207, 0, 0, 0, 112, 255, 255, 11, 0, 0, 0, 249, 255, 143, 244, 255, 207, 0, 0, 0, 112, 255, 255, 11, 0, 0, 0, 249, 255, 143, 244, 255, 207, 0, 0, 0, 112, 255, 255, 11, 0, 0, 0, 249, 255, 143, 244, 255, 207, 0, 0, 0, 112, 255, 255, 11, 0, 0, 0, 249, 255, 143, 244, 255, 207, 0, 0, 0, 112, 255, 255, 11, 0, 0, 0, 249, 255, 143, 244, 255, 207, 0, 0, 0, 112, 255, 255, 11, 0, 0, 0, 249, 255, 143, 244, 255, 207, 0, 0, 0, 112, 255, 255, 11, 0, 0, 0, 249, 255, 143, 244, 255, 207, 0, 0, 0, 112, 255, 255, 11, 0, 0, 0, 249, 255, 143, 244, 255, 207, 0, 0, 0, 112, 255, 255, 11, 0, 0, 0, 249, 255, 143, 244, 255, 207, 0, 0, 0, 112, 255, 255, 11, 0, 0, 0, 249, 255, 143 ];

/// 'n' (ASCII 110)
const Font_TTSatoshi_DemiBold_42_glyph_110: [u8; 247] = [ 21, 22, 25, 2, 22, 0, 0, 0, 0, 165, 253, 222, 57, 0, 0, 0, 244, 255, 63, 212, 255, 255, 255, 255, 10, 0, 0, 244, 255, 207, 255, 255, 255, 255, 255, 223, 0, 0, 244, 255, 255, 255, 255, 255, 255, 255, 255, 10, 0, 244, 255, 255, 255, 255, 254, 255, 255, 255, 63, 0, 244, 255, 255, 239, 22, 16, 229, 255, 255, 175, 0, 244, 255, 255, 30, 0, 0, 16, 254, 255, 239, 0, 244, 255, 255, 5, 0, 0, 0, 246, 255, 255, 1, 244, 255, 255, 0, 0, 0, 0, 241, 255, 255, 3, 244, 255, 223, 0, 0, 0, 0, 224, 255, 255, 3, 244, 255, 207, 0, 0, 0, 0, 208, 255, 255, 3, 244, 255, 207, 0, 0, 0, 0, 208, 255, 255, 3, 244, 255, 207, 0, 0, 0, 0, 208, 255, 255, 3, 244, 255, 207, 0, 0, 0, 0, 208, 255, 255, 3, 244, 255, 207, 0, 0, 0, 0, 208, 255, 255, 3, 244, 255, 207, 0, 0, 0, 0, 208, 255, 255, 3, 244, 255, 207, 0, 0, 0, 0, 208, 255, 255, 3, 244, 255, 207, 0, 0, 0, 0, 208, 255, 255, 3, 244, 255, 207, 0, 0, 0, 0, 208, 255, 255, 3, 244, 255, 207, 0, 0, 0, 0, 208, 255, 255, 3, 244, 255, 207, 0, 0, 0, 0, 208, 255, 255, 3, 244, 255, 207, 0, 0, 0, 0, 208, 255, 255, 3 ];

/// 'o' (ASCII 111)
const Font_TTSatoshi_DemiBold_42_glyph_111: [u8; 281] = [ 23, 23, 25, 1, 22, 0, 0, 0, 64, 217, 254, 206, 56, 0, 0, 0, 0, 0, 0, 80, 253, 255, 255, 255, 255, 43, 0, 0, 0, 0, 0, 248, 255, 255, 255, 255, 255, 255, 4, 0, 0, 0, 144, 255, 255, 255, 255, 255, 255, 255, 79, 0, 0, 0, 246, 255, 255, 255, 206, 255, 255, 255, 255, 2, 0, 0, 254, 255, 255, 43, 0, 64, 254, 255, 255, 10, 0, 112, 255, 255, 143, 0, 0, 0, 193, 255, 255, 47, 0, 208, 255, 255, 13, 0, 0, 0, 48, 255, 255, 143, 0, 241, 255, 255, 4, 0, 0, 0, 0, 250, 255, 207, 0, 244, 255, 255, 0, 0, 0, 0, 0, 245, 255, 255, 0, 245, 255, 223, 0, 0, 0, 0, 0, 243, 255, 255, 0, 246, 255, 191, 0, 0, 0, 0, 0, 241, 255, 255, 1, 245, 255, 223, 0, 0, 0, 0, 0, 243, 255, 255, 0, 244, 255, 255, 0, 0, 0, 0, 0, 245, 255, 255, 0, 241, 255, 255, 4, 0, 0, 0, 0, 249, 255, 207, 0, 208, 255, 255, 13, 0, 0, 0, 32, 255, 255, 143, 0, 112, 255, 255, 143, 0, 0, 0, 193, 255, 255, 47, 0, 0, 254, 255, 255, 43, 0, 64, 254, 255, 255, 10, 0, 0, 246, 255, 255, 255, 223, 255, 255, 255, 255, 2, 0, 0, 144, 255, 255, 255, 255, 255, 255, 255, 79, 0, 0, 0, 0, 249, 255, 255, 255, 255, 255, 255, 4, 0, 0, 0, 0, 80, 253, 255, 255, 255, 255, 43, 0, 0, 0, 0, 0, 0, 64, 217, 254, 206, 56, 0, 0, 0, 0 ];

/// 'p' (ASCII 112)
const Font_TTSatoshi_DemiBold_42_glyph_112: [u8; 346] = [ 22, 31, 26, 2, 22, 0, 0, 0, 0, 148, 237, 223, 90, 0, 0, 0, 244, 255, 63, 177, 255, 255, 255, 255, 77, 0, 0, 244, 255, 143, 253, 255, 255, 255, 255, 255, 6, 0, 244, 255, 255, 255, 255, 255, 255, 255, 255, 95, 0, 244, 255, 255, 255, 255, 222, 255, 255, 255, 255, 1, 244, 255, 255, 255, 43, 0, 80, 254, 255, 255, 9, 244, 255, 255, 159, 0, 0, 0, 209, 255, 255, 31, 244, 255, 255, 14, 0, 0, 0, 64, 255, 255, 95, 244, 255, 255, 6, 0, 0, 0, 0, 252, 255, 159, 244, 255, 255, 2, 0, 0, 0, 0, 247, 255, 207, 244, 255, 255, 0, 0, 0, 0, 0, 245, 255, 223, 244, 255, 223, 0, 0, 0, 0, 0, 243, 255, 239, 244, 255, 239, 0, 0, 0, 0, 0, 245, 255, 223, 244, 255, 255, 1, 0, 0, 0, 0, 247, 255, 207, 244, 255, 255, 4, 0, 0, 0, 0, 251, 255, 159, 244, 255, 255, 12, 0, 0, 0, 64, 255, 255, 95, 244, 255, 255, 127, 0, 0, 0, 209, 255, 255, 31, 244, 255, 255, 255, 26, 0, 64, 254, 255, 255, 9, 244, 255, 255, 255, 255, 206, 255, 255, 255, 255, 2, 244, 255, 255, 255, 255, 255, 255, 255, 255, 95, 0, 244, 255, 239, 253, 255, 255, 255, 255, 255, 6, 0, 244, 255, 207, 177, 255, 255, 255, 255, 77, 0, 0, 244, 255, 207, 0, 148, 253, 223, 90, 0, 0, 0, 244, 255, 207, 0, 0, 0, 0, 0, 0, 0, 0, 244, 255, 207, 0, 0, 0, 0, 0, 0, 0, 0, 244, 255, 207, 0, 0, 0, 0, 0, 0, 0, 0, 244, 255, 207, 0, 0, 0, 0, 0, 0, 0, 0, 244, 255, 207, 0, 0, 0, 0, 0, 0, 0, 0, 244, 255, 207, 0, 0, 0, 0, 0, 0, 0, 0, 244, 255, 207, 0, 0, 0, 0, 0, 0, 0, 0, 244, 255, 207, 0, 0, 0, 0, 0, 0, 0, 0 ];

/// 'q' (ASCII 113)
const Font_TTSatoshi_DemiBold_42_glyph_113: [u8; 346] = [ 22, 31, 26, 1, 22, 0, 0, 0, 130, 236, 239, 107, 1, 0, 0, 0, 0, 0, 144, 255, 255, 255, 255, 110, 176, 255, 223, 0, 16, 253, 255, 255, 255, 255, 255, 247, 255, 223, 0, 208, 255, 255, 255, 255, 255, 255, 255, 255, 223, 0, 249, 255, 255, 255, 222, 255, 255, 255, 255, 223, 32, 255, 255, 255, 42, 0, 96, 255, 255, 255, 223, 144, 255, 255, 127, 0, 0, 0, 226, 255, 255, 223, 224, 255, 255, 12, 0, 0, 0, 96, 255, 255, 223, 242, 255, 255, 4, 0, 0, 0, 0, 253, 255, 223, 244, 255, 255, 0, 0, 0, 0, 0, 250, 255, 223, 246, 255, 223, 0, 0, 0, 0, 0, 247, 255, 223, 246, 255, 207, 0, 0, 0, 0, 0, 245, 255, 223, 246, 255, 223, 0, 0, 0, 0, 0, 247, 255, 223, 244, 255, 255, 0, 0, 0, 0, 0, 250, 255, 223, 242, 255, 255, 4, 0, 0, 0, 0, 253, 255, 223, 224, 255, 255, 12, 0, 0, 0, 96, 255, 255, 223, 144, 255, 255, 127, 0, 0, 0, 226, 255, 255, 223, 32, 255, 255, 255, 26, 0, 80, 255, 255, 255, 223, 0, 249, 255, 255, 255, 206, 255, 255, 255, 255, 223, 0, 208, 255, 255, 255, 255, 255, 255, 255, 255, 223, 0, 16, 253, 255, 255, 255, 255, 255, 251, 255, 223, 0, 0, 145, 255, 255, 255, 255, 94, 245, 255, 223, 0, 0, 0, 130, 236, 239, 107, 1, 245, 255, 223, 0, 0, 0, 0, 0, 0, 0, 0, 245, 255, 223, 0, 0, 0, 0, 0, 0, 0, 0, 245, 255, 223, 0, 0, 0, 0, 0, 0, 0, 0, 245, 255, 223, 0, 0, 0, 0, 0, 0, 0, 0, 245, 255, 223, 0, 0, 0, 0, 0, 0, 0, 0, 245, 255, 223, 0, 0, 0, 0, 0, 0, 0, 0, 245, 255, 223, 0, 0, 0, 0, 0, 0, 0, 0, 245, 255, 223, 0, 0, 0, 0, 0, 0, 0, 0, 245, 255, 223 ];

/// 'r' (ASCII 114)
const Font_TTSatoshi_DemiBold_42_glyph_114: [u8; 152] = [ 14, 21, 16, 2, 21, 244, 255, 63, 144, 253, 255, 127, 244, 255, 159, 253, 255, 255, 127, 244, 255, 255, 255, 255, 255, 127, 244, 255, 255, 255, 255, 255, 127, 244, 255, 255, 255, 239, 221, 109, 244, 255, 255, 143, 0, 0, 0, 244, 255, 255, 7, 0, 0, 0, 244, 255, 255, 0, 0, 0, 0, 244, 255, 239, 0, 0, 0, 0, 244, 255, 207, 0, 0, 0, 0, 244, 255, 207, 0, 0, 0, 0, 244, 255, 207, 0, 0, 0, 0, 244, 255, 207, 0, 0, 0, 0, 244, 255, 207, 0, 0, 0, 0, 244, 255, 207, 0, 0, 0, 0, 244, 255, 207, 0, 0, 0, 0, 244, 255, 207, 0, 0, 0, 0, 244, 255, 207, 0, 0, 0, 0, 244, 255, 207, 0, 0, 0, 0, 244, 255, 207, 0, 0, 0, 0, 244, 255, 207, 0, 0, 0, 0 ];

/// 's' (ASCII 115)
const Font_TTSatoshi_DemiBold_42_glyph_115: [u8; 235] = [ 19, 23, 21, 1, 22, 0, 0, 114, 235, 255, 206, 40, 0, 0, 0, 0, 161, 255, 255, 255, 255, 255, 27, 0, 0, 16, 253, 255, 255, 255, 255, 255, 239, 2, 0, 176, 255, 255, 255, 255, 255, 255, 255, 30, 0, 243, 255, 255, 109, 35, 165, 255, 255, 143, 0, 248, 255, 223, 0, 0, 0, 247, 255, 239, 0, 249, 255, 143, 0, 0, 0, 224, 238, 238, 1, 248, 255, 191, 0, 0, 0, 0, 0, 0, 0, 245, 255, 255, 91, 1, 0, 0, 0, 0, 0, 225, 255, 255, 255, 239, 155, 4, 0, 0, 0, 80, 255, 255, 255, 255, 255, 255, 43, 0, 0, 0, 229, 255, 255, 255, 255, 255, 255, 6, 0, 0, 16, 216, 255, 255, 255, 255, 255, 63, 0, 0, 0, 0, 81, 184, 254, 255, 255, 223, 0, 0, 0, 0, 0, 0, 32, 250, 255, 255, 2, 0, 0, 0, 0, 0, 0, 208, 255, 255, 4, 253, 255, 79, 0, 0, 0, 176, 255, 255, 6, 249, 255, 207, 0, 0, 0, 225, 255, 255, 4, 243, 255, 255, 109, 35, 115, 254, 255, 255, 0, 160, 255, 255, 255, 255, 255, 255, 255, 143, 0, 0, 251, 255, 255, 255, 255, 255, 255, 11, 0, 0, 128, 255, 255, 255, 255, 255, 143, 0, 0, 0, 0, 97, 218, 254, 222, 106, 1, 0, 0 ];

/// 't' (ASCII 116)
const Font_TTSatoshi_DemiBold_42_glyph_116: [u8; 201] = [ 14, 28, 16, 1, 28, 0, 48, 85, 85, 3, 0, 0, 0, 144, 255, 255, 8, 0, 0, 0, 144, 255, 255, 8, 0, 0, 0, 144, 255, 255, 8, 0, 0, 0, 144, 255, 255, 8, 0, 0, 0, 144, 255, 255, 8, 0, 0, 0, 144, 255, 255, 8, 0, 0, 255, 255, 255, 255, 255, 255, 111, 255, 255, 255, 255, 255, 255, 111, 255, 255, 255, 255, 255, 255, 111, 255, 255, 255, 255, 255, 255, 111, 220, 237, 255, 255, 222, 221, 93, 0, 144, 255, 255, 8, 0, 0, 0, 144, 255, 255, 8, 0, 0, 0, 144, 255, 255, 8, 0, 0, 0, 144, 255, 255, 8, 0, 0, 0, 144, 255, 255, 8, 0, 0, 0, 144, 255, 255, 8, 0, 0, 0, 144, 255, 255, 8, 0, 0, 0, 144, 255, 255, 8, 0, 0, 0, 144, 255, 255, 8, 0, 0, 0, 144, 255, 255, 9, 0, 0, 0, 144, 255, 255, 11, 0, 0, 0, 128, 255, 255, 207, 170, 74, 0, 80, 255, 255, 255, 255, 111, 0, 16, 254, 255, 255, 255, 111, 0, 0, 245, 255, 255, 255, 111, 0, 0, 32, 217, 255, 255, 111 ];

/// 'u' (ASCII 117)
const Font_TTSatoshi_DemiBold_42_glyph_117: [u8; 247] = [ 21, 22, 25, 2, 21, 244, 255, 207, 0, 0, 0, 0, 208, 255, 255, 3, 244, 255, 207, 0, 0, 0, 0, 208, 255, 255, 3, 244, 255, 207, 0, 0, 0, 0, 208, 255, 255, 3, 244, 255, 207, 0, 0, 0, 0, 208, 255, 255, 3, 244, 255, 207, 0, 0, 0, 0, 208, 255, 255, 3, 244, 255, 207, 0, 0, 0, 0, 208, 255, 255, 3, 244, 255, 207, 0, 0, 0, 0, 208, 255, 255, 3, 244, 255, 207, 0, 0, 0, 0, 208, 255, 255, 3, 244, 255, 207, 0, 0, 0, 0, 208, 255, 255, 3, 244, 255, 207, 0, 0, 0, 0, 208, 255, 255, 3, 244, 255, 207, 0, 0, 0, 0, 208, 255, 255, 3, 244, 255, 207, 0, 0, 0, 0, 224, 255, 255, 3, 244, 255, 223, 0, 0, 0, 0, 240, 255, 255, 3, 244, 255, 255, 0, 0, 0, 0, 242, 255, 255, 3, 243, 255, 255, 4, 0, 0, 0, 247, 255, 255, 3, 240, 255, 255, 30, 0, 0, 32, 255, 255, 255, 3, 176, 255, 255, 239, 4, 16, 246, 255, 255, 255, 3, 80, 255, 255, 255, 255, 254, 255, 255, 255, 255, 3, 0, 252, 255, 255, 255, 255, 255, 255, 255, 255, 3, 0, 209, 255, 255, 255, 255, 255, 207, 255, 255, 3, 0, 16, 251, 255, 255, 255, 223, 83, 255, 255, 3, 0, 0, 48, 217, 255, 173, 5, 0, 0, 0, 0 ];

/// 'v' (ASCII 118)
const Font_TTSatoshi_DemiBold_42_glyph_118: [u8; 236] = [ 22, 21, 22, 0, 21, 250, 255, 175, 0, 0, 0, 0, 0, 249, 255, 207, 245, 255, 255, 0, 0, 0, 0, 0, 254, 255, 127, 240, 255, 255, 5, 0, 0, 0, 64, 255, 255, 31, 160, 255, 255, 11, 0, 0, 0, 144, 255, 255, 11, 64, 255, 255, 31, 0, 0, 0, 224, 255, 255, 6, 0, 254, 255, 111, 0, 0, 0, 244, 255, 255, 1, 0, 249, 255, 191, 0, 0, 0, 250, 255, 175, 0, 0, 243, 255, 255, 1, 0, 0, 255, 255, 95, 0, 0, 224, 255, 255, 6, 0, 80, 255, 255, 15, 0, 0, 128, 255, 255, 12, 0, 160, 255, 255, 10, 0, 0, 48, 255, 255, 31, 0, 240, 255, 255, 4, 0, 0, 0, 253, 255, 127, 0, 245, 255, 239, 0, 0, 0, 0, 247, 255, 207, 0, 251, 255, 159, 0, 0, 0, 0, 242, 255, 255, 18, 255, 255, 63, 0, 0, 0, 0, 192, 255, 255, 103, 255, 255, 14, 0, 0, 0, 0, 112, 255, 255, 255, 255, 255, 8, 0, 0, 0, 0, 16, 255, 255, 255, 255, 255, 2, 0, 0, 0, 0, 0, 252, 255, 255, 255, 223, 0, 0, 0, 0, 0, 0, 246, 255, 255, 255, 127, 0, 0, 0, 0, 0, 0, 241, 255, 255, 255, 47, 0, 0, 0, 0, 0, 0, 176, 255, 255, 255, 12, 0, 0, 0 ];

/// 'w' (ASCII 119)
const Font_TTSatoshi_DemiBold_42_glyph_119: [u8; 362] = [ 34, 21, 34, 0, 21, 247, 255, 223, 0, 0, 0, 64, 255, 255, 223, 0, 0, 0, 64, 255, 255, 31, 243, 255, 255, 1, 0, 0, 128, 255, 255, 255, 1, 0, 0, 128, 255, 255, 13, 240, 255, 255, 5, 0, 0, 192, 255, 255, 255, 5, 0, 0, 192, 255, 255, 8, 176, 255, 255, 9, 0, 0, 241, 255, 255, 255, 9, 0, 0, 241, 255, 255, 4, 96, 255, 255, 14, 0, 0, 245, 255, 255, 255, 14, 0, 0, 245, 255, 255, 0, 32, 255, 255, 47, 0, 0, 249, 255, 255, 255, 47, 0, 0, 249, 255, 207, 0, 0, 254, 255, 111, 0, 0, 253, 255, 255, 255, 111, 0, 0, 253, 255, 127, 0, 0, 250, 255, 175, 0, 16, 255, 255, 217, 255, 175, 0, 16, 255, 255, 63, 0, 0, 246, 255, 239, 0, 96, 255, 255, 113, 255, 239, 0, 80, 255, 255, 15, 0, 0, 241, 255, 255, 3, 160, 255, 223, 48, 255, 255, 3, 144, 255, 255, 11, 0, 0, 208, 255, 255, 7, 224, 255, 159, 0, 255, 255, 7, 224, 255, 255, 7, 0, 0, 144, 255, 255, 11, 242, 255, 79, 0, 251, 255, 11, 242, 255, 255, 2, 0, 0, 80, 255, 255, 15, 246, 255, 31, 0, 247, 255, 15, 246, 255, 239, 0, 0, 0, 16, 255, 255, 79, 251, 255, 12, 0, 243, 255, 95, 250, 255, 175, 0, 0, 0, 0, 252, 255, 255, 255, 255, 8, 0, 224, 255, 255, 255, 255, 111, 0, 0, 0, 0, 248, 255, 255, 255, 255, 4, 0, 160, 255, 255, 255, 255, 31, 0, 0, 0, 0, 244, 255, 255, 255, 255, 0, 0, 96, 255, 255, 255, 255, 13, 0, 0, 0, 0, 240, 255, 255, 255, 207, 0, 0, 32, 255, 255, 255, 255, 9, 0, 0, 0, 0, 176, 255, 255, 255, 127, 0, 0, 0, 254, 255, 255, 255, 5, 0, 0, 0, 0, 112, 255, 255, 255, 63, 0, 0, 0, 250, 255, 255, 255, 1, 0, 0, 0, 0, 48, 255, 255, 255, 15, 0, 0, 0, 246, 255, 255, 207, 0, 0, 0 ];

/// 'x' (ASCII 120)
const Font_TTSatoshi_DemiBold_42_glyph_120: [u8; 236] = [ 22, 21, 22, 0, 21, 245, 255, 255, 7, 0, 0, 0, 192, 255, 255, 30, 160, 255, 255, 47, 0, 0, 0, 247, 255, 255, 5, 16, 254, 255, 191, 0, 0, 32, 255, 255, 175, 0, 0, 245, 255, 255, 5, 0, 176, 255, 255, 30, 0, 0, 176, 255, 255, 30, 0, 245, 255, 255, 5, 0, 0, 16, 254, 255, 175, 16, 254, 255, 191, 0, 0, 0, 0, 246, 255, 255, 183, 255, 255, 30, 0, 0, 0, 0, 176, 255, 255, 255, 255, 255, 5, 0, 0, 0, 0, 16, 255, 255, 255, 255, 191, 0, 0, 0, 0, 0, 0, 246, 255, 255, 255, 31, 0, 0, 0, 0, 0, 0, 240, 255, 255, 255, 10, 0, 0, 0, 0, 0, 0, 249, 255, 255, 255, 79, 0, 0, 0, 0, 0, 64, 255, 255, 255, 255, 223, 0, 0, 0, 0, 0, 224, 255, 255, 255, 255, 255, 9, 0, 0, 0, 0, 249, 255, 255, 98, 255, 255, 79, 0, 0, 0, 64, 255, 255, 111, 0, 252, 255, 239, 0, 0, 0, 224, 255, 255, 12, 0, 242, 255, 255, 9, 0, 0, 249, 255, 255, 2, 0, 112, 255, 255, 79, 0, 64, 255, 255, 127, 0, 0, 0, 252, 255, 239, 0, 225, 255, 255, 12, 0, 0, 0, 242, 255, 255, 9, 250, 255, 255, 2, 0, 0, 0, 112, 255, 255, 79 ];

/// 'y' (ASCII 121)
const Font_TTSatoshi_DemiBold_42_glyph_121: [u8; 365] = [ 23, 30, 22, 0, 21, 252, 255, 175, 0, 0, 0, 0, 0, 245, 255, 255, 1, 246, 255, 255, 1, 0, 0, 0, 0, 251, 255, 191, 0, 240, 255, 255, 6, 0, 0, 0, 16, 255, 255, 95, 0, 160, 255, 255, 12, 0, 0, 0, 80, 255, 255, 15, 0, 48, 255, 255, 47, 0, 0, 0, 176, 255, 255, 10, 0, 0, 253, 255, 143, 0, 0, 0, 241, 255, 255, 4, 0, 0, 247, 255, 239, 0, 0, 0, 246, 255, 239, 0, 0, 0, 241, 255, 255, 4, 0, 0, 251, 255, 159, 0, 0, 0, 176, 255, 255, 10, 0, 16, 255, 255, 63, 0, 0, 0, 80, 255, 255, 15, 0, 96, 255, 255, 13, 0, 0, 0, 0, 254, 255, 111, 0, 176, 255, 255, 8, 0, 0, 0, 0, 249, 255, 207, 0, 241, 255, 255, 2, 0, 0, 0, 0, 242, 255, 255, 2, 246, 255, 207, 0, 0, 0, 0, 0, 192, 255, 255, 8, 251, 255, 127, 0, 0, 0, 0, 0, 96, 255, 255, 62, 255, 255, 31, 0, 0, 0, 0, 0, 16, 255, 255, 255, 255, 255, 11, 0, 0, 0, 0, 0, 0, 250, 255, 255, 255, 255, 5, 0, 0, 0, 0, 0, 0, 244, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 224, 255, 255, 255, 175, 0, 0, 0, 0, 0, 0, 0, 128, 255, 255, 255, 79, 0, 0, 0, 0, 0, 0, 0, 32, 255, 255, 255, 14, 0, 0, 0, 0, 0, 0, 0, 0, 251, 255, 255, 9, 0, 0, 0, 0, 0, 0, 0, 0, 246, 255, 255, 3, 0, 0, 0, 0, 0, 0, 0, 0, 250, 255, 239, 0, 0, 0, 0, 0, 0, 0, 0, 64, 255, 255, 143, 0, 0, 0, 0, 0, 0, 247, 255, 255, 255, 255, 47, 0, 0, 0, 0, 0, 0, 247, 255, 255, 255, 255, 11, 0, 0, 0, 0, 0, 0, 247, 255, 255, 255, 255, 4, 0, 0, 0, 0, 0, 0, 247, 255, 255, 255, 111, 0, 0, 0, 0, 0, 0, 0, 181, 187, 187, 138, 2, 0, 0, 0, 0, 0, 0 ];

/// 'z' (ASCII 122)
const Font_TTSatoshi_DemiBold_42_glyph_122: [u8; 194] = [ 18, 21, 20, 1, 21, 249, 255, 255, 255, 255, 255, 255, 255, 111, 249, 255, 255, 255, 255, 255, 255, 255, 111, 249, 255, 255, 255, 255, 255, 255, 255, 111, 249, 255, 255, 255, 255, 255, 255, 255, 111, 166, 170, 170, 170, 170, 252, 255, 255, 79, 0, 0, 0, 0, 16, 254, 255, 255, 7, 0, 0, 0, 0, 209, 255, 255, 159, 0, 0, 0, 0, 0, 252, 255, 255, 11, 0, 0, 0, 0, 160, 255, 255, 207, 0, 0, 0, 0, 0, 249, 255, 255, 29, 0, 0, 0, 0, 112, 255, 255, 239, 2, 0, 0, 0, 0, 245, 255, 255, 63, 0, 0, 0, 0, 64, 255, 255, 255, 4, 0, 0, 0, 0, 242, 255, 255, 111, 0, 0, 0, 0, 16, 254, 255, 255, 8, 0, 0, 0, 0, 208, 255, 255, 175, 0, 0, 0, 0, 0, 251, 255, 255, 175, 170, 170, 170, 170, 106, 253, 255, 255, 255, 255, 255, 255, 255, 159, 253, 255, 255, 255, 255, 255, 255, 255, 159, 253, 255, 255, 255, 255, 255, 255, 255, 159, 253, 255, 255, 255, 255, 255, 255, 255, 159 ];

/// Nonprintable glyph (inverse colors of '?')
const Font_TTSatoshi_DemiBold_42_glyph_nonprintable: [u8; 305] = [ 20, 30, 22, 1, 30, 255, 255, 207, 55, 1, 32, 132, 254, 255, 255, 255, 223, 4, 0, 0, 0, 0, 96, 255, 255, 255, 10, 0, 0, 0, 0, 0, 0, 210, 255, 207, 0, 0, 0, 0, 0, 0, 0, 32, 254, 47, 0, 0, 0, 0, 0, 0, 0, 0, 246, 10, 0, 0, 96, 253, 174, 2, 0, 0, 224, 5, 0, 0, 249, 255, 255, 47, 0, 0, 144, 1, 0, 32, 255, 255, 255, 191, 0, 0, 96, 51, 51, 131, 255, 255, 255, 255, 0, 0, 96, 255, 255, 255, 255, 255, 255, 239, 0, 0, 112, 255, 255, 255, 255, 255, 255, 175, 0, 0, 160, 255, 255, 255, 255, 255, 255, 29, 0, 0, 224, 255, 255, 255, 255, 255, 159, 0, 0, 0, 246, 255, 255, 255, 255, 207, 3, 0, 0, 48, 255, 255, 255, 255, 255, 11, 0, 0, 0, 228, 255, 255, 255, 255, 239, 0, 0, 0, 161, 255, 255, 255, 255, 255, 127, 0, 0, 64, 254, 255, 255, 255, 255, 255, 47, 0, 0, 242, 255, 255, 255, 255, 255, 255, 15, 0, 0, 249, 255, 255, 255, 255, 255, 255, 15, 0, 0, 252, 255, 255, 255, 255, 255, 255, 15, 0, 0, 252, 255, 255, 255, 255, 255, 255, 143, 136, 136, 254, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 76, 68, 68, 251, 255, 255, 255, 255, 255, 255, 11, 0, 0, 249, 255, 255, 255, 255, 255, 255, 11, 0, 0, 249, 255, 255, 255, 255, 255, 255, 11, 0, 0, 249, 255, 255, 255, 255, 255, 255, 11, 0, 0, 249, 255, 255, 255, 255, 255, 255, 11, 0, 0, 249, 255, 255, 255 ];

/// Array of references for 'TTSatoshi_DemiBold_42' normal ASCII glyphs
const Font_TTSatoshi_DemiBold_42: [&[u8]; 95] = [
    &Font_TTSatoshi_DemiBold_42_glyph_32,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_97,
    &Font_TTSatoshi_DemiBold_42_glyph_98,
    &Font_TTSatoshi_DemiBold_42_glyph_99,
    &Font_TTSatoshi_DemiBold_42_glyph_100,
    &Font_TTSatoshi_DemiBold_42_glyph_101,
    &Font_TTSatoshi_DemiBold_42_glyph_102,
    &Font_TTSatoshi_DemiBold_42_glyph_103,
    &Font_TTSatoshi_DemiBold_42_glyph_104,
    &Font_TTSatoshi_DemiBold_42_glyph_105,
    &Font_TTSatoshi_DemiBold_42_glyph_106,
    &Font_TTSatoshi_DemiBold_42_glyph_107,
    &Font_TTSatoshi_DemiBold_42_glyph_108,
    &Font_TTSatoshi_DemiBold_42_glyph_109,
    &Font_TTSatoshi_DemiBold_42_glyph_110,
    &Font_TTSatoshi_DemiBold_42_glyph_111,
    &Font_TTSatoshi_DemiBold_42_glyph_112,
    &Font_TTSatoshi_DemiBold_42_glyph_113,
    &Font_TTSatoshi_DemiBold_42_glyph_114,
    &Font_TTSatoshi_DemiBold_42_glyph_115,
    &Font_TTSatoshi_DemiBold_42_glyph_116,
    &Font_TTSatoshi_DemiBold_42_glyph_117,
    &Font_TTSatoshi_DemiBold_42_glyph_118,
    &Font_TTSatoshi_DemiBold_42_glyph_119,
    &Font_TTSatoshi_DemiBold_42_glyph_120,
    &Font_TTSatoshi_DemiBold_42_glyph_121,
    &Font_TTSatoshi_DemiBold_42_glyph_122,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_97,
    &Font_TTSatoshi_DemiBold_42_glyph_98,
    &Font_TTSatoshi_DemiBold_42_glyph_99,
    &Font_TTSatoshi_DemiBold_42_glyph_100,
    &Font_TTSatoshi_DemiBold_42_glyph_101,
    &Font_TTSatoshi_DemiBold_42_glyph_102,
    &Font_TTSatoshi_DemiBold_42_glyph_103,
    &Font_TTSatoshi_DemiBold_42_glyph_104,
    &Font_TTSatoshi_DemiBold_42_glyph_105,
    &Font_TTSatoshi_DemiBold_42_glyph_106,
    &Font_TTSatoshi_DemiBold_42_glyph_107,
    &Font_TTSatoshi_DemiBold_42_glyph_108,
    &Font_TTSatoshi_DemiBold_42_glyph_109,
    &Font_TTSatoshi_DemiBold_42_glyph_110,
    &Font_TTSatoshi_DemiBold_42_glyph_111,
    &Font_TTSatoshi_DemiBold_42_glyph_112,
    &Font_TTSatoshi_DemiBold_42_glyph_113,
    &Font_TTSatoshi_DemiBold_42_glyph_114,
    &Font_TTSatoshi_DemiBold_42_glyph_115,
    &Font_TTSatoshi_DemiBold_42_glyph_116,
    &Font_TTSatoshi_DemiBold_42_glyph_117,
    &Font_TTSatoshi_DemiBold_42_glyph_118,
    &Font_TTSatoshi_DemiBold_42_glyph_119,
    &Font_TTSatoshi_DemiBold_42_glyph_120,
    &Font_TTSatoshi_DemiBold_42_glyph_121,
    &Font_TTSatoshi_DemiBold_42_glyph_122,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
    &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
];

/// FontInfo struct for normal ASCII usage
pub const Font_TTSatoshi_DemiBold_42_info: FontInfo = FontInfo {
    translation_blob_idx: 1,
    height: 42,
    max_height: 44,
    baseline: 9,
    glyph_data: &Font_TTSatoshi_DemiBold_42,
    glyph_nonprintable: &Font_TTSatoshi_DemiBold_42_glyph_nonprintable,
};
