use atat::atat_derive::AtatEnum;
use heapless::String;

use crate::command::common::data::BoundedInteger;

pub type Address = String<64>;
pub type Retry = BoundedInteger<u8, 1, 100>;
pub type DataLength = BoundedInteger<u16, 0, 1024>;
pub type Deciseconds = BoundedInteger<u16, 1, 600>;
pub type TimeToLive = BoundedInteger<u8, 1, 255>;

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum PacketEchoReplyMode {
    Disable = 0,
    EnableEveryAddress = 1,
    EnableAddressSubset = 2,
}

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum IpFilterRulesAction {
    RemoveSpecified = 0,
    AddSpecified = 1,
    DeleteAll = 2,
}

pub type IpFilterRuleItem = BoundedInteger<u8, 1, 20>;
