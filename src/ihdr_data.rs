use cookie_factory::SerializeFn;
use nom::IResult;
use std::io::Write;
use log::Level::Debug;
use log::{log_enabled, debug};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IhdrData {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_type: ColorType,
    pub compression_method: CompressionMethod,
    pub filter_method: FilterMethod,
    pub interlace_method: InterlaceMethod,
}

impl IhdrData {
    pub fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        use nom::number::complete::{be_u32, be_u8};
        use nom::sequence::tuple;

        let (
            i,
            (
                width,
                height,
                bit_depth,
                color_type,
                compression_method,
                filter_method,
                interlace_method,
            ),
        ) = tuple((be_u32, be_u32, be_u8, be_u8, be_u8, be_u8, be_u8))(i)?;

        let parsed = Self {
            width,
            height,
            bit_depth,
            color_type: color_type.into(),
            compression_method: compression_method.into(),
            filter_method: filter_method.into(),
            interlace_method: interlace_method.into(),
        };


        if log_enabled!(Debug) {
            debug!("Parsed IHDR chunk data: {:?}", parsed);
        }

        Ok((i, parsed))
    }

    pub fn serialize<W: Write>(self) -> impl SerializeFn<W> {
        use cookie_factory::{
            bytes::{be_u32, be_u8},
            sequence::tuple,
        };

        tuple((
            be_u32(self.width),
            be_u32(self.height),
            be_u8(self.bit_depth),
            be_u8(self.color_type.into()),
            be_u8(self.compression_method.into()),
            be_u8(self.filter_method.into()),
            be_u8(self.interlace_method.into()),
        ))
    }

    pub fn verify(&self) -> bool {
        let width_valid = (1..=(1 << 31)).contains(&self.width);
        let height_valid = (1..=(1 << 31)).contains(&self.height);

        #[rustfmt::skip]
        let bit_depth_valid = match self.color_type {
            ColorType::GreyScale      => matches!(self.bit_depth, 1 | 2 | 4 | 8 | 16),
            ColorType::RGB            => matches!(self.bit_depth, 8 | 16),
            ColorType::Palette        => matches!(self.bit_depth, 1 | 2 | 4 | 8),
            ColorType::GreyScaleAlpha => matches!(self.bit_depth, 8 | 16),
            ColorType::RGBA           => matches!(self.bit_depth, 8 | 16),
            _                         => false,
        };

        let compression_method_valid = matches!(self.compression_method, CompressionMethod::Flate);
        let filter_method_valid = matches!(self.filter_method, FilterMethod::Adaptive);
        let interlace_method_valid = matches!(
            self.interlace_method,
            InterlaceMethod::NoInterlace | InterlaceMethod::Adam7
        );

        width_valid
            && height_valid
            && bit_depth_valid
            && compression_method_valid
            && filter_method_valid
            && interlace_method_valid
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ColorType {
    GreyScale = 0,
    RGB = 2,
    Palette = 3,
    GreyScaleAlpha = 4,
    RGBA = 6,
    Unrecognized(u8),
}

impl From<u8> for ColorType {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::GreyScale,
            2 => Self::RGB,
            3 => Self::Palette,
            4 => Self::GreyScaleAlpha,
            6 => Self::RGBA,
            x => Self::Unrecognized(x),
        }
    }
}

impl From<ColorType> for u8 {
    fn from(val: ColorType) -> u8 {
        match val {
            ColorType::GreyScale => 0,
            ColorType::RGB => 2,
            ColorType::Palette => 3,
            ColorType::GreyScaleAlpha => 4,
            ColorType::RGBA => 6,
            ColorType::Unrecognized(x) => x,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CompressionMethod {
    Flate = 0,
    Unrecognized(u8),
}

impl From<u8> for CompressionMethod {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Flate,
            x => Self::Unrecognized(x),
        }
    }
}

impl From<CompressionMethod> for u8 {
    fn from(val: CompressionMethod) -> u8 {
        match val {
            CompressionMethod::Flate => 0,
            CompressionMethod::Unrecognized(x) => x,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum FilterMethod {
    Adaptive = 0,
    Unrecognized(u8),
}

impl From<u8> for FilterMethod {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Adaptive,
            x => Self::Unrecognized(x),
        }
    }
}

impl From<FilterMethod> for u8 {
    fn from(val: FilterMethod) -> u8 {
        match val {
            FilterMethod::Adaptive => 0,
            FilterMethod::Unrecognized(x) => x,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum InterlaceMethod {
    NoInterlace = 0,
    Adam7 = 1,
    Unrecognized(u8),
}

impl From<u8> for InterlaceMethod {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::NoInterlace,
            1 => Self::Adam7,
            x => Self::Unrecognized(x),
        }
    }
}

impl From<InterlaceMethod> for u8 {
    fn from(val: InterlaceMethod) -> u8 {
        match val {
            InterlaceMethod::NoInterlace => 0,
            InterlaceMethod::Adam7 => 1,
            InterlaceMethod::Unrecognized(x) => x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cookie_factory::gen;

    const IHDR_DATA: &'static [u8] = &[
        0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x20, 0x01, 0x00, 0x00, 0x00, 0x01,
    ];

    const INVALID_IHDR_DATA: &'static [u8] = &[
        0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x20, 0x01, 0x0A, 0x00, 0x00, 0x01,
    ];

    #[test]
    fn test_color_type_conversion_for_all_u8() {
        for i in 0..=u8::MAX {
            let ct: ColorType = i.into();
            let val: u8 = ct.into();
            assert_eq!(i, val);
        }
    }

    #[test]
    fn test_compression_method_conversion_for_all_u8() {
        for i in 0..=u8::MAX {
            let ct: CompressionMethod = i.into();
            let val: u8 = ct.into();
            assert_eq!(i, val);
        }
    }

    #[test]
    fn test_filter_method_conversion_for_all_u8() {
        for i in 0..=u8::MAX {
            let ct: FilterMethod = i.into();
            let val: u8 = ct.into();
            assert_eq!(i, val);
        }
    }

    #[test]
    fn test_interlace_method_conversion_for_all_u8() {
        for i in 0..=u8::MAX {
            let ct: InterlaceMethod = i.into();
            let val: u8 = ct.into();
            assert_eq!(i, val);
        }
    }

    #[test]
    fn test_valid_ihdr_parse() {
        let parsed = IhdrData::parse(IHDR_DATA).unwrap().1;
        assert_eq!(
            parsed,
            IhdrData {
                width: 0x20,
                height: 0x20,
                bit_depth: 1.into(),
                color_type: 0.into(),
                compression_method: 0.into(),
                filter_method: 0.into(),
                interlace_method: 1.into(),
            }
        );
    }

    #[test]
    fn test_valid_ihdr_verify() {
        let parsed = IhdrData::parse(IHDR_DATA).unwrap().1;
        assert!(parsed.verify());
    }

    #[test]
    fn test_valid_ihdr_serializes() {
        let parsed = IhdrData::parse(IHDR_DATA).unwrap().1;
        let mut buf = Vec::new();
        gen(parsed.serialize(), &mut buf).unwrap();
        assert_eq!(buf, IHDR_DATA);
    }

    #[test]
    fn test_ihdr_parse_color_type_invalid() {
        let parsed = IhdrData::parse(INVALID_IHDR_DATA).unwrap().1;
        assert_eq!(
            parsed,
            IhdrData {
                width: 0x20,
                height: 0x20,
                bit_depth: 1.into(),
                color_type: 10.into(),
                compression_method: 0.into(),
                filter_method: 0.into(),
                interlace_method: 1.into(),
            }
        );
    }

    #[test]
    fn test_invalid_ihdr_doesnt_verify() {
        let parsed = IhdrData::parse(INVALID_IHDR_DATA).unwrap().1;
        assert!(!parsed.verify());
    }

    #[test]
    fn test_invalid_ihdr_serializes() {
        let parsed = IhdrData::parse(INVALID_IHDR_DATA).unwrap().1;
        let mut buf = Vec::new();
        gen(parsed.serialize(), &mut buf).unwrap();
        assert_eq!(buf, INVALID_IHDR_DATA);
    }
}
