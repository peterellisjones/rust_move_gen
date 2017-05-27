// subtraction-based sliding piece attacks
// NOT IMPLEMENTED
// https://chessprogramming.wikispaces.com/SBAMG
use bb::*;
use square::Square;

#[derive(Copy, Clone)]
pub struct LineMask {
    pub upper: BB,
    pub lower: BB,
}


#[allow(dead_code)]
pub fn rook_attacks_from_sq(from: Square, occupied: BB) -> BB {
    let masks = unsafe { *ROOK_LINE_MASKS.get_unchecked(from.to_usize()) };
    let file_attacks = line_attacks(occupied, masks[0]);
    let rank_attacks = line_attacks(occupied, masks[1]);
    file_attacks | rank_attacks
}

#[allow(dead_code)]
pub fn bishop_attacks_from_sq(from: Square, occupied: BB) -> BB {
    let masks = unsafe { *BISHOP_LINE_MASKS.get_unchecked(from.to_usize()) };
    let diag_attacks = line_attacks(occupied, masks[0]);
    let antidiag_attacks = line_attacks(occupied, masks[1]);
    diag_attacks | antidiag_attacks
}

fn line_attacks(occupied: BB, mask: LineMask) -> BB {
    let lower = mask.lower & occupied;
    let upper = mask.upper & occupied;

    let msb = (-BB(1)) << ((if lower == EMPTY { BB(1) } else { lower }).msb() as usize);
    let lsb = upper & -upper;
    let diff = BB(2) * lsb + msb;
    let attacks = diff & (mask.lower | mask.upper);
    attacks
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::testing::*;
    use test;

    #[test]
    fn t_rook_attacks() {
        test_rook_attacks_from_sq(rook_attacks_from_sq);
    }

    #[test]
    fn t_bishop_attacks() {
        test_bishop_attacks_from_sq(bishop_attacks_from_sq);
    }

    #[bench]
    fn bench_rook_attacks_from_sq(b: &mut test::Bencher) {
        bench_attacks_from_sq(b, rook_attacks_from_sq);
    }

    #[bench]
    fn bench_bishop_attacks_from_sq(b: &mut test::Bencher) {
        bench_attacks_from_sq(b, bishop_attacks_from_sq);
    }

    #[bench]
    fn bench_rook_attacks_from_sq_low_density(b: &mut test::Bencher) {
        bench_attacks_from_sq_low_density(b, rook_attacks_from_sq);
    }

    #[bench]
    fn bench_bishop_attacks_from_sq_low_density(b: &mut test::Bencher) {
        bench_attacks_from_sq_low_density(b, bishop_attacks_from_sq);
    }

    #[bench]
    fn bench_rook_attacks_from_sq_high_density(b: &mut test::Bencher) {
        bench_attacks_from_sq_high_density(b, rook_attacks_from_sq);
    }

    #[bench]
    fn bench_bishop_attacks_from_sq_high_density(b: &mut test::Bencher) {
        bench_attacks_from_sq_high_density(b, bishop_attacks_from_sq);
    }
}


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

