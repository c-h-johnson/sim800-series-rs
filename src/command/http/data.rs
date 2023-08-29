use atat::{atat_derive::AtatEnum, AtatLen};
use enumscribe::{EnumDeserialize, EnumSerialize};
use heapless::String;
use heapless_bytes::Bytes;

use crate::command::common::data::{BoundedInteger, Enabled, Integer};

#[derive(EnumSerialize, EnumDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum HttpParameterTag {
    #[enumscribe(str = "CID")]
    BearerProfileId,
    #[enumscribe(str = "URL")]
    ClientUrl,
    #[enumscribe(str = "UA")]
    UserAgent,
    #[enumscribe(str = "PROIP")]
    ProxyServerAddress,
    #[enumscribe(str = "PROPORT")]
    ProxyServerPort,
    #[enumscribe(str = "REDIR")]
    Redirection,
    #[enumscribe(str = "BREAK")]
    Break,
    #[enumscribe(str = "BREAKEND")]
    BreakEnd,
    #[enumscribe(str = "TIMEOUT")]
    Timeout,
    #[enumscribe(str = "CONTENT")]
    ContentType,
}

impl AtatLen for HttpParameterTag {
    const LEN: usize = "BREAKEND".len();
}

// TODO: verify that this is correct
#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
pub enum HttpParameterValue {
    BearerProfileId(String<64>),
    ClientUrl(String<64>),
    UserAgent(String<64>),
    ProxyServerAddress(String<64>),
    ProxyServerPort(Integer),
    Redirection(Enabled),
    Break(Integer),
    BreakEnd(Integer),
    Timeout(BoundedInteger<u16, 30, 1000>),
    ContentType(String<64>),
}

pub type Size = BoundedInteger<u32, 0, 319488>;
pub type Milliseconds = BoundedInteger<u32, 1000, 120000>;

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum HttpMethod {
    Get = 0,
    Post = 1,
    Head = 2,
}

pub type DataLength = BoundedInteger<u32, 1, 319488>;
// TODO: investigate how to make this bigger without running out of memory
pub type HttpData = Bytes<26400>;
