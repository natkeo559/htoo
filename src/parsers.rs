use nom::{
    IResult,
    bytes::complete::take,
    number::complete::{be_u8, be_u16, be_u24, be_u32},
};

use crate::{
    flags::Flags,
    frames::{
        DataFrame, ErrorCode, Frame, FrameHeader, FrameHeaderLength, FrameType, HeadersFrame,
        PriorityFrame, RstStreamFrame, SettingsFrame, SettingsParameter, SettingsParameterFrame,
        StreamDependency, StreamIdentifier,
    },
};

fn parse_optional_padding_length<'a>(
    bytes: &'a [u8],
    flags: &Flags,
) -> IResult<&'a [u8], Option<u8>, nom::error::Error<&'a [u8]>> {
    if flags.contains(Flags::PADDED) {
        let (bytes, pad_len) = be_u8(bytes)?;
        Ok((bytes, Some(pad_len)))
    } else {
        Ok((bytes, None))
    }
}

fn parse_optional_padding_bytes<'a>(
    bytes: &'a [u8],
    maybe_pad_len: Option<u8>,
) -> IResult<&'a [u8], Option<&'a [u8]>, nom::error::Error<&'a [u8]>> {
    if let Some(pl) = maybe_pad_len {
        let (bytes, p) = take(pl)(bytes)?;
        Ok((bytes, Some(p)))
    } else {
        Ok((bytes, None))
    }
}

fn parse_optional_stream_dependency<'a>(
    bytes: &'a [u8],
    flags: &Flags,
) -> IResult<&'a [u8], Option<StreamDependency>, nom::error::Error<&'a [u8]>> {
    if flags.contains(Flags::PRIORITY) {
        let (bytes, sd) = be_u32(bytes).map(|(b, i)| (b, StreamDependency::from_bits(i)))?;
        Ok((bytes, Some(sd)))
    } else {
        Ok((bytes, None))
    }
}

fn parse_stream_dependency<'a>(
    bytes: &'a [u8],
) -> IResult<&'a [u8], StreamDependency, nom::error::Error<&'a [u8]>> {
    Ok(be_u32(bytes).map(|(b, i)| (b, StreamDependency::from_bits(i)))?)
}

fn parse_optional_weight<'a>(
    bytes: &'a [u8],
    flags: &Flags,
) -> IResult<&'a [u8], Option<u8>, nom::error::Error<&'a [u8]>> {
    if flags.contains(Flags::PRIORITY) {
        let (bytes, sd) = be_u8(bytes)?;
        Ok((bytes, Some(sd)))
    } else {
        Ok((bytes, None))
    }
}

fn parse_weight<'a>(bytes: &'a [u8]) -> IResult<&'a [u8], u8, nom::error::Error<&'a [u8]>> {
    Ok(be_u8(bytes)?)
}

fn parse_payload<'a>(
    bytes: &'a [u8],
    length: u32,
) -> IResult<&'a [u8], &'a [u8], nom::error::Error<&'a [u8]>> {
    Ok(take(length)(bytes)?)
}

fn parse_settings_parameter_frame(
    bytes: &[u8],
) -> IResult<&[u8], SettingsParameterFrame, nom::error::Error<&[u8]>> {
    let (tail, bytes) = take(6usize)(bytes)?;
    let (bytes, identifier) = be_u16(bytes).map(|(b, i)| (b, SettingsParameter::from(i)))?;
    let (_bytes, value) = be_u32(bytes)?;

    Ok((tail, SettingsParameterFrame { identifier, value }))
}

impl FrameHeader {
    pub fn parse(bytes: &'_ [u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (tail, bytes) = take(9usize)(bytes)?;
        let (bytes, length) = be_u24(bytes).map(|(b, v)| (b, FrameHeaderLength::from_bits(v)))?;
        let (bytes, frame_type) = be_u8(bytes).map(|(b, v)| (b, FrameType::from(v)))?;
        let (bytes, flags) = be_u8(bytes).map(|(b, v)| (b, Flags::from(v)))?;
        let (_, stream_identifier) =
            be_u32(bytes).map(|(b, v)| (b, StreamIdentifier::from_bits(v)))?;

        Ok((tail, Self {
            length,
            frame_type,
            flags,
            stream_identifier,
        }))
    }
}

impl<'a> DataFrame<'a> {
    pub fn parse(
        bytes: &'a [u8],
        length: &FrameHeaderLength,
        flags: &Flags,
    ) -> IResult<&'a [u8], Self, nom::error::Error<&'a [u8]>> {
        let (bytes, maybe_pad_len) = parse_optional_padding_length(bytes, &flags)?;
        let pad_len = maybe_pad_len.unwrap_or(0) as u32;
        let adjusted_len = length.length().saturating_sub(pad_len);

        let (bytes, data_bytes) = parse_payload(bytes, adjusted_len)?;
        let (bytes, maybe_padding_bytes) = parse_optional_padding_bytes(bytes, maybe_pad_len)?;

        Ok((bytes, Self {
            pad_length: maybe_pad_len,
            data: data_bytes,
            padding: maybe_padding_bytes,
        }))
    }
}

impl<'a> HeadersFrame<'a> {
    pub fn parse(
        bytes: &'a [u8],
        length: &FrameHeaderLength,
        flags: &Flags,
    ) -> IResult<&'a [u8], Self, nom::error::Error<&'a [u8]>> {
        let (bytes, maybe_pad_len) = parse_optional_padding_length(bytes, &flags)?;
        let pad_len = maybe_pad_len.unwrap_or(0) as u32;
        let adjusted_len = length.length().saturating_sub(pad_len);
        let (bytes, maybe_stream_dependency) = parse_optional_stream_dependency(bytes, &flags)?;
        let (bytes, maybe_weight) = parse_optional_weight(bytes, &flags)?;
        let (bytes, header_block_fragment) = parse_payload(bytes, adjusted_len)?;
        let (bytes, maybe_padding_bytes) = parse_optional_padding_bytes(bytes, maybe_pad_len)?;

        Ok((bytes, Self {
            pad_length: maybe_pad_len,
            stream_dependency: maybe_stream_dependency,
            weight: maybe_weight,
            header_block_fragment,
            padding: maybe_padding_bytes,
        }))
    }
}

impl PriorityFrame {
    pub fn parse<'a>(bytes: &'a [u8]) -> IResult<&'a [u8], Self, nom::error::Error<&'a [u8]>> {
        let (tail, bytes) = take(5usize)(bytes)?;
        let (bytes, stream_dependency) = parse_stream_dependency(bytes)?;
        let (_bytes, weight) = parse_weight(bytes)?;

        Ok((tail, Self {
            stream_dependency,
            weight,
        }))
    }
}

impl RstStreamFrame {
    pub fn parse<'a>(bytes: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (bytes, err_code) = be_u32(bytes).map(|(b, v)| (b, ErrorCode::from(v)))?;
        Ok((bytes, Self {
            error_code: err_code,
        }))
    }
}

impl<'a> SettingsFrame<'a> {
    pub fn parse(
        bytes: &[u8],
        length: &FrameHeaderLength,
        flags: &Flags,
    ) -> IResult<&'a [u8], Self, nom::error::Error<&'a [u8]>> {
        let settings = match flags.contains(Flags::ACK) {
            true => {}
            false => {}
        };
    }
}

impl<'a> Frame<'a> {
    pub fn parse(bytes: &'a [u8]) -> () {
        let (bytes, frame_header) = FrameHeader::parse(bytes).unwrap();
        match frame_header.frame_type {
            FrameType::DATA => {
                DataFrame::parse(bytes, &frame_header.length, &frame_header.flags).unwrap();
            }
            FrameType::HEADERS => {
                HeadersFrame::parse(bytes, &frame_header.length, &frame_header.flags).unwrap();
            }
            FrameType::PRIORITY => {
                PriorityFrame::parse(bytes).unwrap();
            }
            FrameType::RST_STREAM => {
                RstStreamFrame::parse(bytes).unwrap();
            }
            FrameType::SETTINGS => todo!(),
            FrameType::PUSH_PROMISE => todo!(),
            FrameType::PING => todo!(),
            FrameType::GOAWAY => todo!(),
            FrameType::WINDOW_UPDATE => todo!(),
            FrameType::CONTINUATION => todo!(),
            FrameType::ALTSVC => todo!(),
            FrameType::ORIGIN => todo!(),
            FrameType::UNKNOWN(_) => todo!(),
        }
    }
}

#[cfg(test)]
mod parse_tests {

    use crate::{
        flags::Flags,
        frames::{FrameHeader, FrameHeaderLength, FrameType, StreamIdentifier},
    };

    #[test]
    fn test_parse_frame_header_from_bytes() {
        let header_0: [u8; 9] = [0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01];
        let header_1: [u8; 9] = [0x00, 0x01, 0x00, 0x01, 0xFF, 0x00, 0x00, 0x00, 0x02];
        let header_2: [u8; 10] = [0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00];
        let header_3: [u8; 8] = [0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00];

        let parsed_header_0 = FrameHeader::parse(&header_0).unwrap_or_default();
        let parsed_header_1 = FrameHeader::parse(&header_1).unwrap_or_default();
        let parsed_header_2 = FrameHeader::parse(&header_2).unwrap_or_default();
        let parsed_header_3 = FrameHeader::parse(&header_3);

        assert_eq!(
            FrameHeader {
                length: FrameHeaderLength::from_bits(16),
                frame_type: FrameType::DATA,
                flags: Flags::NONE,
                stream_identifier: StreamIdentifier::from_bits(1)
            },
            parsed_header_0.1
        );
        assert_eq!(true, parsed_header_0.0.is_empty());

        assert_eq!(
            FrameHeader {
                length: FrameHeaderLength::from_bits(256),
                frame_type: FrameType::HEADERS,
                flags: Flags(255),
                stream_identifier: StreamIdentifier::from_bits(2)
            },
            parsed_header_1.1
        );
        assert_eq!(true, parsed_header_1.0.is_empty());

        assert_eq!(
            FrameHeader {
                length: FrameHeaderLength::from_bits(16),
                frame_type: FrameType::DATA,
                flags: Flags::NONE,
                stream_identifier: StreamIdentifier::from_bits(1)
            },
            parsed_header_2.1
        );
        assert_eq!(false, parsed_header_2.0.is_empty());
        assert_eq!(true, parsed_header_3.is_err())
    }
}
