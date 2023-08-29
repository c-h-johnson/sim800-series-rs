//! AT Commands for SIM Application Toolkit
//!
//! Defined in GSM11.14

pub mod data;
pub mod response;

use atat::{atat_derive::AtatCmd, serde_at::HexStr};

use data::*;
use response::*;

use crate::command::common::response::NoResponse;

use super::common::{
    data::Enabled,
    response::{ReadEnabledResponse, TestEnabledResponse},
};

#[derive(Clone, AtatCmd)]
#[at_cmd("+STKTRS=?", TestStkTerminalResponseResponse)]
pub struct TestStkTerminalResponse {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+STKTRS?", NoResponse)]
pub struct ReadStkTerminalResponse {}

// TODO: verify this works
#[derive(Clone, AtatCmd)]
#[at_cmd("+STKTRS", NoResponse)]
pub struct WriteStkTerminalResponse {
    #[at_arg(position = 0)]
    pub result: HexStr<u8>,
    #[at_arg(position = 1)]
    pub text: Option<HexStr<u128>>,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+STKENVS=?", TestStkEnvelopeCommandResponse)]
pub struct TestStkEnvelopeCommand {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+STKENVS?", NoResponse)]
pub struct ReadStkEnvelopeCommand {}

// TODO: verify this works
#[derive(Clone, AtatCmd)]
#[at_cmd("+STKENVS", NoResponse)]
pub struct WriteStkEnvelopeCommand {
    #[at_arg(position = 0)]
    pub command: HexStr<u8>,
    #[at_arg(position = 1)]
    pub data: Option<HexStr<u8>>,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+STKCALL=?", NoResponse)]
pub struct TestStkCallSetup {}

// TODO: verify this works
#[derive(Clone, AtatCmd)]
#[at_cmd("+STKCALL", NoResponse)]
pub struct WriteStkCallSetup {
    #[at_arg(position = 0)]
    pub command: StkCommand,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+STKSMS=?", NoResponse)]
pub struct TestStkSmsDelivery {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+STKSMS", NoResponse)]
pub struct WriteStkSmsDelivery {
    #[at_arg(position = 0)]
    pub command: StkCommand,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+STKSS=?", NoResponse)]
pub struct TestStkSsSetup {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+STKSS", NoResponse)]
pub struct WriteStkSsSetup {
    #[at_arg(position = 0)]
    pub command: StkCommand,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+STKUSSD=?", NoResponse)]
pub struct TestStkUssdSetup {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+STKUSSD", NoResponse)]
pub struct WriteStkUssdSetup {
    #[at_arg(position = 0)]
    pub command: StkCommand,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+STKDTMF=?", NoResponse)]
pub struct TestStkSendingDtmf {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+STKDTMF", NoResponse)]
pub struct WriteStkSendingDtmf {
    #[at_arg(position = 0)]
    pub command: StkCommand,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+STKMENU=?", NoResponse)]
pub struct TestStkMainMenuCommand {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+STKMENU?", ReadStkMainMenuCommandResponse)]
pub struct ReadStkMainMenuCommand {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+STKPCIS=?", TestEnabledResponse)]
pub struct TestStkUrcSwitchCommand {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+STKPCIS?", ReadEnabledResponse)]
pub struct ReadStkUrcSwitchCommand {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+STKPCIS", NoResponse)]
pub struct WriteStkUrcSwitchCommand {
    #[at_arg(position = 0)]
    pub switch: Enabled,
}
