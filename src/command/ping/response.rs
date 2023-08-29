use atat::atat_derive::AtatResp;

use crate::{bounded_integer_list, enum_list};

use super::data::*;

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestPingRequestResponse {
    #[at_arg(position = 0)]
    pub retry: bounded_integer_list!(Retry),
    #[at_arg(position = 1)]
    pub data_length: bounded_integer_list!(DataLength),
    #[at_arg(position = 2)]
    pub timeout: bounded_integer_list!(Deciseconds),
    #[at_arg(position = 3)]
    pub time_to_live: bounded_integer_list!(TimeToLive),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadPingRequestResponse {
    #[at_arg(position = 0)]
    pub retry: Retry,
    #[at_arg(position = 1)]
    pub data_length: DataLength,
    #[at_arg(position = 2)]
    pub timeout: Deciseconds,
    #[at_arg(position = 3)]
    pub time_to_live: TimeToLive,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestSetReceivingIpPacketModeResponse {
    #[at_arg(position = 0)]
    pub modes: enum_list!(PacketEchoReplyMode),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadSetReceivingIpPacketModeResponse {
    #[at_arg(position = 0)]
    pub mode: PacketEchoReplyMode,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestSetIpFilterRulesResponse {
    #[at_arg(position = 0)]
    pub actions: enum_list!(IpFilterRulesAction),
    #[at_arg(position = 1)]
    pub items: bounded_integer_list!(IpFilterRuleItem),
}

// TODO: verify this can handle multiple responses
#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadSetIpFilterRulesResponse {
    #[at_arg(position = 0)]
    pub item: IpFilterRuleItem,
    #[at_arg(position = 1)]
    pub address: Address,
    #[at_arg(position = 2)]
    pub mask: Address,
}
