use bb::*;
#[cfg(target_feature = "sse3")]
use dbb::DBB;

#[cfg(target_feature = "sse3")]
extern crate simd;

#[cfg(target_feature = "sse3")]
use self::simd::x86::sse2::*;

#[derive(Copy, Clone)]
pub struct LineMask {
    pub upper: BB,
    pub lower: BB,
}




pub const BISHOP_DIRECTIONS: [(u32, BB); 4] = [(9, BB(FILE_A.0 | ROW_1.0)), // up + right
                                               (7, BB(FILE_H.0 | ROW_1.0)), // up + left
                                               (64 - 9, BB(FILE_H.0 | ROW_8.0)), // down + left
                                               (64 - 7, BB(FILE_A.0 | ROW_8.0))]; // down + right

pub const ROOK_DIRECTIONS: [(u32, BB); 4] = [(1, FILE_A), // right
                                             (8, ROW_1), // up
                                             (64 - 1, FILE_H), // left
                                             (64 - 8, ROW_8)]; // down

// use square::Square;
// const KNIGHT_DIRECTIONS: [(u32, BB); 8] =
//     [(8 + 8 + 1, BB(FILE_A.0 | ROW_1.0 | ROW_2.0)), // up + up + right
//      (8 + 1 + 1, BB(FILE_A.0 | FILE_B.0 | ROW_1.0)), // up + right + right
//      ((-8 + 1 + 1) as u32, BB(FILE_A.0 | FILE_B.0 | ROW_8.0)), // down + right + right
//      ((-8 - 8 + 1) as u32, BB(FILE_A.0 | ROW_8.0 | ROW_7.0)), // down + down + right
//      ((-8 - 8 - 1) as u32, BB(FILE_H.0 | ROW_8.0 | ROW_7.0)), // down + down + left
//      ((-8 - 1 - 1) as u32, BB(FILE_H.0 | FILE_G.0 | ROW_8.0)), // down + left + left
//      (8 - 1 - 1, BB(FILE_H.0 | FILE_G.0 | ROW_1.0)), // up + left + left
//      (8 + 8 - 1, BB(FILE_H.0 | ROW_1.0 | ROW_2.0))]; // up + up + left

// The following static arrays are also included as consts below
// Using static muts turned out to have a performance overhead vs consts
// init_all is used to initialize the static arrays which can be then
// be used as consts
// static mut KING_MOVES_STATIC: [BB; 64] = [BB(0); 64];
// static mut KNIGHT_MOVES_STATIC: [BB; 64] = [BB(0); 64];
// #[cfg(not(target_feature = "sse3"))]
// static mut DIAGONALS_STATIC: [BB; 64] = [BB(0); 64];
// #[cfg(not(target_feature = "sse3"))]
// static mut ANTI_DIAGONALS_STATIC: [BB; 64] = [BB(0); 64];
// #[cfg(target_feature = "sse3")]
// static mut BOTH_DIAGONALS_STATIC: [DBB; 64] = [DBB(u64x2::new(0, 0)); 64];

// static mut BISHOP_LINE_MASKS_STATIC: [[LineMask; 2]; 64] = [[LineMask {
//     upper: EMPTY,
//     lower: EMPTY,
// }; 2]; 64];

// // file; rank
// static mut ROOK_LINE_MASKS_STATIC: [[LineMask; 2]; 64] = [[LineMask {
//     upper: EMPTY,
//     lower: EMPTY,
// }; 2]; 64];

// #[allow(dead_code)]
// fn init_all() {
//     init_diagonals();
//     init_rook_line_masks();
//     init_bishop_line_masks();
//     init_king_moves();
//     init_knight_moves();
// }

// fn init_diagonals() {
//     for i in 0..64 {
//         let from = Square(i);
//         let mut diag = EMPTY;
//         let mut anti_diag = EMPTY;
//         for (idx, &(shift, mask)) in BISHOP_DIRECTIONS.iter().enumerate() {
//             let mut targets = BB::new(from).rot_left(shift);
//             loop {
//                 if (targets & mask).any() {
//                     break;
//                 }
//                 if idx % 2 == 0 {
//                     diag |= targets;
//                 } else {
//                     anti_diag |= targets;
//                 }
//                 targets |= targets.rot_left(shift);
//             }
//         }

//         #[cfg(target_feature = "sse3")]
//         unsafe {
//             BOTH_DIAGONALS_STATIC[i] = DBB(u64x2::new(diag.0, anti_diag.0));
//         }

//         #[cfg(not(target_feature = "sse3"))]
//         unsafe {
//             DIAGONALS_STATIC[i] = diag;
//         }

//         #[cfg(not(target_feature = "sse3"))]
//         unsafe {
//             ANTI_DIAGONALS_STATIC[i] = anti_diag;
//         }
//     }
// }

// fn init_rook_line_masks() {
//     for i in 0..64 {
//         let from = Square(i);
//         let mut ret = [BB(0); 4];
//         for (idx, &(shift, mask)) in ROOK_DIRECTIONS.iter().enumerate() {
//             let mut targets = BB::new(from).rot_left(shift);
//             loop {
//                 if (targets & mask).any() {
//                     break;
//                 }
//                 ret[idx] |= targets;
//                 targets |= targets.rot_left(shift);
//             }
//         }

//         let file = LineMask {
//             upper: ret[0],
//             lower: ret[2],
//         };
//         let rank = LineMask {
//             upper: ret[1],
//             lower: ret[3],
//         };
//         unsafe {
//             ROOK_LINE_MASKS[i] = [file, rank];
//         }
//     }
// }

// fn init_bishop_line_masks() {
//     for i in 0..64 {
//         let from = Square(i);
//         let mut ret = [BB(0); 4];
//         for (idx, &(shift, mask)) in BISHOP_DIRECTIONS.iter().enumerate() {
//             let mut targets = BB::new(from).rot_left(shift);
//             loop {
//                 if (targets & mask).any() {
//                     break;
//                 }
//                 ret[idx] |= targets;
//                 targets |= targets.rot_left(shift);
//             }
//         }

//         let anti_diag = LineMask {
//             upper: ret[1],
//             lower: ret[3],
//         };

//         let diag = LineMask {
//             upper: ret[0],
//             lower: ret[2],
//         };

//         unsafe {
//             BISHOP_LINE_MASKS_STATIC[i] = [anti_diag, diag];
//         }
//     }
// }

// fn init_king_moves() {
//     for i in 0..64 {
//         let from = Square(i);
//         let mut ret = BB(0);
//         for &(shift, mask) in ROOK_DIRECTIONS.iter() {
//             ret |= BB::new(from).rot_left(shift) & !mask;
//         }
//         for &(shift, mask) in BISHOP_DIRECTIONS.iter() {
//             ret |= BB::new(from).rot_left(shift) & !mask;
//         }

//         unsafe {
//             KING_MOVES_STATIC[i] = ret;
//         }
//     }
// }

// fn init_knight_moves() {
//     for i in 0..64 {
//         let from = Square(i);
//         let mut ret = BB(0);
//         for &(shift, mask) in KNIGHT_DIRECTIONS.iter() {
//             ret |= BB::new(from).rot_left(shift) & !mask;
//         }

//         unsafe {
//             KNIGHT_MOVES_STATIC[i] = ret;
//         }
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::init_all;

//     #[test]
//     fn print_all_statics() {
//         init_all();

//         println!("{}", "pub const BISHOP_LINE_MASKS: [[LineMask; 2]; 64] = [");
//         unsafe {
//             for masks in super::BISHOP_LINE_MASKS.iter() {
//                 println!("{}", "[LineMask {");
//                 println!("upper: BB({}),", masks[0].upper.0);
//                 println!("lower: BB({}),", masks[0].lower.0);
//                 println!("{}", "},LineMask {");
//                 println!("upper: BB({}),", masks[1].upper.0);
//                 println!("lower: BB({}),", masks[1].lower.0);
//                 println!("{}", "}],");
//             }
//         }
//         println!("{}", "]");

//         println!("{}", "pub const ROOK_LINE_MASKS: [[LineMask; 2]; 64] = [");
//         unsafe {
//             for masks in super::ROOK_LINE_MASKS.iter() {
//                 println!("{}", "[LineMask {");
//                 println!("upper: BB({}),", masks[0].upper.0);
//                 println!("lower: BB({}),", masks[0].lower.0);
//                 println!("{}", "},LineMask {");
//                 println!("upper: BB({}),", masks[1].upper.0);
//                 println!("lower: BB({}),", masks[1].lower.0);
//                 println!("{}", "}],");
//             }
//         }
//         println!("{}", "]");

//         assert!(false);
//     }
// }

pub const KING_MOVES: [BB; 64] = [BB(0x0000000000000302u64),
                                  BB(0x0000000000000705u64),
                                  BB(0x0000000000000E0Au64),
                                  BB(0x0000000000001C14u64),
                                  BB(0x0000000000003828u64),
                                  BB(0x0000000000007050u64),
                                  BB(0x000000000000E0A0u64),
                                  BB(0x000000000000C040u64),
                                  BB(0x0000000000030203u64),
                                  BB(0x0000000000070507u64),
                                  BB(0x00000000000E0A0Eu64),
                                  BB(0x00000000001C141Cu64),
                                  BB(0x0000000000382838u64),
                                  BB(0x0000000000705070u64),
                                  BB(0x0000000000E0A0E0u64),
                                  BB(0x0000000000C040C0u64),
                                  BB(0x0000000003020300u64),
                                  BB(0x0000000007050700u64),
                                  BB(0x000000000E0A0E00u64),
                                  BB(0x000000001C141C00u64),
                                  BB(0x0000000038283800u64),
                                  BB(0x0000000070507000u64),
                                  BB(0x00000000E0A0E000u64),
                                  BB(0x00000000C040C000u64),
                                  BB(0x0000000302030000u64),
                                  BB(0x0000000705070000u64),
                                  BB(0x0000000E0A0E0000u64),
                                  BB(0x0000001C141C0000u64),
                                  BB(0x0000003828380000u64),
                                  BB(0x0000007050700000u64),
                                  BB(0x000000E0A0E00000u64),
                                  BB(0x000000C040C00000u64),
                                  BB(0x0000030203000000u64),
                                  BB(0x0000070507000000u64),
                                  BB(0x00000E0A0E000000u64),
                                  BB(0x00001C141C000000u64),
                                  BB(0x0000382838000000u64),
                                  BB(0x0000705070000000u64),
                                  BB(0x0000E0A0E0000000u64),
                                  BB(0x0000C040C0000000u64),
                                  BB(0x0003020300000000u64),
                                  BB(0x0007050700000000u64),
                                  BB(0x000E0A0E00000000u64),
                                  BB(0x001C141C00000000u64),
                                  BB(0x0038283800000000u64),
                                  BB(0x0070507000000000u64),
                                  BB(0x00E0A0E000000000u64),
                                  BB(0x00C040C000000000u64),
                                  BB(0x0302030000000000u64),
                                  BB(0x0705070000000000u64),
                                  BB(0x0E0A0E0000000000u64),
                                  BB(0x1C141C0000000000u64),
                                  BB(0x3828380000000000u64),
                                  BB(0x7050700000000000u64),
                                  BB(0xE0A0E00000000000u64),
                                  BB(0xC040C00000000000u64),
                                  BB(0x0203000000000000u64),
                                  BB(0x0507000000000000u64),
                                  BB(0x0A0E000000000000u64),
                                  BB(0x141C000000000000u64),
                                  BB(0x2838000000000000u64),
                                  BB(0x5070000000000000u64),
                                  BB(0xA0E0000000000000u64),
                                  BB(0x40C0000000000000u64)];

pub const KNIGHT_MOVES: [BB; 64] = [BB(0x0000000000020400u64),
                                    BB(0x0000000000050800u64),
                                    BB(0x00000000000A1100u64),
                                    BB(0x0000000000142200u64),
                                    BB(0x0000000000284400u64),
                                    BB(0x0000000000508800u64),
                                    BB(0x0000000000A01000u64),
                                    BB(0x0000000000402000u64),
                                    BB(0x0000000002040004u64),
                                    BB(0x0000000005080008u64),
                                    BB(0x000000000A110011u64),
                                    BB(0x0000000014220022u64),
                                    BB(0x0000000028440044u64),
                                    BB(0x0000000050880088u64),
                                    BB(0x00000000A0100010u64),
                                    BB(0x0000000040200020u64),
                                    BB(0x0000000204000402u64),
                                    BB(0x0000000508000805u64),
                                    BB(0x0000000A1100110Au64),
                                    BB(0x0000001422002214u64),
                                    BB(0x0000002844004428u64),
                                    BB(0x0000005088008850u64),
                                    BB(0x000000A0100010A0u64),
                                    BB(0x0000004020002040u64),
                                    BB(0x0000020400040200u64),
                                    BB(0x0000050800080500u64),
                                    BB(0x00000A1100110A00u64),
                                    BB(0x0000142200221400u64),
                                    BB(0x0000284400442800u64),
                                    BB(0x0000508800885000u64),
                                    BB(0x0000A0100010A000u64),
                                    BB(0x0000402000204000u64),
                                    BB(0x0002040004020000u64),
                                    BB(0x0005080008050000u64),
                                    BB(0x000A1100110A0000u64),
                                    BB(0x0014220022140000u64),
                                    BB(0x0028440044280000u64),
                                    BB(0x0050880088500000u64),
                                    BB(0x00A0100010A00000u64),
                                    BB(0x0040200020400000u64),
                                    BB(0x0204000402000000u64),
                                    BB(0x0508000805000000u64),
                                    BB(0x0A1100110A000000u64),
                                    BB(0x1422002214000000u64),
                                    BB(0x2844004428000000u64),
                                    BB(0x5088008850000000u64),
                                    BB(0xA0100010A0000000u64),
                                    BB(0x4020002040000000u64),
                                    BB(0x0400040200000000u64),
                                    BB(0x0800080500000000u64),
                                    BB(0x1100110A00000000u64),
                                    BB(0x2200221400000000u64),
                                    BB(0x4400442800000000u64),
                                    BB(0x8800885000000000u64),
                                    BB(0x100010A000000000u64),
                                    BB(0x2000204000000000u64),
                                    BB(0x0004020000000000u64),
                                    BB(0x0008050000000000u64),
                                    BB(0x00110A0000000000u64),
                                    BB(0x0022140000000000u64),
                                    BB(0x0044280000000000u64),
                                    BB(0x0088500000000000u64),
                                    BB(0x0010A00000000000u64),
                                    BB(0x0020400000000000u64)];

#[cfg(not(target_feature = "sse3"))]
pub const DIAGONALS: [BB; 64] = [BB(0),
                                 BB(256),
                                 BB(66048),
                                 BB(16909312),
                                 BB(4328785920),
                                 BB(1108169199616),
                                 BB(283691315109888),
                                 BB(72624976668147712),
                                 BB(2),
                                 BB(65540),
                                 BB(16908296),
                                 BB(4328783888),
                                 BB(1108169195552),
                                 BB(283691315101760),
                                 BB(72624976668131456),
                                 BB(145249953336262656),
                                 BB(516),
                                 BB(16778248),
                                 BB(4328523792),
                                 BB(1108168675360),
                                 BB(283691314061376),
                                 BB(72624976666050688),
                                 BB(145249953332101120),
                                 BB(290499906664136704),
                                 BB(132104),
                                 BB(4295231504),
                                 BB(1108102090784),
                                 BB(283691180892224),
                                 BB(72624976399712384),
                                 BB(145249952799424512),
                                 BB(290499905598783488),
                                 BB(580999811180789760),
                                 BB(33818640),
                                 BB(1099579265056),
                                 BB(283674135240768),
                                 BB(72624942308409472),
                                 BB(145249884616818688),
                                 BB(290499769233571840),
                                 BB(580999538450366464),
                                 BB(1161999072605765632),
                                 BB(8657571872),
                                 BB(281492291854400),
                                 BB(72620578621636736),
                                 BB(145241157243273216),
                                 BB(290482314486480896),
                                 BB(580964628956184576),
                                 BB(1161929253617401856),
                                 BB(2323857407723175936),
                                 BB(2216338399296),
                                 BB(72062026714726528),
                                 BB(144124053429452800),
                                 BB(288248106858840064),
                                 BB(576496213700902912),
                                 BB(1152992423106838528),
                                 BB(2305983746702049280),
                                 BB(4611686018427387904),
                                 BB(567382630219904),
                                 BB(1134765260439552),
                                 BB(2269530520813568),
                                 BB(4539061024849920),
                                 BB(9078117754732544),
                                 BB(18155135997837312),
                                 BB(36028797018963968),
                                 BB(0)];

// 8 * 64 = 512 bytes
#[cfg(not(target_feature = "sse3"))]
pub const ANTI_DIAGONALS: [BB; 64] = [BB(9241421688590303744),
                                      BB(36099303471055872),
                                      BB(141012904183808),
                                      BB(550831656960),
                                      BB(2151686144),
                                      BB(8404992),
                                      BB(32768),
                                      BB(0),
                                      BB(4620710844295151616),
                                      BB(9241421688590303233),
                                      BB(36099303471054850),
                                      BB(141012904181764),
                                      BB(550831652872),
                                      BB(2151677968),
                                      BB(8388640),
                                      BB(64),
                                      BB(2310355422147510272),
                                      BB(4620710844295020800),
                                      BB(9241421688590041601),
                                      BB(36099303470531586),
                                      BB(141012903135236),
                                      BB(550829559816),
                                      BB(2147491856),
                                      BB(16416),
                                      BB(1155177711056977920),
                                      BB(2310355422114021376),
                                      BB(4620710844228043008),
                                      BB(9241421688456086017),
                                      BB(36099303202620418),
                                      BB(141012367312900),
                                      BB(549757915144),
                                      BB(4202512),
                                      BB(577588851233521664),
                                      BB(1155177702483820544),
                                      BB(2310355404967706624),
                                      BB(4620710809935413504),
                                      BB(9241421619870827009),
                                      BB(36099166032102402),
                                      BB(140738026276868),
                                      BB(1075843080),
                                      BB(288793326105133056),
                                      BB(577586656505233408),
                                      BB(1155173313027244032),
                                      BB(2310346626054553600),
                                      BB(4620693252109107456),
                                      BB(9241386504218214913),
                                      BB(36028934726878210),
                                      BB(275415828484),
                                      BB(144115188075855872),
                                      BB(288231475663339520),
                                      BB(576462955621646336),
                                      BB(1152925911260069888),
                                      BB(2305851822520205312),
                                      BB(4611703645040410880),
                                      BB(9223407290080821761),
                                      BB(70506452091906),
                                      BB(0),
                                      BB(281474976710656),
                                      BB(564049465049088),
                                      BB(1128103225065472),
                                      BB(2256206466908160),
                                      BB(4512412933881856),
                                      BB(9024825867763968),
                                      BB(18049651735527937)];

// 8 * 64 = 512 bytes
#[cfg(target_feature = "sse3")]
pub const BOTH_DIAGONALS: [DBB; 64] = [DBB(u64x2::new(0, 9241421688590303744)),
                                       DBB(u64x2::new(256, 36099303471055872)),
                                       DBB(u64x2::new(66048, 141012904183808)),
                                       DBB(u64x2::new(16909312, 550831656960)),
                                       DBB(u64x2::new(4328785920, 2151686144)),
                                       DBB(u64x2::new(1108169199616, 8404992)),
                                       DBB(u64x2::new(283691315109888, 32768)),
                                       DBB(u64x2::new(72624976668147712, 0)),
                                       DBB(u64x2::new(2, 4620710844295151616)),
                                       DBB(u64x2::new(65540, 9241421688590303233)),
                                       DBB(u64x2::new(16908296, 36099303471054850)),
                                       DBB(u64x2::new(4328783888, 141012904181764)),
                                       DBB(u64x2::new(1108169195552, 550831652872)),
                                       DBB(u64x2::new(283691315101760, 2151677968)),
                                       DBB(u64x2::new(72624976668131456, 8388640)),
                                       DBB(u64x2::new(145249953336262656, 64)),
                                       DBB(u64x2::new(516, 2310355422147510272)),
                                       DBB(u64x2::new(16778248, 4620710844295020800)),
                                       DBB(u64x2::new(4328523792, 9241421688590041601)),
                                       DBB(u64x2::new(1108168675360, 36099303470531586)),
                                       DBB(u64x2::new(283691314061376, 141012903135236)),
                                       DBB(u64x2::new(72624976666050688, 550829559816)),
                                       DBB(u64x2::new(145249953332101120, 2147491856)),
                                       DBB(u64x2::new(290499906664136704, 16416)),
                                       DBB(u64x2::new(132104, 1155177711056977920)),
                                       DBB(u64x2::new(4295231504, 2310355422114021376)),
                                       DBB(u64x2::new(1108102090784, 4620710844228043008)),
                                       DBB(u64x2::new(283691180892224, 9241421688456086017)),
                                       DBB(u64x2::new(72624976399712384, 36099303202620418)),
                                       DBB(u64x2::new(145249952799424512, 141012367312900)),
                                       DBB(u64x2::new(290499905598783488, 549757915144)),
                                       DBB(u64x2::new(580999811180789760, 4202512)),
                                       DBB(u64x2::new(33818640, 577588851233521664)),
                                       DBB(u64x2::new(1099579265056, 1155177702483820544)),
                                       DBB(u64x2::new(283674135240768, 2310355404967706624)),
                                       DBB(u64x2::new(72624942308409472, 4620710809935413504)),
                                       DBB(u64x2::new(145249884616818688, 9241421619870827009)),
                                       DBB(u64x2::new(290499769233571840, 36099166032102402)),
                                       DBB(u64x2::new(580999538450366464, 140738026276868)),
                                       DBB(u64x2::new(1161999072605765632, 1075843080)),
                                       DBB(u64x2::new(8657571872, 288793326105133056)),
                                       DBB(u64x2::new(281492291854400, 577586656505233408)),
                                       DBB(u64x2::new(72620578621636736, 1155173313027244032)),
                                       DBB(u64x2::new(145241157243273216, 2310346626054553600)),
                                       DBB(u64x2::new(290482314486480896, 4620693252109107456)),
                                       DBB(u64x2::new(580964628956184576, 9241386504218214913)),
                                       DBB(u64x2::new(1161929253617401856, 36028934726878210)),
                                       DBB(u64x2::new(2323857407723175936, 275415828484)),
                                       DBB(u64x2::new(2216338399296, 144115188075855872)),
                                       DBB(u64x2::new(72062026714726528, 288231475663339520)),
                                       DBB(u64x2::new(144124053429452800, 576462955621646336)),
                                       DBB(u64x2::new(288248106858840064, 1152925911260069888)),
                                       DBB(u64x2::new(576496213700902912, 2305851822520205312)),
                                       DBB(u64x2::new(1152992423106838528, 4611703645040410880)),
                                       DBB(u64x2::new(2305983746702049280, 9223407290080821761)),
                                       DBB(u64x2::new(4611686018427387904, 70506452091906)),
                                       DBB(u64x2::new(567382630219904, 0)),
                                       DBB(u64x2::new(1134765260439552, 281474976710656)),
                                       DBB(u64x2::new(2269530520813568, 564049465049088)),
                                       DBB(u64x2::new(4539061024849920, 1128103225065472)),
                                       DBB(u64x2::new(9078117754732544, 2256206466908160)),
                                       DBB(u64x2::new(18155135997837312, 4512412933881856)),
                                       DBB(u64x2::new(36028797018963968, 9024825867763968)),
                                       DBB(u64x2::new(0, 18049651735527937))];

pub const BISHOP_LINE_MASKS: [[LineMask; 2]; 64] = [[LineMask {
                                                         upper: BB(0),
                                                         lower: BB(0),
                                                     },
                                                     LineMask {
                                                         upper: BB(9241421688590303744),
                                                         lower: BB(0),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(256),
                                                         lower: BB(0),
                                                     },
                                                     LineMask {
                                                         upper: BB(36099303471055872),
                                                         lower: BB(0),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(66048),
                                                         lower: BB(0),
                                                     },
                                                     LineMask {
                                                         upper: BB(141012904183808),
                                                         lower: BB(0),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(16909312),
                                                         lower: BB(0),
                                                     },
                                                     LineMask {
                                                         upper: BB(550831656960),
                                                         lower: BB(0),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(4328785920),
                                                         lower: BB(0),
                                                     },
                                                     LineMask {
                                                         upper: BB(2151686144),
                                                         lower: BB(0),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(1108169199616),
                                                         lower: BB(0),
                                                     },
                                                     LineMask {
                                                         upper: BB(8404992),
                                                         lower: BB(0),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(283691315109888),
                                                         lower: BB(0),
                                                     },
                                                     LineMask {
                                                         upper: BB(32768),
                                                         lower: BB(0),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(72624976668147712),
                                                         lower: BB(0),
                                                     },
                                                     LineMask {
                                                         upper: BB(0),
                                                         lower: BB(0),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(0),
                                                         lower: BB(2),
                                                     },
                                                     LineMask {
                                                         upper: BB(4620710844295151616),
                                                         lower: BB(0),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(65536),
                                                         lower: BB(4),
                                                     },
                                                     LineMask {
                                                         upper: BB(9241421688590303232),
                                                         lower: BB(1),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(16908288),
                                                         lower: BB(8),
                                                     },
                                                     LineMask {
                                                         upper: BB(36099303471054848),
                                                         lower: BB(2),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(4328783872),
                                                         lower: BB(16),
                                                     },
                                                     LineMask {
                                                         upper: BB(141012904181760),
                                                         lower: BB(4),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(1108169195520),
                                                         lower: BB(32),
                                                     },
                                                     LineMask {
                                                         upper: BB(550831652864),
                                                         lower: BB(8),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(283691315101696),
                                                         lower: BB(64),
                                                     },
                                                     LineMask {
                                                         upper: BB(2151677952),
                                                         lower: BB(16),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(72624976668131328),
                                                         lower: BB(128),
                                                     },
                                                     LineMask {
                                                         upper: BB(8388608),
                                                         lower: BB(32),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(145249953336262656),
                                                         lower: BB(0),
                                                     },
                                                     LineMask {
                                                         upper: BB(0),
                                                         lower: BB(64),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(0),
                                                         lower: BB(516),
                                                     },
                                                     LineMask {
                                                         upper: BB(2310355422147510272),
                                                         lower: BB(0),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(16777216),
                                                         lower: BB(1032),
                                                     },
                                                     LineMask {
                                                         upper: BB(4620710844295020544),
                                                         lower: BB(256),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(4328521728),
                                                         lower: BB(2064),
                                                     },
                                                     LineMask {
                                                         upper: BB(9241421688590041088),
                                                         lower: BB(513),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(1108168671232),
                                                         lower: BB(4128),
                                                     },
                                                     LineMask {
                                                         upper: BB(36099303470530560),
                                                         lower: BB(1026),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(283691314053120),
                                                         lower: BB(8256),
                                                     },
                                                     LineMask {
                                                         upper: BB(141012903133184),
                                                         lower: BB(2052),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(72624976666034176),
                                                         lower: BB(16512),
                                                     },
                                                     LineMask {
                                                         upper: BB(550829555712),
                                                         lower: BB(4104),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(145249953332068352),
                                                         lower: BB(32768),
                                                     },
                                                     LineMask {
                                                         upper: BB(2147483648),
                                                         lower: BB(8208),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(290499906664136704),
                                                         lower: BB(0),
                                                     },
                                                     LineMask {
                                                         upper: BB(0),
                                                         lower: BB(16416),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(0),
                                                         lower: BB(132104),
                                                     },
                                                     LineMask {
                                                         upper: BB(1155177711056977920),
                                                         lower: BB(0),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(4294967296),
                                                         lower: BB(264208),
                                                     },
                                                     LineMask {
                                                         upper: BB(2310355422113955840),
                                                         lower: BB(65536),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(1108101562368),
                                                         lower: BB(528416),
                                                     },
                                                     LineMask {
                                                         upper: BB(4620710844227911680),
                                                         lower: BB(131328),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(283691179835392),
                                                         lower: BB(1056832),
                                                     },
                                                     LineMask {
                                                         upper: BB(9241421688455823360),
                                                         lower: BB(262657),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(72624976397598720),
                                                         lower: BB(2113664),
                                                     },
                                                     LineMask {
                                                         upper: BB(36099303202095104),
                                                         lower: BB(525314),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(145249952795197440),
                                                         lower: BB(4227072),
                                                     },
                                                     LineMask {
                                                         upper: BB(141012366262272),
                                                         lower: BB(1050628),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(290499905590394880),
                                                         lower: BB(8388608),
                                                     },
                                                     LineMask {
                                                         upper: BB(549755813888),
                                                         lower: BB(2101256),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(580999811180789760),
                                                         lower: BB(0),
                                                     },
                                                     LineMask {
                                                         upper: BB(0),
                                                         lower: BB(4202512),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(0),
                                                         lower: BB(33818640),
                                                     },
                                                     LineMask {
                                                         upper: BB(577588851233521664),
                                                         lower: BB(0),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(1099511627776),
                                                         lower: BB(67637280),
                                                     },
                                                     LineMask {
                                                         upper: BB(1155177702467043328),
                                                         lower: BB(16777216),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(283673999966208),
                                                         lower: BB(135274560),
                                                     },
                                                     LineMask {
                                                         upper: BB(2310355404934086656),
                                                         lower: BB(33619968),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(72624942037860352),
                                                         lower: BB(270549120),
                                                     },
                                                     LineMask {
                                                         upper: BB(4620710809868173312),
                                                         lower: BB(67240192),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(145249884075720704),
                                                         lower: BB(541097984),
                                                     },
                                                     LineMask {
                                                         upper: BB(9241421619736346624),
                                                         lower: BB(134480385),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(290499768151441408),
                                                         lower: BB(1082130432),
                                                     },
                                                     LineMask {
                                                         upper: BB(36099165763141632),
                                                         lower: BB(268960770),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(580999536302882816),
                                                         lower: BB(2147483648),
                                                     },
                                                     LineMask {
                                                         upper: BB(140737488355328),
                                                         lower: BB(537921540),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(1161999072605765632),
                                                         lower: BB(0),
                                                     },
                                                     LineMask {
                                                         upper: BB(0),
                                                         lower: BB(1075843080),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(0),
                                                         lower: BB(8657571872),
                                                     },
                                                     LineMask {
                                                         upper: BB(288793326105133056),
                                                         lower: BB(0),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(281474976710656),
                                                         lower: BB(17315143744),
                                                     },
                                                     LineMask {
                                                         upper: BB(577586652210266112),
                                                         lower: BB(4294967296),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(72620543991349248),
                                                         lower: BB(34630287488),
                                                     },
                                                     LineMask {
                                                         upper: BB(1155173304420532224),
                                                         lower: BB(8606711808),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(145241087982698496),
                                                         lower: BB(69260574720),
                                                     },
                                                     LineMask {
                                                         upper: BB(2310346608841064448),
                                                         lower: BB(17213489152),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(290482175965396992),
                                                         lower: BB(138521083904),
                                                     },
                                                     LineMask {
                                                         upper: BB(4620693217682128896),
                                                         lower: BB(34426978560),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(580964351930793984),
                                                         lower: BB(277025390592),
                                                     },
                                                     LineMask {
                                                         upper: BB(9241386435364257792),
                                                         lower: BB(68853957121),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(1161928703861587968),
                                                         lower: BB(549755813888),
                                                     },
                                                     LineMask {
                                                         upper: BB(36028797018963968),
                                                         lower: BB(137707914242),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(2323857407723175936),
                                                         lower: BB(0),
                                                     },
                                                     LineMask {
                                                         upper: BB(0),
                                                         lower: BB(275415828484),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(0),
                                                         lower: BB(2216338399296),
                                                     },
                                                     LineMask {
                                                         upper: BB(144115188075855872),
                                                         lower: BB(0),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(72057594037927936),
                                                         lower: BB(4432676798592),
                                                     },
                                                     LineMask {
                                                         upper: BB(288230376151711744),
                                                         lower: BB(1099511627776),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(144115188075855872),
                                                         lower: BB(8865353596928),
                                                     },
                                                     LineMask {
                                                         upper: BB(576460752303423488),
                                                         lower: BB(2203318222848),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(288230376151711744),
                                                         lower: BB(17730707128320),
                                                     },
                                                     LineMask {
                                                         upper: BB(1152921504606846976),
                                                         lower: BB(4406653222912),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(576460752303423488),
                                                         lower: BB(35461397479424),
                                                     },
                                                     LineMask {
                                                         upper: BB(2305843009213693952),
                                                         lower: BB(8813306511360),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(1152921504606846976),
                                                         lower: BB(70918499991552),
                                                     },
                                                     LineMask {
                                                         upper: BB(4611686018427387904),
                                                         lower: BB(17626613022976),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(2305843009213693952),
                                                         lower: BB(140737488355328),
                                                     },
                                                     LineMask {
                                                         upper: BB(9223372036854775808),
                                                         lower: BB(35253226045953),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(4611686018427387904),
                                                         lower: BB(0),
                                                     },
                                                     LineMask {
                                                         upper: BB(0),
                                                         lower: BB(70506452091906),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(0),
                                                         lower: BB(567382630219904),
                                                     },
                                                     LineMask {
                                                         upper: BB(0),
                                                         lower: BB(0),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(0),
                                                         lower: BB(1134765260439552),
                                                     },
                                                     LineMask {
                                                         upper: BB(0),
                                                         lower: BB(281474976710656),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(0),
                                                         lower: BB(2269530520813568),
                                                     },
                                                     LineMask {
                                                         upper: BB(0),
                                                         lower: BB(564049465049088),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(0),
                                                         lower: BB(4539061024849920),
                                                     },
                                                     LineMask {
                                                         upper: BB(0),
                                                         lower: BB(1128103225065472),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(0),
                                                         lower: BB(9078117754732544),
                                                     },
                                                     LineMask {
                                                         upper: BB(0),
                                                         lower: BB(2256206466908160),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(0),
                                                         lower: BB(18155135997837312),
                                                     },
                                                     LineMask {
                                                         upper: BB(0),
                                                         lower: BB(4512412933881856),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(0),
                                                         lower: BB(36028797018963968),
                                                     },
                                                     LineMask {
                                                         upper: BB(0),
                                                         lower: BB(9024825867763968),
                                                     }],
                                                    [LineMask {
                                                         upper: BB(0),
                                                         lower: BB(0),
                                                     },
                                                     LineMask {
                                                         upper: BB(0),
                                                         lower: BB(18049651735527937),
                                                     }]];

pub const ROOK_LINE_MASKS: [[LineMask; 2]; 64] = [[LineMask {
                                                       upper: BB(254),
                                                       lower: BB(0),
                                                   },
                                                   LineMask {
                                                       upper: BB(72340172838076672),
                                                       lower: BB(0),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(252),
                                                       lower: BB(1),
                                                   },
                                                   LineMask {
                                                       upper: BB(144680345676153344),
                                                       lower: BB(0),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(248),
                                                       lower: BB(3),
                                                   },
                                                   LineMask {
                                                       upper: BB(289360691352306688),
                                                       lower: BB(0),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(240),
                                                       lower: BB(7),
                                                   },
                                                   LineMask {
                                                       upper: BB(578721382704613376),
                                                       lower: BB(0),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(224),
                                                       lower: BB(15),
                                                   },
                                                   LineMask {
                                                       upper: BB(1157442765409226752),
                                                       lower: BB(0),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(192),
                                                       lower: BB(31),
                                                   },
                                                   LineMask {
                                                       upper: BB(2314885530818453504),
                                                       lower: BB(0),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(128),
                                                       lower: BB(63),
                                                   },
                                                   LineMask {
                                                       upper: BB(4629771061636907008),
                                                       lower: BB(0),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(0),
                                                       lower: BB(127),
                                                   },
                                                   LineMask {
                                                       upper: BB(9259542123273814016),
                                                       lower: BB(0),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(65024),
                                                       lower: BB(0),
                                                   },
                                                   LineMask {
                                                       upper: BB(72340172838076416),
                                                       lower: BB(1),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(64512),
                                                       lower: BB(256),
                                                   },
                                                   LineMask {
                                                       upper: BB(144680345676152832),
                                                       lower: BB(2),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(63488),
                                                       lower: BB(768),
                                                   },
                                                   LineMask {
                                                       upper: BB(289360691352305664),
                                                       lower: BB(4),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(61440),
                                                       lower: BB(1792),
                                                   },
                                                   LineMask {
                                                       upper: BB(578721382704611328),
                                                       lower: BB(8),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(57344),
                                                       lower: BB(3840),
                                                   },
                                                   LineMask {
                                                       upper: BB(1157442765409222656),
                                                       lower: BB(16),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(49152),
                                                       lower: BB(7936),
                                                   },
                                                   LineMask {
                                                       upper: BB(2314885530818445312),
                                                       lower: BB(32),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(32768),
                                                       lower: BB(16128),
                                                   },
                                                   LineMask {
                                                       upper: BB(4629771061636890624),
                                                       lower: BB(64),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(0),
                                                       lower: BB(32512),
                                                   },
                                                   LineMask {
                                                       upper: BB(9259542123273781248),
                                                       lower: BB(128),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(16646144),
                                                       lower: BB(0),
                                                   },
                                                   LineMask {
                                                       upper: BB(72340172838010880),
                                                       lower: BB(257),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(16515072),
                                                       lower: BB(65536),
                                                   },
                                                   LineMask {
                                                       upper: BB(144680345676021760),
                                                       lower: BB(514),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(16252928),
                                                       lower: BB(196608),
                                                   },
                                                   LineMask {
                                                       upper: BB(289360691352043520),
                                                       lower: BB(1028),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(15728640),
                                                       lower: BB(458752),
                                                   },
                                                   LineMask {
                                                       upper: BB(578721382704087040),
                                                       lower: BB(2056),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(14680064),
                                                       lower: BB(983040),
                                                   },
                                                   LineMask {
                                                       upper: BB(1157442765408174080),
                                                       lower: BB(4112),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(12582912),
                                                       lower: BB(2031616),
                                                   },
                                                   LineMask {
                                                       upper: BB(2314885530816348160),
                                                       lower: BB(8224),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(8388608),
                                                       lower: BB(4128768),
                                                   },
                                                   LineMask {
                                                       upper: BB(4629771061632696320),
                                                       lower: BB(16448),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(0),
                                                       lower: BB(8323072),
                                                   },
                                                   LineMask {
                                                       upper: BB(9259542123265392640),
                                                       lower: BB(32896),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(4261412864),
                                                       lower: BB(0),
                                                   },
                                                   LineMask {
                                                       upper: BB(72340172821233664),
                                                       lower: BB(65793),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(4227858432),
                                                       lower: BB(16777216),
                                                   },
                                                   LineMask {
                                                       upper: BB(144680345642467328),
                                                       lower: BB(131586),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(4160749568),
                                                       lower: BB(50331648),
                                                   },
                                                   LineMask {
                                                       upper: BB(289360691284934656),
                                                       lower: BB(263172),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(4026531840),
                                                       lower: BB(117440512),
                                                   },
                                                   LineMask {
                                                       upper: BB(578721382569869312),
                                                       lower: BB(526344),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(3758096384),
                                                       lower: BB(251658240),
                                                   },
                                                   LineMask {
                                                       upper: BB(1157442765139738624),
                                                       lower: BB(1052688),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(3221225472),
                                                       lower: BB(520093696),
                                                   },
                                                   LineMask {
                                                       upper: BB(2314885530279477248),
                                                       lower: BB(2105376),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(2147483648),
                                                       lower: BB(1056964608),
                                                   },
                                                   LineMask {
                                                       upper: BB(4629771060558954496),
                                                       lower: BB(4210752),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(0),
                                                       lower: BB(2130706432),
                                                   },
                                                   LineMask {
                                                       upper: BB(9259542121117908992),
                                                       lower: BB(8421504),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(1090921693184),
                                                       lower: BB(0),
                                                   },
                                                   LineMask {
                                                       upper: BB(72340168526266368),
                                                       lower: BB(16843009),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(1082331758592),
                                                       lower: BB(4294967296),
                                                   },
                                                   LineMask {
                                                       upper: BB(144680337052532736),
                                                       lower: BB(33686018),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(1065151889408),
                                                       lower: BB(12884901888),
                                                   },
                                                   LineMask {
                                                       upper: BB(289360674105065472),
                                                       lower: BB(67372036),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(1030792151040),
                                                       lower: BB(30064771072),
                                                   },
                                                   LineMask {
                                                       upper: BB(578721348210130944),
                                                       lower: BB(134744072),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(962072674304),
                                                       lower: BB(64424509440),
                                                   },
                                                   LineMask {
                                                       upper: BB(1157442696420261888),
                                                       lower: BB(269488144),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(824633720832),
                                                       lower: BB(133143986176),
                                                   },
                                                   LineMask {
                                                       upper: BB(2314885392840523776),
                                                       lower: BB(538976288),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(549755813888),
                                                       lower: BB(270582939648),
                                                   },
                                                   LineMask {
                                                       upper: BB(4629770785681047552),
                                                       lower: BB(1077952576),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(0),
                                                       lower: BB(545460846592),
                                                   },
                                                   LineMask {
                                                       upper: BB(9259541571362095104),
                                                       lower: BB(2155905152),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(279275953455104),
                                                       lower: BB(0),
                                                   },
                                                   LineMask {
                                                       upper: BB(72339069014638592),
                                                       lower: BB(4311810305),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(277076930199552),
                                                       lower: BB(1099511627776),
                                                   },
                                                   LineMask {
                                                       upper: BB(144678138029277184),
                                                       lower: BB(8623620610),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(272678883688448),
                                                       lower: BB(3298534883328),
                                                   },
                                                   LineMask {
                                                       upper: BB(289356276058554368),
                                                       lower: BB(17247241220),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(263882790666240),
                                                       lower: BB(7696581394432),
                                                   },
                                                   LineMask {
                                                       upper: BB(578712552117108736),
                                                       lower: BB(34494482440),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(246290604621824),
                                                       lower: BB(16492674416640),
                                                   },
                                                   LineMask {
                                                       upper: BB(1157425104234217472),
                                                       lower: BB(68988964880),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(211106232532992),
                                                       lower: BB(34084860461056),
                                                   },
                                                   LineMask {
                                                       upper: BB(2314850208468434944),
                                                       lower: BB(137977929760),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(140737488355328),
                                                       lower: BB(69269232549888),
                                                   },
                                                   LineMask {
                                                       upper: BB(4629700416936869888),
                                                       lower: BB(275955859520),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(0),
                                                       lower: BB(139637976727552),
                                                   },
                                                   LineMask {
                                                       upper: BB(9259400833873739776),
                                                       lower: BB(551911719040),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(71494644084506624),
                                                       lower: BB(0),
                                                   },
                                                   LineMask {
                                                       upper: BB(72057594037927936),
                                                       lower: BB(1103823438081),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(70931694131085312),
                                                       lower: BB(281474976710656),
                                                   },
                                                   LineMask {
                                                       upper: BB(144115188075855872),
                                                       lower: BB(2207646876162),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(69805794224242688),
                                                       lower: BB(844424930131968),
                                                   },
                                                   LineMask {
                                                       upper: BB(288230376151711744),
                                                       lower: BB(4415293752324),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(67553994410557440),
                                                       lower: BB(1970324836974592),
                                                   },
                                                   LineMask {
                                                       upper: BB(576460752303423488),
                                                       lower: BB(8830587504648),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(63050394783186944),
                                                       lower: BB(4222124650659840),
                                                   },
                                                   LineMask {
                                                       upper: BB(1152921504606846976),
                                                       lower: BB(17661175009296),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(54043195528445952),
                                                       lower: BB(8725724278030336),
                                                   },
                                                   LineMask {
                                                       upper: BB(2305843009213693952),
                                                       lower: BB(35322350018592),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(36028797018963968),
                                                       lower: BB(17732923532771328),
                                                   },
                                                   LineMask {
                                                       upper: BB(4611686018427387904),
                                                       lower: BB(70644700037184),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(0),
                                                       lower: BB(35747322042253312),
                                                   },
                                                   LineMask {
                                                       upper: BB(9223372036854775808),
                                                       lower: BB(141289400074368),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(18302628885633695744),
                                                       lower: BB(0),
                                                   },
                                                   LineMask {
                                                       upper: BB(0),
                                                       lower: BB(282578800148737),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(18158513697557839872),
                                                       lower: BB(72057594037927936),
                                                   },
                                                   LineMask {
                                                       upper: BB(0),
                                                       lower: BB(565157600297474),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(17870283321406128128),
                                                       lower: BB(216172782113783808),
                                                   },
                                                   LineMask {
                                                       upper: BB(0),
                                                       lower: BB(1130315200594948),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(17293822569102704640),
                                                       lower: BB(504403158265495552),
                                                   },
                                                   LineMask {
                                                       upper: BB(0),
                                                       lower: BB(2260630401189896),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(16140901064495857664),
                                                       lower: BB(1080863910568919040),
                                                   },
                                                   LineMask {
                                                       upper: BB(0),
                                                       lower: BB(4521260802379792),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(13835058055282163712),
                                                       lower: BB(2233785415175766016),
                                                   },
                                                   LineMask {
                                                       upper: BB(0),
                                                       lower: BB(9042521604759584),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(9223372036854775808),
                                                       lower: BB(4539628424389459968),
                                                   },
                                                   LineMask {
                                                       upper: BB(0),
                                                       lower: BB(18085043209519168),
                                                   }],
                                                  [LineMask {
                                                       upper: BB(0),
                                                       lower: BB(9151314442816847872),
                                                   },
                                                   LineMask {
                                                       upper: BB(0),
                                                       lower: BB(36170086419038336),
                                                   }]];