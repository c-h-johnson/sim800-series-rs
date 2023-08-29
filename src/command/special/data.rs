use atat::{atat_derive::AtatEnum, AtatLen};
use enumscribe::{EnumDeserialize, EnumSerialize, ScribeStaticStr};
use heapless::String;
use serde::Serialize;

use crate::command::common::data::BoundedInteger;

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum AudioChannel {
    Main = 0,
    Aux = 1,
}

pub type Gain = BoundedInteger<u8, 0, 15>;

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum PowerOffMode {
    Urgent = 0,
    Normal = 1,
}

pub type Alarm = BoundedInteger<u8, 1, 5>;

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum AdcStatus {
    Fail = 0,
    Success = 1,
}

pub type AdcValue = BoundedInteger<u16, 0, 2800>;

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum SingleNumberingScheme {
    Voice = 0,
    Fax = 2,
    Data = 4,
}

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum AlternatingModeCalls {
    Default = 0,
}

pub type DtmfToneDuration = BoundedInteger<u8, 0, 100>;

#[derive(EnumSerialize, EnumDeserialize, Clone, PartialEq, Eq, Debug, ScribeStaticStr)]
pub enum Dtmf {
    #[enumscribe(str = "0")]
    _0,
    #[enumscribe(str = "1")]
    _1,
    #[enumscribe(str = "2")]
    _2,
    #[enumscribe(str = "3")]
    _3,
    #[enumscribe(str = "4")]
    _4,
    #[enumscribe(str = "5")]
    _5,
    #[enumscribe(str = "6")]
    _6,
    #[enumscribe(str = "7")]
    _7,
    #[enumscribe(str = "8")]
    _8,
    #[enumscribe(str = "9")]
    _9,
    #[enumscribe(str = "#")]
    Hashtag,
    #[enumscribe(str = "*")]
    Asterisk,
    #[enumscribe(str = "A")]
    A,
    #[enumscribe(str = "B")]
    B,
    #[enumscribe(str = "C")]
    C,
    #[enumscribe(str = "D")]
    D,
}

impl AtatLen for Dtmf {
    const LEN: usize = 1;
}

#[derive(Clone, Debug, Serialize)]
pub struct DtmfString(String<20>);

impl DtmfString {
    pub fn new(dtmfs: &[Dtmf]) -> Result<Self, ()> {
        let mut string = String::new();
        let mut first = true;
        for dtmf in dtmfs {
            if !first {
                string.push(',')?;
            } else {
                first = false;
            }
            string.push_str(dtmf.scribe())?
        }

        Ok(Self(string))
    }
}

impl AtatLen for DtmfString {
    const LEN: usize = 20;
}

// TODO: try the test command to determine what these lengths should be set to
pub type VoiceMailNumber = String<10>;
pub type AlphaString = String<64>;

#[derive(EnumSerialize, EnumDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum OperationBand {
    #[enumscribe(str = "EGSM_MODE")]
    Egsm,
    #[enumscribe(str = "PGSM_MODE")]
    Pgsm,
    #[enumscribe(str = "DCS_MODE")]
    Dcs,
    #[enumscribe(str = "GSM850_MODE")]
    Gsm850,
    #[enumscribe(str = "PCS_MODE")]
    Pcs,
    #[enumscribe(str = "EGSM_DCS_MODE")]
    EgsmDcs,
    #[enumscribe(str = "GSM850_PCS_MODE")]
    Gsm850Pcs,
    #[enumscribe(str = "EGSM_PCS_MODE")]
    EgsmPcs,
    #[enumscribe(str = "ALL_BAND")]
    All,
}

impl AtatLen for OperationBand {
    const LEN: usize = "GSM850_PCS_MODE".len();
}

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum SmsCodeMode {
    Nokia = 0,
    Siemens = 1,
}

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum SimInserted {
    NotInserted = 0,
    Inserted = 1,
}
