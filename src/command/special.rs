//! AT Commands Special for SIMCom

pub mod data;
pub mod response;

use atat::atat_derive::AtatCmd;

use data::*;
use response::*;

use crate::command::common::response::NoResponse;

use super::common::{
    data::Enabled,
    response::{ReadEnabledResponse, TestEnabledResponse},
};

#[derive(Clone, AtatCmd)]
#[at_cmd("+SIDET=?", TestChangeGainResponse)]
pub struct TestChangeSideToneGain {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+SIDET?", ReadChangeGainResponse)]
pub struct ReadChangeSideToneGain {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+SIDET", NoResponse)]
pub struct WriteChangeSideToneGain {
    #[at_arg(position = 0)]
    pub channel: AudioChannel,
    #[at_arg(position = 1)]
    pub gain: Gain,
}

// TODO handle [NORMAL POWER DOWN] response
#[derive(Clone, AtatCmd)]
#[at_cmd("+CPOWD", NoResponse)]
pub struct WritePowerOff {
    #[at_arg(position = 0)]
    pub mode: PowerOffMode,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+SPIC", NoResponse)]
pub struct ExecuteTimesToInputSimPin {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CMIC=?", TestChangeGainResponse)]
pub struct TestChangeMicrophoneGain {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CMIC?", ReadChangeGainResponse)]
pub struct ReadChangeMicrophoneGain {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CMIC", NoResponse)]
pub struct WriteChangeMicrophoneGain {
    #[at_arg(position = 0)]
    pub channel: AudioChannel,
    #[at_arg(position = 1)]
    pub gain: Gain,
}

// TODO
// #[derive(Clone, AtatCmd)]
// #[at_cmd("+CALA=?", TestSetAlarmTimeResponse)]
// pub struct TestSetAlarmTime {}

// TODO
// #[derive(Clone, AtatCmd)]
// #[at_cmd("+CALA?", ReadSetAlarmTimeResponse)]
// pub struct ReadSetAlarmTime {}

// TODO
// #[derive(Clone, AtatCmd)]
// #[at_cmd("+CALA", NoResponse)]
// pub struct WriteSetAlarmTime {
//     #[at_arg(position = 0)]
//     pub time: Integer,
//     #[at_arg(position = 1)]
//     pub alarm: Option<Alarm>,
//     #[at_arg(position = 2)]
//     pub recurr: Option<Integer>,
// }

#[derive(Clone, AtatCmd)]
#[at_cmd("+CALD=?", TestDeleteAlarmResponse)]
pub struct TestDeleteAlarm {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CALD", NoResponse)]
pub struct WriteDeleteAlarm {
    #[at_arg(position = 0)]
    pub alarm: Alarm,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CADC=?", TestReadAdcResponse)]
pub struct TestReadAdc {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CADC?", ReadReadAdcResponse, timeout_ms = 2000)]
pub struct ReadReadAdc {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CSNS=?", TestSingleNumberingSchemeResponse)]
pub struct TestSingleNumberingScheme {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CSNS?", ReadSingleNumberingSchemeResponse)]
pub struct ReadSingleNumberingScheme {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CSNS", NoResponse)]
pub struct WriteSingleNumberingScheme {
    #[at_arg(position = 0)]
    pub mode: SingleNumberingScheme,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CDSCB", NoResponse)]
pub struct ExecuteResetCellBroadcast {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CMOD=?", TestAlternatingModeCallsResponse)]
pub struct TestAlternatingModeCalls {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CMOD?", ReadAlternatingModeCallsResponse)]
pub struct ReadAlternatingModeCalls {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CMOD", NoResponse)]
pub struct WriteAlternatingModeCalls {
    #[at_arg(position = 0)]
    pub mode: AlternatingModeCalls,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CFGRI=?", TestEnabledResponse)]
pub struct TestIndicateRi {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CFGRI?", ReadEnabledResponse)]
pub struct ReadIndicateRi {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CFGRI", NoResponse)]
pub struct WriteIndicateRi {
    #[at_arg(position = 0)]
    pub status: Enabled,
}

// TODO
// #[derive(Clone, AtatCmd)]
// #[at_cmd("+CLTS=?", TestGetLocalTimestampResponse)]
// pub struct TestGetLocalTimestamp {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CLTS?", ReadEnabledResponse)]
pub struct ReadGetLocalTimestamp {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CLTS", NoResponse)]
pub struct WriteGetLocalTimestamp {
    #[at_arg(position = 0)]
    pub mode: Enabled,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CLDTMF=?", TestLocalDtmfToneGenerationResponse)]
pub struct TestLocalDtmfToneGeneration {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CLDTMF", NoResponse)]
pub struct WriteLocalDtmfToneGeneration {
    #[at_arg(position = 0)]
    pub duration: DtmfToneDuration,
    #[at_arg(position = 1)]
    pub dtmf_string: DtmfString,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CLDTMF", NoResponse)]
pub struct ExecuteLocalDtmfToneGeneration {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CDRIND=?", TestEnabledResponse)]
pub struct TestCallTerminationIndication {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CDRIND?", ReadEnabledResponse)]
pub struct ReadCallTerminationIndication {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CDRIND", NoResponse)]
pub struct WriteCallTerminationIndication {
    #[at_arg(position = 0)]
    pub state: Enabled,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CSPN?", ReadGetServiceProviderResponse)]
pub struct ReadGetServiceProvider {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CCVM=?", TestVoiceMailNumberResponse)]
pub struct TestVoiceMailNumber {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CCVM?", ReadVoiceMailNumberResponse)]
pub struct ReadVoiceMailNumber {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CCVM", NoResponse)]
pub struct WriteVoiceMailNumber {
    #[at_arg(position = 0)]
    pub number: VoiceMailNumber,
    #[at_arg(position = 1)]
    pub alpha: Option<AlphaString>,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CBAND=?", TestMobileOperationBandResponse)]
pub struct TestMobileOperationBand {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CBAND?", ReadMobileOperationBandResponse)]
pub struct ReadMobileOperationBand {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CBAND", NoResponse)]
pub struct WriteMobileOperationBand {
    #[at_arg(position = 0)]
    pub band: OperationBand,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CHF=?", TestHandsFreeResponse)]
pub struct TestHandsFree {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CHF?", ReadHandsFreeResponse)]
pub struct ReadHandsFree {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CHF", NoResponse)]
pub struct WriteHandsFree {
    #[at_arg(position = 0)]
    pub indicator: Enabled,
    #[at_arg(position = 1)]
    pub state: Option<AudioChannel>,
}

// TODO: 6.2.19 AT+CHFA Swap the Audio Channels

#[derive(Clone, AtatCmd)]
#[at_cmd("+CSCLK=?", TestEnabledResponse)]
pub struct TestSlowClock {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CSCLK?", ReadEnabledResponse)]
pub struct ReadSlowClock {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CSCLK", NoResponse)]
pub struct WriteSlowClock {
    #[at_arg(position = 0)]
    pub state: Enabled,
}

// TODO: 6.2.21 AT+CENG Switch On or Off Engineering Mode

#[derive(Clone, AtatCmd)]
#[at_cmd("+SCLASS0=?", TestEnabledResponse)]
pub struct TestStoreClass0Sms {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+SCLASS0?", ReadEnabledResponse)]
pub struct ReadStoreClass0Sms {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+SCLASS0", NoResponse)]
pub struct WriteStoreClass0Sms {
    #[at_arg(position = 0)]
    pub mode: Enabled,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CCID=?", NoResponse)]
pub struct TestShowIccid {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CCID", ExecuteShowIccidResponse, timeout_ms = 2000)]
pub struct ExecuteShowIccid {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CMTE?", ReadTemperatureResponse)]
pub struct ReadTemperature {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CMTE", NoResponse)]
pub struct WriteTemperature {
    #[at_arg(position = 0)]
    pub mode: Enabled,
}

// TODO: 6.2.25 AT+CMGDA Delete All SMS
// TODO: 6.2.26 AT+STTONE Play SIM Toolkit Tone
// TODO: 6.2.27 AT+SIMTONE Generate Specifically Tone

#[derive(Clone, AtatCmd)]
#[at_cmd("+CCPD=?", TestEnabledResponse)]
pub struct TestAlphaString {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CCPD?", ReadEnabledResponse)]
pub struct ReadAlphaString {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CCPD", NoResponse)]
pub struct WriteAlphaString {
    #[at_arg(position = 0)]
    pub mode: Enabled,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CGID", ExecuteSimGroupIdResponse)]
pub struct ExecuteSimGroupId {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+MORING=?", TestEnabledResponse)]
pub struct TestShowMobileOriginatedCall {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+MORING?", ReadEnabledResponse)]
pub struct ReadShowMobileOriginatedCall {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+MORING", NoResponse)]
pub struct WriteShowMobileOriginatedCall {
    #[at_arg(position = 0)]
    pub mode: Enabled,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CMGHEX=?", TestEnabledResponse)]
pub struct TestNonAsciiSms {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CMGHEX?", ReadEnabledResponse)]
pub struct ReadNonAsciiSms {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CMGHEX", NoResponse)]
pub struct WriteNonAsciiSms {
    #[at_arg(position = 0)]
    pub mode: Enabled,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CCODE=?", TestSmsCodeModeResponse)]
pub struct TestSmsCodeMode {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CCODE?", ReadSmsCodeModeResponse)]
pub struct ReadSmsCodeMode {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CCODE", NoResponse)]
pub struct WriteSmsCodeMode {
    #[at_arg(position = 0)]
    pub mode: SmsCodeMode,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIURC=?", TestEnabledResponse)]
pub struct TestInitialUrcPresentation {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIURC?", ReadEnabledResponse)]
pub struct ReadInitialUrcPresentation {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIURC", NoResponse)]
pub struct WriteInitialUrcPresentation {
    #[at_arg(position = 0)]
    pub mode: Enabled,
}

// TODO: 6.2.34 AT+CPSPWD Change PS Super Password
// TODO: 6.2.35 AT+EXUNSOL Enable or Disable Proprietary Unsolicited Indications
// TODO: 6.2.36 AT+CGMSCLASS Change GPRS Multislot Class
// TODO: 6.2.37 AT+CDEVICE View Current Flash Device Type

#[derive(Clone, AtatCmd)]
#[at_cmd("+CCALR=?", TestEnabledResponse)]
pub struct TestCallReadyQuery {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CCALR?", ReadEnabledResponse)]
pub struct ReadCallReadyQuery {}

// TODO: 6.2.39 AT+GSV Display Product Identification Information
// TODO: 6.2.40 AT+SGPIO Control the GPIO
// TODO: 6.2.41 AT+SPWM Generate the Pulse-Width-Modulation
// TODO: 6.2.42 AT+ECHO Echo Cancellation Control
// TODO: 6.2.43 AT+CAAS Control Auto Audio Switch
// TODO: 6.2.44 AT+SVR Configure Voice Coding Type for Voice Calls
// TODO: 6.2.45 AT+GSMBUSY Reject Incoming Call
// TODO: 6.2.46 AT+CEMNL Set the List of Emergency Number
// TODO: 6.2.47 AT*CELLLOCK Set the List of ARFCN Which Needs to Be Locked
// TODO: 6.2.48 AT+SLEDS Set the Timer Period of Net Light

#[derive(Clone, AtatCmd)]
#[at_cmd("+CBUZZERRING?", ReadEnabledResponse)]
pub struct ReadUseBuzzer {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CBUZZERRING", NoResponse)]
pub struct WriteUseBuzzer {
    #[at_arg(position = 0)]
    pub mode: Enabled,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CEXTERNTONE=?", TestEnabledResponse)]
pub struct TestCloseMicrophone {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CEXTERNTONE?", ReadEnabledResponse)]
pub struct ReadCloseMicrophone {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CEXTERNTONE", NoResponse)]
pub struct WriteCloseMicrophone {
    #[at_arg(position = 0)]
    pub mode: Enabled,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CNETLIGHT", NoResponse)]
pub struct WriteOpenNetLight {
    #[at_arg(position = 0)]
    pub mode: Enabled,
}

// TODO: 6.2.52 AT+CWHITELIST Set the White List

#[derive(Clone, AtatCmd)]
#[at_cmd("+CSDT=?", TestEnabledResponse)]
pub struct TestDetectingSimCard {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CSDT?", ReadEnabledResponse)]
pub struct ReadDetectingSimCard {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CSDT", NoResponse)]
pub struct WriteDetectingSimCard {
    #[at_arg(position = 0)]
    pub mode: Enabled,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CSMINS=?", TestEnabledResponse)]
pub struct TestSimInsertedStatusReporting {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CSMINS?", ReadSimInsertedStatusReportingResponse)]
pub struct ReadSimInsertedStatusReporting {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CSMINS", NoResponse)]
pub struct WriteSimInsertedStatusReporting {
    #[at_arg(position = 0)]
    pub state: Enabled,
}

// TODO: 6.2.55 AT+CSGS Netlight Indication of GPRS Status
// TODO: 6.2.56 AT+CMICBIAS Close or Open the MICBIAS
