use atat::atat_derive::AtatResp;
use heapless::{String, Vec};

use crate::command::common::data::Integer;
use crate::enum_list;

use super::data::*;

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestDeleteSmsMessageResponse {
    #[at_arg(position = 0)]
    // TODO verify this is big enough
    pub indexes: Vec<Integer, 256>,
    #[at_arg(position = 1)]
    pub flags: enum_list!(DeleteFlag),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestSelectSmsMessageFormatResponse {
    #[at_arg(position = 0)]
    pub modes: enum_list!(SmsMessageFormat),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadSelectSmsMessageFormatResponse {
    #[at_arg(position = 0)]
    pub mode: SmsMessageFormat,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestListSmsMessagesTextResponse {
    #[at_arg(position = 0)]
    pub stats: enum_list!(ListSmsMessagesTextStat),
}

// TODO deal with all combinations of arguments
#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct WriteListSmsMessagesTextResponse {
    #[at_arg(position = 0)]
    pub index: Integer,
    #[at_arg(position = 1)]
    pub stat: ListSmsMessagesTextStat,
    #[at_arg(position = 2)]
    pub oa_da: String<128>,
    #[at_arg(position = 3)]
    pub alpha: Option<String<128>>,
    #[at_arg(position = 4)]
    pub scts: Option<String<128>>,
    #[at_arg(position = 5)]
    pub tooa_toda: Option<Integer>,
    #[at_arg(position = 6)]
    pub length: Option<Integer>,
    // TODO deal with <CR><LF>
    #[at_arg(position = 7)]
    pub data: String<128>,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestListSmsMessagesPduResponse {
    #[at_arg(position = 0)]
    pub stats: enum_list!(ListSmsMessagesStat),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct WriteListSmsMessagesPduResponse {
    #[at_arg(position = 0)]
    pub index: Integer,
    #[at_arg(position = 1)]
    pub stat: ListSmsMessagesStat,
    #[at_arg(position = 2)]
    pub alpha: Option<String<128>>,
    #[at_arg(position = 3)]
    pub length: Integer,
    // TODO deal with <CR><LF>
    #[at_arg(position = 4)]
    // TODO use PDU type
    pub pdu: String<128>,
    // TODO deal with <CR><LF>
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestReadSmsMessageResponse {}

// TODO deal with all combinations of arguments
#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct WriteReadSmsMessageTextResponse {
    #[at_arg(position = 0)]
    pub stat: ListSmsMessagesTextStat,
    #[at_arg(position = 1)]
    pub oa_da: String<128>,
    #[at_arg(position = 2)]
    pub alpha: Option<String<128>>,
    #[at_arg(position = 3)]
    pub scts: String<128>,
    #[at_arg(position = 4)]
    pub tooa_toda: Option<Integer>,
    #[at_arg(position = 5)]
    pub fo: Option<Integer>,
    #[at_arg(position = 6)]
    pub pid: Option<Integer>,
    #[at_arg(position = 7)]
    pub dcs: Option<Integer>,
    #[at_arg(position = 8)]
    pub vp: Option<String<128>>,
    #[at_arg(position = 9)]
    pub sca: Option<String<128>>,
    #[at_arg(position = 10)]
    pub tosca: Option<Integer>,
    #[at_arg(position = 11)]
    pub length: Option<Integer>,
    // TODO deal with <CR><LF>
    #[at_arg(position = 12)]
    pub data: String<128>,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct WriteReadSmsMessagePduResponse {
    #[at_arg(position = 0)]
    pub stat: ListSmsMessagesStat,
    #[at_arg(position = 1)]
    pub alpha: Option<String<128>>,
    #[at_arg(position = 2)]
    pub length: Integer,
    // TODO deal with <CR><LF>
    #[at_arg(position = 3)]
    // TODO use PDU type
    pub pdu: String<128>,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct WriteSendSmsMessageResponse {
    #[at_arg(position = 0)]
    pub message_reference: Integer,
}
