use atat::atat_derive::AtatEnum;

use crate::command::common::data::BoundedInteger;

pub type Rings = u8;
pub type Seconds = u8;
pub type Deciseconds = BoundedInteger<u8, 1, 254>;

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum ConnectResultCodeFormat {
    ResultCodeOnly = 0,
    ResultCodeOnlyText = 1,
    ResultCodeAndDialTone = 2,
    ResultCodeAndBusyDetection = 3,
    All = 4,
}

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum DcdOn {
    Always = 0,
    DataCarrierPresent = 1,
}

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum DtrMode {
    IgnoreStatus = 0,
    KeepConnectedCall = 1,
    DisconnectCall = 2,
}
