use atat::atat_derive::AtatResp;
use atat::serde_at::HexStr;
use heapless::{String, Vec};
use heapless_bytes::Bytes;

use crate::command::common::data::{Enabled, Integer};
use crate::{bounded_integer_list, enum_list};

use super::data::*;

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestChangeGainResponse {
    #[at_arg(position = 0)]
    pub channels: enum_list!(AudioChannel),
    #[at_arg(position = 1)]
    pub gain_levels: bounded_integer_list!(Gain),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadChangeGainResponse {
    #[at_arg(position = 0)]
    pub gains_levels: Vec<(AudioChannel, Gain), 2>,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ExecuteTimesToInputSimPinResponse {
    #[at_arg(position = 0)]
    pub pin1: Integer,
    #[at_arg(position = 1)]
    pub pin2: Integer,
    #[at_arg(position = 2)]
    pub puk1: Integer,
    #[at_arg(position = 3)]
    pub puk2: Integer,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestDeleteAlarmResponse {
    #[at_arg(position = 0)]
    pub alarms: bounded_integer_list!(Alarm),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestReadAdcResponse {
    #[at_arg(position = 0)]
    pub statuses: enum_list!(AdcStatus),
    #[at_arg(position = 1)]
    pub values: bounded_integer_list!(AdcValue),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadReadAdcResponse {
    #[at_arg(position = 0)]
    pub status: AdcStatus,
    #[at_arg(position = 1)]
    pub value: AdcValue,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestSingleNumberingSchemeResponse {
    #[at_arg(position = 0)]
    pub modes: enum_list!(SingleNumberingScheme),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadSingleNumberingSchemeResponse {
    #[at_arg(position = 0)]
    pub mode: SingleNumberingScheme,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestAlternatingModeCallsResponse {
    #[at_arg(position = 0)]
    pub modes: enum_list!(AlternatingModeCalls),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadAlternatingModeCallsResponse {
    #[at_arg(position = 0)]
    pub mode: AlternatingModeCalls,
}

// TODO
// #[derive(Clone, PartialEq, Eq, AtatResp)]
// pub struct TestGetLocalTimestampResponse {
//     #[at_arg(position = 0)]
//     pub modes: ,
// }

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestLocalDtmfToneGenerationResponse {
    #[at_arg(position = 0)]
    pub duration: bounded_integer_list!(DtmfToneDuration),
    #[at_arg(position = 1)]
    pub dtmfs: enum_list!(Dtmf),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadGetServiceProviderResponse {
    #[at_arg(position = 0)]
    pub service_provider: String<64>,
    #[at_arg(position = 1)]
    pub display_mode: Enabled,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestVoiceMailNumberResponse {
    #[at_arg(position = 0)]
    pub number_max_len: Integer,
    #[at_arg(position = 1)]
    pub alpha_max_len: Integer,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadVoiceMailNumberResponse {
    #[at_arg(position = 0)]
    pub number: Option<VoiceMailNumber>,
    #[at_arg(position = 1)]
    pub alpha: Option<AlphaString>,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestMobileOperationBandResponse {
    #[at_arg(position = 0)]
    pub bands: enum_list!(OperationBand),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadMobileOperationBandResponse {
    #[at_arg(position = 0)]
    pub band: OperationBand,
    #[at_arg(position = 1)]
    pub all_band: Option<OperationBand>,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestHandsFreeResponse {
    #[at_arg(position = 0)]
    pub indicators: enum_list!(Enabled),
    #[at_arg(position = 1)]
    pub states: enum_list!(AudioChannel),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadHandsFreeResponse {
    #[at_arg(position = 0)]
    pub indicator: Enabled,
    #[at_arg(position = 1)]
    pub state: AudioChannel,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ExecuteShowIccidResponse {
    #[at_arg(position = 0)]
    pub iccid: HexStr<u128>,
}

#[derive(Clone, PartialEq, AtatResp)]
pub struct ReadTemperatureResponse {
    #[at_arg(position = 0)]
    pub mode: Enabled,
    // TODO: get atat to deserialize f32
    #[at_arg(position = 1)]
    pub temperature: Bytes<6>,
}

#[derive(Clone, PartialEq, AtatResp)]
pub struct ExecuteSimGroupIdResponse {
    #[at_arg(position = 0)]
    pub gid1: Integer,
    #[at_arg(position = 1)]
    pub gid2: Integer,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestSmsCodeModeResponse {
    #[at_arg(position = 0)]
    pub modes: enum_list!(SmsCodeMode),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadSmsCodeModeResponse {
    #[at_arg(position = 0)]
    pub mode: SmsCodeMode,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadSimInsertedStatusReportingResponse {
    #[at_arg(position = 0)]
    pub state: Enabled,
    #[at_arg(position = 1)]
    pub inserted: SimInserted,
}
