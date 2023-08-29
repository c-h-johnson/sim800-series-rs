use atat::{atat_derive::AtatEnum, AtatLen};
use enumscribe::{EnumDeserialize, EnumSerialize};
use heapless::String;

pub const MAX_OPERATORS: usize = 128;

#[derive(EnumSerialize, EnumDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum CharacterSet {
    #[enumscribe(str = "GSM")]
    Gsm,
    #[enumscribe(str = "UCS2")]
    Ucs2,
    #[enumscribe(str = "IRA")]
    Ira,
    #[enumscribe(str = "HEX")]
    Hex,
    #[enumscribe(str = "PCCP")]
    Pccp,
    #[enumscribe(str = "PCDN")]
    Pcdn,
    #[enumscribe(str = "8859-1")]
    Iso,
}

impl AtatLen for CharacterSet {
    const LEN: usize = "8859-1".len();
}

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum OperatorStatus {
    Unknown = 0,
    Available = 1,
    Current = 2,
    Forbidden = 3,
}

pub type Operator = String<64>;
pub type NumericOperator = String<8>;

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum OperatorMode {
    Automatic = 0,
    Manual = 1,
    ManualDeregister = 2,
    SetOnlyFormat = 3,
    TryManual = 4,
}

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum OperatorFormat {
    LongAlphanumeric = 0,
    ShortAlphanumeric = 1,
    Numeric = 2,
}

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum NetworkRegistrationUrcMode {
    Disable = 0,
    Enable = 1,
    EnableWithLocation = 2,
}

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum NetworkRegistrationStatus {
    NotRegisteredOrSearching = 0,
    RegisteredHome = 1,
    NotRegistered = 2,
    Denied = 3,
    Unknown = 4,
    RegisteredRoaming = 5,
}

pub type LocationAreaCode = String<2>;
pub type CellId = LocationAreaCode;

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum ChargeStatus {
    NotCharging = 0,
    Charging = 1,
    ChargingFinished = 2,
}
