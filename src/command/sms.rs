//! AT Commands According to 3GPP TS 27.005
//!
//! The 3GPP TS 27.005 commands are for performing SMS and CBS related
//! operations.

pub mod data;
pub mod response;
pub mod urc;

use atat::atat_derive::AtatCmd;

use data::*;
use response::*;

use crate::command::common::{data::Integer, response::NoResponse};

#[derive(Clone, AtatCmd)]
#[at_cmd("+CMGD=?", TestDeleteSmsMessageResponse)]
pub struct TestDeleteSmsMessage {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CMGD", NoResponse, timeout_ms = 25000)]
pub struct WriteDeleteSmsMessage {
    #[at_arg(position = 0)]
    pub index: Integer,
    #[at_arg(position = 1)]
    pub flag: Option<DeleteFlag>,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CMGF=?", TestSelectSmsMessageFormatResponse)]
pub struct TestSelectSmsMessageFormat {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CMGF?", ReadSelectSmsMessageFormatResponse)]
pub struct ReadSelectSmsMessageFormat {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CMGF", NoResponse)]
pub struct WriteSelectSmsMessageFormat {
    #[at_arg(position = 0)]
    pub mode: SmsMessageFormat,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CMGL=?", TestListSmsMessagesTextResponse)]
pub struct TestListSmsMessagesText {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CMGL", WriteListSmsMessagesTextResponse, timeout_ms = 20000)]
pub struct WriteListSmsMessagesText {
    #[at_arg(position = 0)]
    pub stat: ListSmsMessagesTextStat,
    #[at_arg(position = 1)]
    pub mode: Option<SmsMessageStatusMode>,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CMGL", WriteListSmsMessagesTextResponse, timeout_ms = 20000)]
pub struct ExecuteListSmsMessagesText {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CMGL=?", TestListSmsMessagesPduResponse)]
pub struct TestListSmsMessagesPdu {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CMGL", WriteListSmsMessagesPduResponse, timeout_ms = 20000)]
pub struct WriteListSmsMessagesPdu {
    #[at_arg(position = 0)]
    pub stat: ListSmsMessagesStat,
    #[at_arg(position = 1)]
    pub mode: Option<SmsMessageStatusMode>,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CMGL", WriteListSmsMessagesPduResponse, timeout_ms = 20000)]
pub struct ExecuteListSmsMessagesPdu {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CMGR=?", TestReadSmsMessageResponse)]
pub struct TestReadSmsMessage {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CMGR", WriteReadSmsMessageTextResponse, timeout_ms = 5000)]
pub struct WriteReadSmsTextMessage {
    #[at_arg(position = 0)]
    pub index: Integer,
    #[at_arg(position = 1)]
    pub mode: Option<SmsMessageStatusMode>,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CMGR", WriteReadSmsMessagePduResponse, timeout_ms = 5000)]
pub struct WriteReadSmsPduMessage {
    #[at_arg(position = 0)]
    pub index: Integer,
    #[at_arg(position = 1)]
    pub mode: Option<SmsMessageStatusMode>,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CMGS=?", NoResponse)]
pub struct TestSendSmsMessage {}

/// Split into 2 commands as described here: https://github.com/BlackbirdHQ/atat/issues/149#issuecomment-1540585123
#[derive(Clone, AtatCmd)]
#[at_cmd("+CMGS", NoResponse)]
pub struct WriteSendSmsMessage1 {
    #[at_arg(position = 0)]
    pub recipient: SmsRecipient,
}

#[derive(Clone, AtatCmd)]
#[at_cmd(
    "",
    WriteSendSmsMessageResponse,
    value_sep = false,
    cmd_prefix = "",
    termination = "\x1a",
    force_receive_state = true,
    timeout_ms = 60000,
    quote_escape_strings = false
)]
pub struct WriteSendSmsMessage2 {
    #[at_arg(position = 0)]
    pub message: SmsMessage,
}

// TODO: 4.2.6 AT+CMGW Write SMS Message to Memory
// TODO: 4.2.7 AT+CMSS Send SMS Message from Storage

// #[derive(Clone, AtatCmd)]
// #[at_cmd("+CNMI=?", TestNewSmsMessageIndicationsResponse)]
// pub struct TestNewSmsMessageIndications {}

// #[derive(Clone, AtatCmd)]
// #[at_cmd("+CNMI?", ReadNewSmsMessageIndicationsResponse)]
// pub struct ReadNewSmsMessageIndications {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CNMI", NoResponse)]
pub struct WriteNewSmsMessageIndications {
    #[at_arg(position = 0)]
    pub mode: NewSmsMessageIndicationsMode,
    #[at_arg(position = 1)]
    pub deliver: Option<NewSmsMessageIndicationsDeliver>,
    #[at_arg(position = 2)]
    pub cbm: Option<NewSmsMessageIndicationsCbm>,
    #[at_arg(position = 3)]
    pub status_reports: Option<NewSmsMessageIndicationsStatusReports>,
    #[at_arg(position = 4)]
    pub buffer: Option<NewSmsMessageIndicationsBuffer>,
}

// TODO: 4.2.9 AT+CPMS Preferred SMS Message Storage
// TODO: 4.2.10 AT+CRES Restore SMS Settings
// TODO: 4.2.11 AT+CSAS Save SMS Settings
// TODO: 4.2.12 AT+CSCA SMS Service Center Address
// TODO: 4.2.13 AT+CSCB Select Cell Broadcast SMS Messages
// TODO: 4.2.14 AT+CSDH Show SMS Text Mode Parameters
// TODO: 4.2.15 AT+CSMP Set SMS Text Mode Parameters
// TODO: 4.2.16 AT+CSMS Select Message Service
