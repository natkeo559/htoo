use bitfield_struct::bitfield;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::flags::Flags;

/// A 32-bit bitfield storing a 24-bit length and 8 bits of reserved space.
/// 
/// The `length` field holds the frame size, while the remaining 8 bits (`_padding`) 
/// are unused or reserved for future use.
#[bitfield(u32)]
#[derive(PartialEq, Eq, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct FrameHeaderLength {
    #[bits(24)]
    pub length: u32,

    #[bits(8)]
    _padding: u8,
}

/// A 32-bit layout that reserves the first bit, with the remaining 31 bits used
/// for the stream identifier. The `_reserved` field is unused.
#[bitfield(u32)]
#[derive(PartialEq, Eq, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct StreamIdentifier {
    #[bits(1)]
    pub _reserved: u8,

    #[bits(31)]
    pub stream_identifier: u32,
}

/// A 32-bit bitfield where the first bit indicates whether the dependency is exclusive, 
/// and the remaining 31 bits store the stream identifier.
#[bitfield(u32)]
#[derive(PartialEq, Eq, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct StreamDependency {
    #[bits(1)]
    pub exclusive: bool,

    #[bits(31)]
    pub stream_identifier: u32,
}

/// A 32-bit bitfield with the first bit reserved, and the remaining 31 bits representing 
/// the window size.
#[bitfield(u32)]
#[derive(PartialEq, Eq, FromBytes, IntoBytes, KnownLayout, Immutable)]
pub struct WindowSizeIncrement {
    #[bits(1)]
    pub _reserved: u8,

    #[bits(31)]
    pub window_size: u32,
}

/// HTTP/2 error codes mapped to their 32-bit representation.
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

impl From<u32> for ErrorCode {
    fn from(value: u32) -> Self {
        match value {
            0x0 => Self::NO_ERROR,
            0x1 => Self::PROTOCOL_ERROR,
            0x2 => Self::INTERNAL_ERROR,
            0x3 => Self::FLOW_CONTROL_ERROR,
            0x4 => Self::SETTINGS_TIMEOUT,
            0x5 => Self::STREAM_CLOSED,
            0x6 => Self::FRAME_SIZE_ERROR,
            0x7 => Self::REFUSED_STREAM,
            0x8 => Self::CANCEL,
            0x9 => Self::COMPRESSION_ERROR,
            0xa => Self::CONNECT_ERROR,
            0xb => Self::ENHANCE_YOUR_CALM,
            0xc => Self::INADEQUATE_SECURITY,
            0xd => Self::HTTP_1_1_REQUIRED,
            v => Self::UNKNOWN(v),
        }
    }
}


/// Enumerates known HTTP/2 frame types as 8-bit values, including a variant for unknown types.
/// 
/// The default variant is `DATA` (`0x0`).
#[derive(Debug, Default, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum FrameType {
    #[default]
    /// RFC 7540: Hypertext Transfer Protocol Version 2
    DATA = 0x0,
    /// RFC 7540: Hypertext Transfer Protocol Version 2
    HEADERS = 0x1,
    /// RFC 7540: Hypertext Transfer Protocol Version 2
    PRIORITY = 0x2,
    /// RFC 7540: Hypertext Transfer Protocol Version 2
    RST_STREAM = 0x3,
    /// RFC 7540: Hypertext Transfer Protocol Version 2
    SETTINGS = 0x4,
    /// RFC 7540: Hypertext Transfer Protocol Version 2
    PUSH_PROMISE = 0x5,
    /// RFC 7540: Hypertext Transfer Protocol Version 2
    PING = 0x6,
    /// RFC 7540: Hypertext Transfer Protocol Version 2
    GOAWAY = 0x7,
    /// RFC 7540: Hypertext Transfer Protocol Version 2
    WINDOW_UPDATE = 0x8,
    /// RFC 7540: Hypertext Transfer Protocol Version 2
    CONTINUATION = 0x9,
    /// RFC 7838: HTTP Alternate Services
    ALTSVC = 0xa,
    /// RFC 8336: The ORIGIN HTTP/2 Frame
    ORIGIN = 0xc,
    /// Unknown Frame Type
    UNKNOWN(u8),
}

impl From<u8> for FrameType {
    fn from(value: u8) -> Self {
        match value {
            0x0 => Self::DATA,
            0x1 => Self::HEADERS,
            0x2 => Self::PRIORITY,
            0x3 => Self::RST_STREAM,
            0x4 => Self::SETTINGS,
            0x5 => Self::PUSH_PROMISE,
            0x6 => Self::PING,
            0x7 => Self::GOAWAY,
            0x8 => Self::WINDOW_UPDATE,
            0x9 => Self::CONTINUATION,
            0xa => Self::ALTSVC,
            0xc => Self::ORIGIN,
            value => Self::UNKNOWN(value),
        }
    }
}

/// Enumerates 16-bit HTTP/2 SETTINGS parameters, with a variant for reserved codes.
///
/// These parameters correspond to RFC 7540-defined values.
#[allow(non_camel_case_types)]
#[repr(u16)]
#[derive(Debug)]
pub enum SettingsParameter {
    SETTINGS_HEADER_TABLE_SIZE = 0x1,
    SETTINGS_ENABLE_PUSH = 0x2,
    SETTINGS_MAX_CONCURRENT_STREAMS = 0x3,
    SETTINGS_INITIAL_WINDOW_SIZE = 0x4,
    SETTINGS_MAX_FRAME_SIZE = 0x5,
    SETTINGS_MAX_HEADER_LIST_SIZE = 0x6,
    RESERVED(u16),
}

impl Default for SettingsParameter {
    fn default() -> Self {
        Self::RESERVED(0)
    }
}

impl From<u16> for SettingsParameter {
    fn from(value: u16) -> Self {
        match value {
            0x1 => Self::SETTINGS_HEADER_TABLE_SIZE,
            0x2 => Self::SETTINGS_ENABLE_PUSH,
            0x3 => Self::SETTINGS_MAX_CONCURRENT_STREAMS,
            0x4 => Self::SETTINGS_INITIAL_WINDOW_SIZE,
            0x5 => Self::SETTINGS_MAX_FRAME_SIZE,
            0x6 => Self::SETTINGS_MAX_HEADER_LIST_SIZE,
            v => Self::RESERVED(v),
        }
    }
}

/// An HTTP/2 DATA frame, containing optional padding and a payload.
pub struct DataFrame<'a> {
    pub pad_length: Option<u8>,
    pub data: &'a [u8],
    pub padding: Option<&'a [u8]>,
}

/// An HTTP/2 HEADERS frame, optionally including padding, stream dependency, and a weight.
pub struct HeadersFrame<'a> {
    pub pad_length: Option<u8>,
    pub stream_dependency: Option<StreamDependency>,
    pub weight: Option<u8>,
    pub header_block_fragment: &'a [u8],
    pub padding: Option<&'a [u8]>,
}

/// An HTTP/2 PRIORITY frame, indicating the stream dependency and weight.
pub struct PriorityFrame {
    pub stream_dependency: StreamDependency,
    pub weight: u8,
}

/// An HTTP/2 RST_STREAM frame, carrying an error code that explains why the stream is reset.
pub struct RstStreamFrame {
    pub error_code: ErrorCode,
}

/// Represents a single parameter-value pair in a SETTINGS frame.
pub struct SettingsParameterFrame {
    pub identifier: SettingsParameter,
    pub value: u32,
}

/// An HTTP/2 SETTINGS frame, containing zero or more parameter-value pairs.
pub struct SettingsFrame<'a> {
    pub parameters: Option<&'a [SettingsParameterFrame]>,
}

/// An HTTP/2 PING frame, carrying opaque data used to measure round-trip time or other diagnostics.
pub struct PingFrame {
    pub opaque_data: u64,
}

/// An HTTP/2 GOAWAY frame, indicating that no further streams can be initiated on this connection.
pub struct GoAwayFrame<'a> {
    pub last_stream_identifier: StreamIdentifier,
    pub error_code: ErrorCode,
    pub debug_data: Option<&'a [u8]>,
}

/// An HTTP/2 PUSH_PROMISE frame, which reserves a stream in advance of a request.
pub struct PushPromiseFrame<'a> {
    pub pad_length: Option<u8>,
    pub promised_stream_identifier: StreamIdentifier,
    pub header_block_fragment: &'a [u8],
    pub padding: Option<&'a [u8]>,
}

/// An HTTP/2 WINDOW_UPDATE frame, used to increase the flow-control window.
pub struct WindowUpdateFrame {
    pub window_size_increment: WindowSizeIncrement,
}

/// An HTTP/2 CONTINUATION frame, extending a header block begun by a previous HEADERS or PUSH_PROMISE.
pub struct ContinuationFrame<'a> {
    pub header_block_fragment: &'a [u8],
}

/// A single origin entry carried in an ORIGIN frame, containing its length and optional ASCII string.
pub struct OriginEntry<'a> {
    pub origin_length: u16,
    pub ascii_origin: Option<&'a str>
}

/// An HTTP/2 ORIGIN frame, optionally carrying a single origin entry.
pub struct OriginFrame<'a> {
    pub origin_entry: Option<OriginEntry<'a>>
}

/// The universal frame header for HTTP/2, containing length, type, flags, and stream ID.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct FrameHeader {
    pub length: FrameHeaderLength,
    pub frame_type: FrameType,
    pub flags: Flags,
    pub stream_identifier: StreamIdentifier,
}

/// A high-level representation of any HTTP/2 frame, combining a [`FrameHeader`] 
/// with the specific structure for that frame type.
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
