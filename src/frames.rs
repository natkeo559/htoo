use bitfield_struct::bitfield;

use crate::flags::Flags;

#[bitfield(u32)]
pub struct FrameHeaderLength {
    #[bits(24)]
    length: u32,

    #[bits(8)]
    __: u8,
}

#[bitfield(u32)]
pub struct StreamIdentifier {
    #[bits(1)]
    _reserved: u8,

    #[bits(31)]
    stream_identifier: u32,
}

#[bitfield(u32)]
pub struct StreamDependency {
    #[bits(1)]
    exclusive: bool,

    #[bits(31)]
    stream_identifier: u32,
}

#[bitfield(u32)]
pub struct WindowSizeIncrement {
    #[bits(1)]
    _reserved: u8,

    #[bits(31)]
    window_size: u32,
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum ErrorCode {
    /// **Code 0x0**
    ///
    /// The associated condition is not a result of an error. For example, a GOAWAY might include
    /// this code to indicate graceful shutdown of a connection.
    NO_ERROR = 0x0,

    /// **Code 0x1**
    ///
    /// The endpoint detected an unspecific protocol error. This error is for use when a more
    /// specific error code is not available.
    PROTOCOL_ERROR = 0x1,

    /// **Code 0x2**
    ///
    /// The endpoint encountered an unexpected internal error.
    INTERNAL_ERROR = 0x2,

    /// **Code 0x3**
    ///
    /// The endpoint detected that its peer violated the flow-control protocol.
    FLOW_CONTROL_ERROR = 0x3,

    /// **Code 0x4**
    ///
    /// The endpoint sent a SETTINGS frame but did not receive a response in a timely manner.
    SETTINGS_TIMEOUT = 0x4,

    /// **Code 0x5**
    ///
    /// The endpoint received a frame after a stream was half-closed.
    STREAM_CLOSED = 0x5,

    /// **Code 0x6**
    ///
    /// The endpoint received a frame with an invalid size.
    FRAME_SIZE_ERROR = 0x6,

    /// **Code 0x7**
    ///
    /// The endpoint refused the stream prior to performing any application processing.
    REFUSED_STREAM = 0x7,

    /// **Code 0x8**
    ///
    /// Used by the endpoint to indicate that the stream is no longer needed.
    CANCEL = 0x8,

    /// **Code 0x9**
    ///
    /// The endpoint is unable to maintain the header compression context for the connection.
    COMPRESSION_ERROR = 0x9,

    /// **Code 0xa**
    ///
    /// The connection established in response to a CONNECT request was reset or abnormally closed.
    CONNECT_ERROR = 0xa,

    /// **Code 0xb**
    ///
    /// The endpoint detected that its peer is exhibiting a behavior that might be generating
    /// excessive load.
    ENHANCE_YOUR_CALM = 0xb,

    /// **Code 0xc**
    ///
    /// The underlying transport has properties that do not meet minimum security requirements.
    INADEQUATE_SECURITY = 0xc,

    /// **Code 0xd**
    ///
    /// The endpoint requires that HTTP/1.1 be used instead of HTTP/2.
    HTTP_1_1_REQUIRED = 0xd,

    /// Unknown error code.
    UNKNOWN(u32),
}

#[repr(u8)]
pub enum FrameType {
    /// RFC 7540: Hypertext Transfer Protocol Version 2
    Data = 0x1,
    /// RFC 7540: Hypertext Transfer Protocol Version 2
    Headers = 0x2,
    /// RFC 7540: Hypertext Transfer Protocol Version 2
    Priority = 0x3,
    /// RFC 7540: Hypertext Transfer Protocol Version 2
    RstStream = 0x4,
    /// RFC 7540: Hypertext Transfer Protocol Version 2
    Settings = 0x5,
    /// RFC 7540: Hypertext Transfer Protocol Version 2
    PushPromise = 0x6,
    /// RFC 7540: Hypertext Transfer Protocol Version 2
    Ping = 0x7,
    /// RFC 7540: Hypertext Transfer Protocol Version 2
    GoAway = 0x8,
    /// RFC 7540: Hypertext Transfer Protocol Version 2
    WindowUpdate = 0x9,
    /// RFC 7540: Hypertext Transfer Protocol Version 2
    Continuation = 0xa,
    /// RFC 7838: HTTP Alternate Services
    AltSvc = 0xb,
    /// RFC 8336: The ORIGIN HTTP/2 Frame
    Origin = 0xc,
}

#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SettingsParameters {
    SETTINGS_HEADER_TABLE_SIZE = 0x1,
    SETTINGS_ENABLE_PUSH = 0x2,
    SETTINGS_MAX_CONCURRENT_STREAMS = 0x3,
    SETTINGS_INITIAL_WINDOW_SIZE = 0x4,
    SETTINGS_MAX_FRAME_SIZE = 0x5,
    SETTINGS_MAX_HEADER_LIST_SIZE = 0x6,
    RESERVED,
}

pub struct DataFrame<'a> {
    pub pad_length: Option<u8>,
    pub data: &'a [u8],
    pub padding: Option<&'a [u8]>,
}

pub struct HeadersFrame<'a> {
    pub pad_length: Option<u8>,
    pub stream_dependency: Option<StreamDependency>,
    pub weight: Option<u8>,
    pub header_block_fragment: &'a [u8],
    pub padding: Option<&'a [u8]>,
}

pub struct PriorityFrame {
    pub stream_dependency: StreamDependency,
    pub weight: u8,
}

pub struct RstStreamFrame {
    pub error_code: ErrorCode,
}

pub struct SettingsParameterFrame {
    pub identifier: u16,
    pub value: u32,
}

pub struct SettingsFrame<'a> {
    pub parameters: Option<&'a [SettingsParameterFrame]>,
}

pub struct PingFrame {
    pub opaque_data: u64,
}

pub struct GoAwayFrame<'a> {
    pub last_stream_identifier: StreamIdentifier,
    pub error_code: ErrorCode,
    pub debug_data: Option<&'a [u8]>,
}

pub struct PushPromiseFrame<'a> {
    pub pad_length: Option<u8>,
    pub promised_stream_identifier: StreamIdentifier,
    pub header_block_fragment: &'a [u8],
    pub padding: Option<&'a [u8]>,
}

pub struct WindowUpdateFrame {
    pub window_size_increment: WindowSizeIncrement,
}

pub struct ContinuationFrame<'a> {
    pub header_block_fragment: &'a [u8],
}

pub struct FrameHeader {
    pub length: FrameHeaderLength,
    pub frame_type: FrameType,
    pub flags: Flags,
    pub stream_identifier: StreamIdentifier,
}

pub enum Frame<'a> {
    Data(FrameHeader, DataFrame<'a>),
    Headers(FrameHeader, HeadersFrame<'a>),
    Priority(FrameHeader, PriorityFrame),
    RstStream(FrameHeader, RstStreamFrame),
    Settings(FrameHeader, SettingsFrame<'a>),
    PushPromise(FrameHeader, PushPromiseFrame<'a>),
    Ping(FrameHeader, PingFrame),
    GoAway(FrameHeader, GoAwayFrame<'a>),
    WindowUpdate(FrameHeader, WindowUpdateFrame),
    Continuation(FrameHeader, ContinuationFrame<'a>),
}

#[cfg(test)]
mod tests {
    use crate::flags::DataFlags;

    #[test]
    fn test_bitflags() {
        let flag = DataFlags::from_bits(1);

        assert_eq!(Some(DataFlags::END_STREAM), flag)
    }
}
