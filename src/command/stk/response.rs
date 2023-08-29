use atat::atat_derive::AtatResp;
use heapless::{String, Vec};

use crate::command::common::data::Integer;

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestStkTerminalResponseResponse {
    #[at_arg(position = 0)]
    pub result_length: Integer,
    #[at_arg(position = 1)]
    pub text_length: Integer,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestStkEnvelopeCommandResponse {
    #[at_arg(position = 0)]
    pub command_length: Integer,
    #[at_arg(position = 1)]
    pub data_length: Integer,
}

// TODO: verify this works
#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadStkMainMenuCommandResponse {
    #[at_arg(position = 0)]
    pub stk_menus: Vec<(Integer, Integer, String<64>), 64>,
}
