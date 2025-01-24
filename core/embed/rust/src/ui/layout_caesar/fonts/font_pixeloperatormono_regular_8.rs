//! This file is generated by core/tools/codegen/gen_font.py
#![cfg_attr(any(), rustfmt::skip)]
#![allow(non_upper_case_globals)]
// Each glyph:
//   - first two bytes: width, height
//   - next three bytes: advance, bearingX, bearingY
//   - rest is packed 1-bit glyph data

use crate::ui::display::font::FontInfo;

// NOTE: hand-changed all the advances with 8 to 7
// NOTE: hand-changed the visuals of "m", "w", "M" and "W" to make it narrower


/// ' ' (ASCII 32)
const Font_PixelOperatorMono_Regular_8_glyph_32: [u8; 5] = [ 0, 0, 7, 0, 0 ];  // width hand-changed from 8 to 7 to have 9px space between words

/// '!' (ASCII 33)
const Font_PixelOperatorMono_Regular_8_glyph_33: [u8; 6] = [ 1, 7, 7, 2, 7, 250 ];

/// '"' (ASCII 34)
const Font_PixelOperatorMono_Regular_8_glyph_34: [u8; 7] = [ 3, 3, 7, 1, 7, 182, 128 ];

/// '#' (ASCII 35)
const Font_PixelOperatorMono_Regular_8_glyph_35: [u8; 10] = [ 6, 6, 7, 0, 6, 75, 244, 146, 253, 32 ];

/// '$' (ASCII 36)
const Font_PixelOperatorMono_Regular_8_glyph_36: [u8; 10] = [ 5, 7, 7, 0, 7, 35, 168, 226, 248, 128 ];

/// '%' (ASCII 37)
const Font_PixelOperatorMono_Regular_8_glyph_37: [u8; 12] = [ 7, 7, 7, 0, 7, 65, 74, 162, 162, 169, 65, 0 ];

/// '&' (ASCII 38)
const Font_PixelOperatorMono_Regular_8_glyph_38: [u8; 10] = [ 5, 7, 7, 0, 7, 116, 96, 232, 197, 224 ];

/// ''' (ASCII 39)
const Font_PixelOperatorMono_Regular_8_glyph_39: [u8; 6] = [ 1, 3, 7, 2, 7, 224 ];

/// '(' (ASCII 40)
const Font_PixelOperatorMono_Regular_8_glyph_40: [u8; 8] = [ 3, 7, 7, 2, 7, 42, 72, 136 ];

/// ')' (ASCII 41)
const Font_PixelOperatorMono_Regular_8_glyph_41: [u8; 8] = [ 3, 7, 7, 0, 7, 136, 146, 160 ];

/// '*' (ASCII 42)
const Font_PixelOperatorMono_Regular_8_glyph_42: [u8; 9] = [ 5, 5, 7, 0, 7, 37, 93, 82, 0 ];

/// '+' (ASCII 43)
const Font_PixelOperatorMono_Regular_8_glyph_43: [u8; 9] = [ 5, 5, 7, 0, 6, 33, 62, 66, 0 ];

/// ',' (ASCII 44)
const Font_PixelOperatorMono_Regular_8_glyph_44: [u8; 6] = [ 2, 3, 7, 1, 2, 88 ];

/// '-' (ASCII 45)
const Font_PixelOperatorMono_Regular_8_glyph_45: [u8; 6] = [ 4, 1, 7, 1, 4, 240 ];

/// '.' (ASCII 46)
const Font_PixelOperatorMono_Regular_8_glyph_46: [u8; 6] = [ 1, 1, 7, 2, 1, 128 ];

/// '/' (ASCII 47)
const Font_PixelOperatorMono_Regular_8_glyph_47: [u8; 8] = [ 3, 7, 7, 1, 7, 37, 37, 32 ];

/// '0' (ASCII 48)
const Font_PixelOperatorMono_Regular_8_glyph_48: [u8; 10] = [ 5, 7, 7, 0, 7, 116, 103, 92, 197, 192 ];

/// '1' (ASCII 49)
const Font_PixelOperatorMono_Regular_8_glyph_49: [u8; 10] = [ 5, 7, 7, 0, 7, 35, 40, 66, 19, 224 ];

/// '2' (ASCII 50)
const Font_PixelOperatorMono_Regular_8_glyph_50: [u8; 10] = [ 5, 7, 7, 0, 7, 116, 66, 34, 35, 224 ];

/// '3' (ASCII 51)
const Font_PixelOperatorMono_Regular_8_glyph_51: [u8; 10] = [ 5, 7, 7, 0, 7, 116, 66, 96, 197, 192 ];

/// '4' (ASCII 52)
const Font_PixelOperatorMono_Regular_8_glyph_52: [u8; 10] = [ 5, 7, 7, 0, 7, 25, 83, 31, 132, 32 ];

/// '5' (ASCII 53)
const Font_PixelOperatorMono_Regular_8_glyph_53: [u8; 10] = [ 5, 7, 7, 0, 7, 252, 60, 16, 197, 192 ];

/// '6' (ASCII 54)
const Font_PixelOperatorMono_Regular_8_glyph_54: [u8; 10] = [ 5, 7, 7, 0, 7, 116, 97, 232, 197, 192 ];

/// '7' (ASCII 55)
const Font_PixelOperatorMono_Regular_8_glyph_55: [u8; 10] = [ 5, 7, 7, 0, 7, 248, 68, 68, 66, 0 ];

/// '8' (ASCII 56)
const Font_PixelOperatorMono_Regular_8_glyph_56: [u8; 10] = [ 5, 7, 7, 0, 7, 116, 98, 232, 197, 192 ];

/// '9' (ASCII 57)
const Font_PixelOperatorMono_Regular_8_glyph_57: [u8; 10] = [ 5, 7, 7, 0, 7, 116, 98, 240, 197, 192 ];

/// ':' (ASCII 58)
const Font_PixelOperatorMono_Regular_8_glyph_58: [u8; 6] = [ 1, 5, 7, 2, 5, 136 ];

/// ';' (ASCII 59)
const Font_PixelOperatorMono_Regular_8_glyph_59: [u8; 7] = [ 2, 6, 7, 1, 5, 65, 96 ];

/// '<' (ASCII 60)
const Font_PixelOperatorMono_Regular_8_glyph_60: [u8; 7] = [ 3, 5, 7, 1, 6, 42, 34 ];

/// '=' (ASCII 61)
const Font_PixelOperatorMono_Regular_8_glyph_61: [u8; 7] = [ 4, 3, 7, 1, 5, 240, 240 ];

/// '>' (ASCII 62)
const Font_PixelOperatorMono_Regular_8_glyph_62: [u8; 7] = [ 3, 5, 7, 1, 6, 136, 168 ];

/// '?' (ASCII 63)
const Font_PixelOperatorMono_Regular_8_glyph_63: [u8; 10] = [ 5, 7, 7, 0, 7, 116, 66, 34, 0, 128 ];

/// '@' (ASCII 64)
const Font_PixelOperatorMono_Regular_8_glyph_64: [u8; 13] = [ 7, 8, 7, 0, 7, 125, 6, 109, 90, 179, 160, 62, 0 ];

/// 'A' (ASCII 65)
const Font_PixelOperatorMono_Regular_8_glyph_65: [u8; 10] = [ 5, 7, 7, 0, 7, 116, 99, 31, 198, 32 ];

/// 'B' (ASCII 66)
const Font_PixelOperatorMono_Regular_8_glyph_66: [u8; 10] = [ 5, 7, 7, 0, 7, 244, 99, 232, 199, 192 ];

/// 'C' (ASCII 67)
const Font_PixelOperatorMono_Regular_8_glyph_67: [u8; 10] = [ 5, 7, 7, 0, 7, 116, 97, 8, 69, 192 ];

/// 'D' (ASCII 68)
const Font_PixelOperatorMono_Regular_8_glyph_68: [u8; 10] = [ 5, 7, 7, 0, 7, 244, 99, 24, 199, 192 ];

/// 'E' (ASCII 69)
const Font_PixelOperatorMono_Regular_8_glyph_69: [u8; 10] = [ 5, 7, 7, 0, 7, 252, 33, 200, 67, 224 ];

/// 'F' (ASCII 70)
const Font_PixelOperatorMono_Regular_8_glyph_70: [u8; 10] = [ 5, 7, 7, 0, 7, 252, 33, 200, 66, 0 ];

/// 'G' (ASCII 71)
const Font_PixelOperatorMono_Regular_8_glyph_71: [u8; 10] = [ 5, 7, 7, 0, 7, 116, 97, 56, 197, 224 ];

/// 'H' (ASCII 72)
const Font_PixelOperatorMono_Regular_8_glyph_72: [u8; 10] = [ 5, 7, 7, 0, 7, 140, 99, 248, 198, 32 ];

/// 'I' (ASCII 73)
const Font_PixelOperatorMono_Regular_8_glyph_73: [u8; 10] = [ 5, 7, 7, 0, 7, 249, 8, 66, 19, 224 ];

/// 'J' (ASCII 74)
const Font_PixelOperatorMono_Regular_8_glyph_74: [u8; 11] = [ 6, 7, 7, 0, 7, 60, 32, 130, 10, 39, 0 ];

/// 'K' (ASCII 75)
const Font_PixelOperatorMono_Regular_8_glyph_75: [u8; 10] = [ 5, 7, 7, 0, 7, 140, 169, 138, 74, 32 ];

/// 'L' (ASCII 76)
const Font_PixelOperatorMono_Regular_8_glyph_76: [u8; 10] = [ 5, 7, 7, 0, 7, 132, 33, 8, 67, 224 ];

/// 'M' (ASCII 77)
const Font_PixelOperatorMono_Regular_8_glyph_77: [u8; 10] = [ 5, 7, 7, 0, 7, 142, 235, 24, 198, 32 ];

/// 'N' (ASCII 78)
const Font_PixelOperatorMono_Regular_8_glyph_78: [u8; 10] = [ 5, 7, 7, 0, 7, 140, 115, 89, 198, 32 ];

/// 'O' (ASCII 79)
const Font_PixelOperatorMono_Regular_8_glyph_79: [u8; 10] = [ 5, 7, 7, 0, 7, 116, 99, 24, 197, 192 ];

/// 'P' (ASCII 80)
const Font_PixelOperatorMono_Regular_8_glyph_80: [u8; 10] = [ 5, 7, 7, 0, 7, 244, 99, 31, 66, 0 ];

/// 'Q' (ASCII 81)
const Font_PixelOperatorMono_Regular_8_glyph_81: [u8; 10] = [ 5, 7, 7, 0, 7, 116, 99, 26, 201, 160 ];

/// 'R' (ASCII 82)
const Font_PixelOperatorMono_Regular_8_glyph_82: [u8; 10] = [ 5, 7, 7, 0, 7, 244, 99, 31, 74, 32 ];

/// 'S' (ASCII 83)
const Font_PixelOperatorMono_Regular_8_glyph_83: [u8; 10] = [ 5, 7, 7, 0, 7, 116, 96, 224, 197, 192 ];

/// 'T' (ASCII 84)
const Font_PixelOperatorMono_Regular_8_glyph_84: [u8; 10] = [ 5, 7, 7, 0, 7, 249, 8, 66, 16, 128 ];

/// 'U' (ASCII 85)
const Font_PixelOperatorMono_Regular_8_glyph_85: [u8; 10] = [ 5, 7, 7, 0, 7, 140, 99, 24, 197, 192 ];

/// 'V' (ASCII 86)
const Font_PixelOperatorMono_Regular_8_glyph_86: [u8; 10] = [ 5, 7, 7, 0, 7, 140, 99, 24, 168, 128 ];

/// 'W' (ASCII 87)
const Font_PixelOperatorMono_Regular_8_glyph_87: [u8; 10] = [ 5, 7, 7, 0, 7, 140, 107, 90, 213, 64 ];

/// 'X' (ASCII 88)
const Font_PixelOperatorMono_Regular_8_glyph_88: [u8; 10] = [ 5, 7, 7, 0, 7, 140, 84, 69, 70, 32 ];

/// 'Y' (ASCII 89)
const Font_PixelOperatorMono_Regular_8_glyph_89: [u8; 10] = [ 5, 7, 7, 0, 7, 140, 84, 66, 16, 128 ];

/// 'Z' (ASCII 90)
const Font_PixelOperatorMono_Regular_8_glyph_90: [u8; 10] = [ 5, 7, 7, 0, 7, 248, 68, 68, 67, 224 ];

/// '[' (ASCII 91)
const Font_PixelOperatorMono_Regular_8_glyph_91: [u8; 8] = [ 3, 7, 7, 2, 7, 242, 73, 56 ];

/// '\' (ASCII 92)
const Font_PixelOperatorMono_Regular_8_glyph_92: [u8; 8] = [ 3, 7, 7, 1, 7, 145, 36, 72 ];

/// ']' (ASCII 93)
const Font_PixelOperatorMono_Regular_8_glyph_93: [u8; 8] = [ 3, 7, 7, 0, 7, 228, 146, 120 ];

/// '^' (ASCII 94)
const Font_PixelOperatorMono_Regular_8_glyph_94: [u8; 7] = [ 5, 3, 7, 0, 7, 34, 162 ];

/// '_' (ASCII 95)
const Font_PixelOperatorMono_Regular_8_glyph_95: [u8; 6] = [ 7, 1, 7, 0, 0, 254 ];

/// '`' (ASCII 96)
const Font_PixelOperatorMono_Regular_8_glyph_96: [u8; 6] = [ 2, 2, 7, 1, 7, 144 ];

/// 'a' (ASCII 97)
const Font_PixelOperatorMono_Regular_8_glyph_97: [u8; 9] = [ 5, 5, 7, 0, 5, 112, 95, 23, 128 ];

/// 'b' (ASCII 98)
const Font_PixelOperatorMono_Regular_8_glyph_98: [u8; 10] = [ 5, 7, 7, 0, 7, 132, 61, 24, 199, 192 ];

/// 'c' (ASCII 99)
const Font_PixelOperatorMono_Regular_8_glyph_99: [u8; 9] = [ 5, 5, 7, 0, 5, 116, 97, 23, 0 ];

/// 'd' (ASCII 100)
const Font_PixelOperatorMono_Regular_8_glyph_100: [u8; 10] = [ 5, 7, 7, 0, 7, 8, 95, 24, 197, 224 ];

/// 'e' (ASCII 101)
const Font_PixelOperatorMono_Regular_8_glyph_101: [u8; 9] = [ 5, 5, 7, 0, 5, 116, 127, 7, 0 ];

/// 'f' (ASCII 102)
const Font_PixelOperatorMono_Regular_8_glyph_102: [u8; 9] = [ 4, 7, 7, 1, 7, 52, 244, 68, 64 ];

/// 'g' (ASCII 103)
const Font_PixelOperatorMono_Regular_8_glyph_103: [u8; 9] = [ 5, 6, 7, 0, 5, 124, 98, 240, 184 ];

/// 'h' (ASCII 104)
const Font_PixelOperatorMono_Regular_8_glyph_104: [u8; 10] = [ 5, 7, 7, 0, 7, 132, 61, 24, 198, 32 ];

/// 'i' (ASCII 105)
const Font_PixelOperatorMono_Regular_8_glyph_105: [u8; 10] = [ 5, 7, 7, 0, 7, 32, 56, 66, 19, 224 ];

/// 'j' (ASCII 106)
const Font_PixelOperatorMono_Regular_8_glyph_106: [u8; 10] = [ 5, 8, 7, 0, 7, 8, 14, 16, 134, 46 ];

/// 'k' (ASCII 107)
const Font_PixelOperatorMono_Regular_8_glyph_107: [u8; 10] = [ 5, 7, 7, 0, 7, 132, 37, 78, 74, 32 ];

/// 'l' (ASCII 108)
const Font_PixelOperatorMono_Regular_8_glyph_108: [u8; 10] = [ 5, 7, 7, 0, 7, 225, 8, 66, 19, 224 ];

/// 'm' (ASCII 109)
const Font_PixelOperatorMono_Regular_8_glyph_109: [u8; 9] = [ 5, 5, 7, 0, 5, 213, 107, 24, 128 ];

/// 'n' (ASCII 110)
const Font_PixelOperatorMono_Regular_8_glyph_110: [u8; 9] = [ 5, 5, 7, 0, 5, 244, 99, 24, 128 ];

/// 'o' (ASCII 111)
const Font_PixelOperatorMono_Regular_8_glyph_111: [u8; 9] = [ 5, 5, 7, 0, 5, 116, 99, 23, 0 ];

/// 'p' (ASCII 112)
const Font_PixelOperatorMono_Regular_8_glyph_112: [u8; 9] = [ 5, 6, 7, 0, 5, 244, 99, 232, 64 ];

/// 'q' (ASCII 113)
const Font_PixelOperatorMono_Regular_8_glyph_113: [u8; 9] = [ 5, 6, 7, 0, 5, 124, 98, 240, 132 ];

/// 'r' (ASCII 114)
const Font_PixelOperatorMono_Regular_8_glyph_114: [u8; 9] = [ 5, 5, 7, 0, 5, 157, 49, 8, 0 ];

/// 's' (ASCII 115)
const Font_PixelOperatorMono_Regular_8_glyph_115: [u8; 9] = [ 5, 5, 7, 0, 5, 116, 28, 31, 0 ];

/// 't' (ASCII 116)
const Font_PixelOperatorMono_Regular_8_glyph_116: [u8; 8] = [ 4, 6, 7, 1, 6, 79, 68, 67 ];

/// 'u' (ASCII 117)
const Font_PixelOperatorMono_Regular_8_glyph_117: [u8; 9] = [ 5, 5, 7, 0, 5, 140, 99, 23, 0 ];

/// 'v' (ASCII 118)
const Font_PixelOperatorMono_Regular_8_glyph_118: [u8; 9] = [ 5, 5, 7, 0, 5, 140, 98, 162, 0 ];

/// 'w' (ASCII 119)
const Font_PixelOperatorMono_Regular_8_glyph_119: [u8; 9] = [ 5, 5, 7, 0, 5, 140, 107, 85, 0 ];

/// 'x' (ASCII 120)
const Font_PixelOperatorMono_Regular_8_glyph_120: [u8; 9] = [ 5, 5, 7, 0, 5, 138, 136, 168, 128 ];

/// 'y' (ASCII 121)
const Font_PixelOperatorMono_Regular_8_glyph_121: [u8; 9] = [ 5, 6, 7, 0, 5, 140, 98, 240, 184 ];

/// 'z' (ASCII 122)
const Font_PixelOperatorMono_Regular_8_glyph_122: [u8; 9] = [ 5, 5, 7, 0, 5, 248, 136, 143, 128 ];

/// '{' (ASCII 123)
const Font_PixelOperatorMono_Regular_8_glyph_123: [u8; 9] = [ 4, 7, 7, 1, 7, 52, 72, 68, 48 ];

/// '|' (ASCII 124)
const Font_PixelOperatorMono_Regular_8_glyph_124: [u8; 6] = [ 1, 7, 7, 2, 7, 254 ];

/// '}' (ASCII 125)
const Font_PixelOperatorMono_Regular_8_glyph_125: [u8; 9] = [ 4, 7, 7, 0, 7, 194, 33, 34, 192 ];

/// '~' (ASCII 126)
const Font_PixelOperatorMono_Regular_8_glyph_126: [u8; 7] = [ 6, 2, 7, 0, 7, 102, 96 ];

/// Nonprintable glyph (inverse colors of '?')
const Font_PixelOperatorMono_Regular_8_glyph_nonprintable: [u8; 10] = [ 5, 7, 7, 0, 7, 139, 189, 221, 255, 127 ];

/// Array of references for 'PixelOperatorMono_Regular_8' normal ASCII glyphs
const Font_PixelOperatorMono_Regular_8: [&[u8]; 95] = [
    &Font_PixelOperatorMono_Regular_8_glyph_32,
    &Font_PixelOperatorMono_Regular_8_glyph_33,
    &Font_PixelOperatorMono_Regular_8_glyph_34,
    &Font_PixelOperatorMono_Regular_8_glyph_35,
    &Font_PixelOperatorMono_Regular_8_glyph_36,
    &Font_PixelOperatorMono_Regular_8_glyph_37,
    &Font_PixelOperatorMono_Regular_8_glyph_38,
    &Font_PixelOperatorMono_Regular_8_glyph_39,
    &Font_PixelOperatorMono_Regular_8_glyph_40,
    &Font_PixelOperatorMono_Regular_8_glyph_41,
    &Font_PixelOperatorMono_Regular_8_glyph_42,
    &Font_PixelOperatorMono_Regular_8_glyph_43,
    &Font_PixelOperatorMono_Regular_8_glyph_44,
    &Font_PixelOperatorMono_Regular_8_glyph_45,
    &Font_PixelOperatorMono_Regular_8_glyph_46,
    &Font_PixelOperatorMono_Regular_8_glyph_47,
    &Font_PixelOperatorMono_Regular_8_glyph_48,
    &Font_PixelOperatorMono_Regular_8_glyph_49,
    &Font_PixelOperatorMono_Regular_8_glyph_50,
    &Font_PixelOperatorMono_Regular_8_glyph_51,
    &Font_PixelOperatorMono_Regular_8_glyph_52,
    &Font_PixelOperatorMono_Regular_8_glyph_53,
    &Font_PixelOperatorMono_Regular_8_glyph_54,
    &Font_PixelOperatorMono_Regular_8_glyph_55,
    &Font_PixelOperatorMono_Regular_8_glyph_56,
    &Font_PixelOperatorMono_Regular_8_glyph_57,
    &Font_PixelOperatorMono_Regular_8_glyph_58,
    &Font_PixelOperatorMono_Regular_8_glyph_59,
    &Font_PixelOperatorMono_Regular_8_glyph_60,
    &Font_PixelOperatorMono_Regular_8_glyph_61,
    &Font_PixelOperatorMono_Regular_8_glyph_62,
    &Font_PixelOperatorMono_Regular_8_glyph_63,
    &Font_PixelOperatorMono_Regular_8_glyph_64,
    &Font_PixelOperatorMono_Regular_8_glyph_65,
    &Font_PixelOperatorMono_Regular_8_glyph_66,
    &Font_PixelOperatorMono_Regular_8_glyph_67,
    &Font_PixelOperatorMono_Regular_8_glyph_68,
    &Font_PixelOperatorMono_Regular_8_glyph_69,
    &Font_PixelOperatorMono_Regular_8_glyph_70,
    &Font_PixelOperatorMono_Regular_8_glyph_71,
    &Font_PixelOperatorMono_Regular_8_glyph_72,
    &Font_PixelOperatorMono_Regular_8_glyph_73,
    &Font_PixelOperatorMono_Regular_8_glyph_74,
    &Font_PixelOperatorMono_Regular_8_glyph_75,
    &Font_PixelOperatorMono_Regular_8_glyph_76,
    &Font_PixelOperatorMono_Regular_8_glyph_77,
    &Font_PixelOperatorMono_Regular_8_glyph_78,
    &Font_PixelOperatorMono_Regular_8_glyph_79,
    &Font_PixelOperatorMono_Regular_8_glyph_80,
    &Font_PixelOperatorMono_Regular_8_glyph_81,
    &Font_PixelOperatorMono_Regular_8_glyph_82,
    &Font_PixelOperatorMono_Regular_8_glyph_83,
    &Font_PixelOperatorMono_Regular_8_glyph_84,
    &Font_PixelOperatorMono_Regular_8_glyph_85,
    &Font_PixelOperatorMono_Regular_8_glyph_86,
    &Font_PixelOperatorMono_Regular_8_glyph_87,
    &Font_PixelOperatorMono_Regular_8_glyph_88,
    &Font_PixelOperatorMono_Regular_8_glyph_89,
    &Font_PixelOperatorMono_Regular_8_glyph_90,
    &Font_PixelOperatorMono_Regular_8_glyph_91,
    &Font_PixelOperatorMono_Regular_8_glyph_92,
    &Font_PixelOperatorMono_Regular_8_glyph_93,
    &Font_PixelOperatorMono_Regular_8_glyph_94,
    &Font_PixelOperatorMono_Regular_8_glyph_95,
    &Font_PixelOperatorMono_Regular_8_glyph_96,
    &Font_PixelOperatorMono_Regular_8_glyph_97,
    &Font_PixelOperatorMono_Regular_8_glyph_98,
    &Font_PixelOperatorMono_Regular_8_glyph_99,
    &Font_PixelOperatorMono_Regular_8_glyph_100,
    &Font_PixelOperatorMono_Regular_8_glyph_101,
    &Font_PixelOperatorMono_Regular_8_glyph_102,
    &Font_PixelOperatorMono_Regular_8_glyph_103,
    &Font_PixelOperatorMono_Regular_8_glyph_104,
    &Font_PixelOperatorMono_Regular_8_glyph_105,
    &Font_PixelOperatorMono_Regular_8_glyph_106,
    &Font_PixelOperatorMono_Regular_8_glyph_107,
    &Font_PixelOperatorMono_Regular_8_glyph_108,
    &Font_PixelOperatorMono_Regular_8_glyph_109,
    &Font_PixelOperatorMono_Regular_8_glyph_110,
    &Font_PixelOperatorMono_Regular_8_glyph_111,
    &Font_PixelOperatorMono_Regular_8_glyph_112,
    &Font_PixelOperatorMono_Regular_8_glyph_113,
    &Font_PixelOperatorMono_Regular_8_glyph_114,
    &Font_PixelOperatorMono_Regular_8_glyph_115,
    &Font_PixelOperatorMono_Regular_8_glyph_116,
    &Font_PixelOperatorMono_Regular_8_glyph_117,
    &Font_PixelOperatorMono_Regular_8_glyph_118,
    &Font_PixelOperatorMono_Regular_8_glyph_119,
    &Font_PixelOperatorMono_Regular_8_glyph_120,
    &Font_PixelOperatorMono_Regular_8_glyph_121,
    &Font_PixelOperatorMono_Regular_8_glyph_122,
    &Font_PixelOperatorMono_Regular_8_glyph_123,
    &Font_PixelOperatorMono_Regular_8_glyph_124,
    &Font_PixelOperatorMono_Regular_8_glyph_125,
    &Font_PixelOperatorMono_Regular_8_glyph_126,
];

/// FontInfo struct for normal ASCII usage
pub const Font_PixelOperatorMono_Regular_8_info: FontInfo = FontInfo {
    height: 8,
    max_height: 8,
    baseline: 1,
    glyph_data: &Font_PixelOperatorMono_Regular_8,
    glyph_nonprintable: &Font_PixelOperatorMono_Regular_8_glyph_nonprintable,
};
