use bitflags::bitflags;

bitflags! {
    /// Valid flags for a HEADERS frame
    #[derive(Default, Debug, PartialEq, Eq)]
    pub struct HeadersFlags: u8 {
        /// `0x01`: END_STREAM (for HEADERS)
        const END_STREAM  = 0x01;
        /// `0x04`: END_HEADERS
        const END_HEADERS = 0x04;
        /// `0x08`: PADDED
        const PADDED      = 0x08;
        /// `0x20`: PRIORITY
        const PRIORITY    = 0x20;
    }
}

bitflags! {
    /// Valid flags for a DATA frame
    #[derive(Default, Debug, PartialEq, Eq)]
    pub struct DataFlags: u8 {
        /// `0x01`: END_STREAM (for DATA)
        const END_STREAM = 0x01;
        /// `0x08`: PADDED
        const PADDED     = 0x08;
    }
}

bitflags! {
    /// Valid flags for a SETTINGS frame
    #[derive(Default, Debug, PartialEq, Eq)]
    pub struct SettingsFlags: u8 {
        /// `0x01`: ACK
        const ACK = 0x01;
    }
}

bitflags! {
    /// Valid flags for a PING frame
    #[derive(Default, Debug, PartialEq, Eq)]
    pub struct PingFlags: u8 {
        /// `0x01`: ACK
        const ACK = 0x01;
    }
}

bitflags! {
    /// Valid flags for a PUSH_PROMISE frame
    #[derive(Default, Debug, PartialEq, Eq)]
    pub struct PushPromiseFlags: u8 {
        /// `0x04`: END_HEADERS
        const END_HEADERS = 0x04;

        /// `0x08`: PADDED
        const PADDED      = 0x08;
    }
}

bitflags! {
    /// Valid flags for a CONTINUATION frame
    #[derive(Default, Debug, PartialEq, Eq)]
    pub struct ContinuationFlags: u8 {
        /// `0x04`: END_HEADERS
        const END_HEADERS = 0x04;
    }
}

bitflags! {
    /// Valid flags for a PRIORITY frame
    #[derive(Default, Debug, PartialEq, Eq)]
    pub struct PriorityFlags: u8 { }
}

bitflags! {
    /// Valid flags for a RST_STREAM frame
    #[derive(Default, Debug, PartialEq, Eq)]
    pub struct RstStreamFlags: u8 { }
}

bitflags! {
    /// Valid flags for a GOAWAY frame
    #[derive(Default, Debug, PartialEq, Eq)]
    pub struct GoAwayFlags: u8 { }
}

bitflags! {
    /// Valid flags for a WINDOW_UPDATE frame
    #[derive(Default, Debug, PartialEq, Eq)]
    pub struct WindowUpdateFlags: u8 { }
}

bitflags! {
    /// Unknown flag
    #[derive(Default, Debug, PartialEq, Eq)]
    pub struct UnknownFlags: u8 { }
}

#[derive(Debug)]
pub enum Flags {
    Data(DataFlags),
    Headers(HeadersFlags),
    Priority(PriorityFlags),
    RstStream(RstStreamFlags),
    Settings(SettingsFlags),
    PushPromise(PushPromiseFlags),
    Ping(PingFlags),
    GoAway(GoAwayFlags),
    WindowUpdate(WindowUpdateFlags),
    Continuation(ContinuationFlags),
    Unknown(UnknownFlags),
}
