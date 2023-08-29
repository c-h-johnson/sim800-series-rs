//! AT Commands for PING Support

pub mod data;
pub mod response;

use atat::atat_derive::AtatCmd;

use data::*;
use response::*;

use crate::command::common::response::NoResponse;

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPPING=?", TestPingRequestResponse)]
pub struct TestPingRequest {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPPING?", ReadPingRequestResponse)]
pub struct ReadPingRequest {}

// TODO: handle response
#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPPING", NoResponse)]
pub struct WritePingRequest {
    #[at_arg(position = 0)]
    pub address: Address,
    #[at_arg(position = 1)]
    pub retry: Option<Retry>,
    #[at_arg(position = 2)]
    pub data_length: Option<DataLength>,
    #[at_arg(position = 3)]
    pub timeout: Option<Deciseconds>,
    #[at_arg(position = 4)]
    pub time_to_live: Option<TimeToLive>,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPCTL=?", TestSetReceivingIpPacketModeResponse)]
pub struct TestSetReceivingIpPacketMode {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPCTL?", ReadSetReceivingIpPacketModeResponse)]
pub struct ReadSetReceivingIpPacketMode {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPCTL", NoResponse)]
pub struct WriteSetReceivingIpPacketMode {
    #[at_arg(position = 0)]
    pub mode: PacketEchoReplyMode,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPFLT=?", TestSetIpFilterRulesResponse)]
pub struct TestSetIpFilterRules {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPFLT?", ReadSetIpFilterRulesResponse)]
pub struct ReadSetIpFilterRules {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPFLT", NoResponse)]
pub struct WriteSetIpFilterRules {
    #[at_arg(position = 0)]
    pub action: IpFilterRulesAction,
    #[at_arg(position = 1)]
    pub item: Option<IpFilterRuleItem>,
    #[at_arg(position = 2)]
    pub address: Option<Address>,
    #[at_arg(position = 3)]
    pub mask: Option<Address>,
}
