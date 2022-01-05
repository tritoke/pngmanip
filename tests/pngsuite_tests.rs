#![feature(assert_matches)]

use concat_idents::concat_idents;
use std::assert_matches::assert_matches;
use pngmanip::Image;
use pngmanip::ihdr_data::IhdrData;

macro_rules! create_test_image_parses {
    ($i:ident) => {
        concat_idents!(test_name = test_pngsuite_, $i, _parses {
            #[test]
            fn test_name() {
                let im_bytes = include_bytes!(concat!("../test_images/", stringify!($i), ".png"));
                let remaining = Image::parse(im_bytes).unwrap().0;
                assert!(remaining.is_empty());
            }
        });
    };
}

macro_rules! create_test_image_verifies {
    ($i:ident) => {
        concat_idents!(test_name = test_pngsuite_, $i, _verifies {
            #[test]
            fn test_name() {
                let im_bytes = include_bytes!(concat!("../test_images/", stringify!($i), ".png"));
                let parsed = Image::parse(im_bytes).unwrap().1;
                assert!(parsed.verify().is_ok());
            }
        });
    };
}

macro_rules! create_test_image_ihdr_verifies {
    ($i:ident) => {
        concat_idents!(test_name = test_pngsuite_, $i, _ihdr_verifies {
            #[test]
            fn test_name() {
                let im_bytes = include_bytes!(concat!("../test_images/", stringify!($i), ".png"));
                let parsed = Image::parse(im_bytes).unwrap().1;
                let ihdr_chunk = parsed.chunks.into_iter().find(|c| c.chunk_type == "IHDR").unwrap();
                let ihdr_data = IhdrData::parse(&ihdr_chunk.chunk_data).unwrap().1;
                assert!(ihdr_data.verify());
            }
        });
    };
}

macro_rules! create_test_image_ihdr_verify_fails {
    ($i:ident) => {
        concat_idents!(test_name = test_pngsuite_, $i, _ihdr_verify_fails {
            #[test]
            fn test_name() {
                let im_bytes = include_bytes!(concat!("../test_images/", stringify!($i), ".png"));
                let parsed = Image::parse(im_bytes).unwrap().1;
                let ihdr_chunk = parsed.chunks.into_iter().find(|c| c.chunk_type == "IHDR").unwrap();
                let ihdr_data = IhdrData::parse(&ihdr_chunk.chunk_data).unwrap().1;
                assert!(!ihdr_data.verify());
            }
        });
    };
}

macro_rules! create_test_image_verify_fails {
    ($i:ident, $err:pat) => {
        concat_idents!(test_name = test_pngsuite_, $i, _verifies {
            #[test]
            fn test_name() {
                let im_bytes = include_bytes!(concat!("../test_images/", stringify!($i), ".png"));
                let parsed = Image::parse(im_bytes).unwrap().1;
                assert_matches!(parsed.verify(), $err);
            }
        });
    };
}

macro_rules! create_test_image_serializes {
    ($i:ident) => {
        concat_idents!(test_name = test_pngsuite_, $i, _serializes {
            #[test]
            fn test_name() {
                let im_bytes = include_bytes!(concat!("../test_images/", stringify!($i), ".png"));
                let parsed = Image::parse(im_bytes).unwrap().1;
                let mut buf = Vec::new();
                cookie_factory::gen(parsed.serialize(), &mut buf).unwrap();
                assert_eq!(buf, im_bytes);
            }
        });
    };
}


macro_rules! create_tests_header_fails {
    ($i:ident) => {
        concat_idents!(test_name = test_pngsuite_, $i, _parse_fails {
            #[test]
            fn test_name() {
                let im_bytes = include_bytes!(concat!("../test_images/", stringify!($i), ".png"));
                let error = Image::parse(im_bytes).unwrap_err();
                assert_matches!(
                    error,
                    nom::Err::Error(
                        nom::error::Error {
                            code: nom::error::ErrorKind::Tag,
                            ..
                        }
                    )
                );
            }
        });
    };
}

macro_rules! create_standard_test_functions {
    ($($id:ident),+) => {
        $(
            create_test_image_parses!{$id}
            create_test_image_verifies!{$id}
            create_test_image_ihdr_verifies!{$id}
            create_test_image_serializes!{$id}
        )+
    };
}

macro_rules! create_invalid_ihdr_verify_test_functions {
    ($($id:ident),+) => {
        $(
            create_test_image_parses!{$id}
            create_test_image_verifies!{$id}
            create_test_image_ihdr_verify_fails!{$id}
            create_test_image_serializes!{$id}
        )+
    };
}

macro_rules! create_invalid_verify_test_functions {
    ($($id:ident => $err:pat),+) => {
        $(
            create_test_image_parses!{$id}
            create_test_image_verify_fails!{$id, $err}
            create_test_image_serializes!{$id}
        )+
    };
}

macro_rules! create_header_parse_fail_test_functions {
    ($($id:ident),+) => {
        $( create_tests_header_fails!{$id} )+
    };
}

create_standard_test_functions! {
    basi0g01, basi0g02, basi0g04, basi0g08, basi0g16, basi2c08, basi2c16,
    basi3p01, basi3p02, basi3p04, basi3p08, basi4a08, basi4a16, basi6a08,
    basi6a16, basn0g01, basn0g02, basn0g04, basn0g08, basn0g16, basn2c08,
    basn2c16, basn3p01, basn3p02, basn3p04, basn3p08, basn4a08, basn4a16,
    basn6a08, basn6a16, bgai4a08, bgai4a16, bgan6a08, bgan6a16, bgbn4a08,
    bggn4a16, bgwn6a08, bgyn6a16, ccwn2c08, ccwn3p08, cdfn2c08, cdhn2c08,
    cdsn2c08, cdun2c08, ch1n3p04, ch2n3p08, cm0n0g04, cm7n0g04, cm9n0g04,
    cs3n2c16, cs3n3p08, cs5n2c08, cs5n3p08, cs8n2c08, cs8n3p08, ct0n0g04,
    ct1n0g04, cten0g04, ctfn0g04, ctgn0g04, cthn0g04, ctjn0g04, ctzn0g04,
    exif2c08, f00n0g08, f00n2c08, f01n0g08, f01n2c08, f02n0g08, f02n2c08,
    f03n0g08, f03n2c08, f04n0g08, f04n2c08, f99n0g04, g03n0g16, g03n2c08,
    g03n3p04, g04n0g16, g04n2c08, g04n3p04, g05n0g16, g05n2c08, g05n3p04,
    g07n0g16, g07n2c08, g07n3p04, g10n0g16, g10n2c08, g10n3p04, g25n0g16,
    g25n2c08, g25n3p04, oi1n0g16, oi1n2c16, oi2n0g16, oi2n2c16, oi4n0g16,
    oi4n2c16, oi9n0g16, oi9n2c16, pp0n2c16, pp0n6a08, ps1n0g08, ps1n2c16,
    ps2n0g08, ps2n2c16, s01i3p01, s01n3p01, s02i3p01, s02n3p01, s03i3p01,
    s03n3p01, s04i3p01, s04n3p01, s05i3p02, s05n3p02, s06i3p02, s06n3p02,
    s07i3p02, s07n3p02, s08i3p02, s08n3p02, s09i3p02, s09n3p02, s32i3p04,
    s32n3p04, s33i3p04, s33n3p04, s34i3p04, s34n3p04, s35i3p04, s35n3p04,
    s36i3p04, s36n3p04, s37i3p04, s37n3p04, s38i3p04, s38n3p04, s39i3p04,
    s39n3p04, s40i3p04, s40n3p04, tbbn0g04, tbbn2c16, tbbn3p08, tbgn2c16,
    tbgn3p08, tbrn2c08, tbwn0g16, tbwn3p08, tbyn3p08, tm3n3p02, tp0n0g08,
    tp0n2c08, tp0n3p08, tp1n3p08, xdtn0g01, z00n2c08, z03n2c08, z06n2c08,
    z09n2c08
}

create_header_parse_fail_test_functions!{ xcrn0g04, xlfn0g04, xs1n0g01, xs2n0g01, xs4n0g01, xs7n0g01 }
create_invalid_verify_test_functions!{ xhdn0g08 => Err(0), xcsn0g01 => Err(2) }
create_invalid_ihdr_verify_test_functions!{ xc1n0g08, xc9n2c08, xd0n2c08, xd3n2c08, xd9n2c08 }
