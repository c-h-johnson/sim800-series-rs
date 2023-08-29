use atat::{atat_derive::AtatEnum, AtatLen};
use enumscribe::{EnumDeserialize, EnumSerialize};

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum GprsAttachment {
    Detached = 0,
    Attached = 1,
}

#[derive(EnumSerialize, EnumDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum Level2Protocol {
    #[enumscribe(str = "PPP")]
    PointToPoint,
}

impl AtatLen for Level2Protocol {
    const LEN: usize = "PPP".len();
}

#[derive(EnumSerialize, EnumDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum GprsMobileClass {
    #[enumscribe(str = "B")]
    B,
    #[enumscribe(str = "CG")]
    CGprs,
    #[enumscribe(str = "CC")]
    CCircuitSwitched,
}

impl AtatLen for GprsMobileClass {
    const LEN: usize = "CG".len();
}

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Service {
    PacketDomain = 0,
    CircuitSwitched = 1,
    PacketDomainPreferred = 2,
    CircuitSwitchedPreferred = 3,
}
