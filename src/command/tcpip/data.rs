use atat::atat_derive::AtatEnum;
use heapless::String;

use crate::command::common::data::BoundedInteger;

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum IpConnection {
    Single = 0,
    Multi = 1,
}

pub type GprsString = String<64>;
pub type Seconds = BoundedInteger<u8, 1, 100>;

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum SendPrompt {
    SendOkOnly = 0,
    Both = 1,
    None = 2,
}

pub type Interval = BoundedInteger<u8, 1, 180>;
pub type Timer = BoundedInteger<u8, 1, 10>;

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum TcpipApplicationMode {
    Normal = 0,
    Transparent = 1,
}

pub type TimerSingle = BoundedInteger<u16, 100, 4000>;
pub type TimerMulti = BoundedInteger<u16, 100, 7000>;

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum GprsPdpContext {
    First = 0,
    Second = 1,
}
