//! AT Commands According to V.25TER
//!
//! These AT Commands are designed according to the ITU-T (International
//! Telecommunication Union, Telecommunication sector) V.25ter document.

pub mod data;
pub mod response;

use atat::atat_derive::AtatCmd;

use data::*;
use response::*;

use crate::command::common::response::NoResponse;

use super::common::data::Enabled;

// do NOT implement 2.2.1 A/ Re-issues the Last Command Given because there is no way to get the response

#[derive(Clone, AtatCmd)]
#[at_cmd("A", NoResponse, timeout_ms = 20000)]
pub struct ExecuteAnswerIncomingCall {}

// TODO: 2.2.3 ATD Mobile Originated Call to Dial A Number
// #[derive(Clone, AtatCmd)]
// #[at_cmd("D", NoResponse, timeout_ms = 20000)]
// pub struct ExecuteMobileOriginatedCall {}

// TODO: 2.2.4 ATD><n> Originate Call to Phone Number in Current Memory
// TODO: 2.2.5 ATD><str> Originate Call to Phone Number in Memory Which Corresponds to Field <str>

#[derive(Clone, AtatCmd)]
#[at_cmd("DL", NoResponse)]
pub struct ExecuteRedialLastNumber {}

#[derive(Clone, AtatCmd)]
#[at_cmd("E", NoResponse)]
pub struct ExecuteSetCommandEchoMode {
    #[at_arg(position = 0)]
    pub value: Enabled,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("H", NoResponse, timeout_ms = 20000)]
pub struct ExecuteDisconnectExistingConnection {}

// TODO: 2.2.9 ATI Display Product Identification Information
// TODO: 2.2.10 ATL Set Monitor speaker loudness
// TODO: 2.2.11 ATM Set Monitor Speaker Mode

#[derive(Clone, AtatCmd)]
#[at_cmd("+++", NoResponse, timeout_ms = 2000, cmd_prefix = "")]
pub struct ExecuteSwitchToCommandMode {}

#[derive(Clone, AtatCmd)]
#[at_cmd("O0", NoResponse)]
pub struct ExecuteSwitchToDataMode {}

#[derive(Clone, AtatCmd)]
#[at_cmd("P", NoResponse)]
pub struct ExecuteSelectPulseDialling {}

// do NOT implement because atat requires result codes
// #[derive(Clone, AtatCmd)]
// #[at_cmd("Q", NoResponse, value_sep = false)]
// pub struct ExecuteSetResultCodePresentationMode {
//     #[at_arg(position = 0)]
//     pub mode: PresentationMode,
// }

#[derive(Clone, AtatCmd)]
#[at_cmd("S0?", ReadRingsBeforeAutomaticallyAnsweringCallResponse)]
pub struct ReadRingsBeforeAutomaticallyAnsweringCall {}

#[derive(Clone, AtatCmd)]
#[at_cmd("S0", NoResponse)]
pub struct WriteRingsBeforeAutomaticallyAnsweringCall {
    #[at_arg(position = 0)]
    pub rings: Rings,
}

// do NOT implement 2.2.17 ATS3 Set Command Line Termination Character because atat only works with <CR>
// do NOT implement 2.2.18 ATS4 Set Response Formatting Character for same reason
// do NOT implement 2.2.19 ATS5 Set Command Line Editing Character because it is not useful
// do NOT implement 2.2.20 ATS6 Pause Before Blind Dialling for same reason

#[derive(Clone, AtatCmd)]
#[at_cmd("S7?", ReadSecondsResponse)]
pub struct ReadSecondsToWaitForConnection {}

#[derive(Clone, AtatCmd)]
#[at_cmd("S7", NoResponse)]
pub struct WriteSecondsToWaitForConnection {
    #[at_arg(position = 0)]
    pub seconds: Seconds,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("S8?", ReadSecondsResponse)]
pub struct ReadSecondsToWaitForCommaDialModifier {}

#[derive(Clone, AtatCmd)]
#[at_cmd("S8", NoResponse)]
pub struct WriteSecondsToWaitForCommaDialModifier {
    #[at_arg(position = 0)]
    pub seconds: Seconds,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("S10?", ReadDisconnectDelayAfterDataCarrierAbsenceResponse)]
pub struct ReadDisconnectDelayAfterDataCarrierAbsence {}

#[derive(Clone, AtatCmd)]
#[at_cmd("S10", NoResponse)]
pub struct WriteDisconnectDelayAfterDataCarrierAbsence {
    #[at_arg(position = 0)]
    pub delay: Deciseconds,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("T", NoResponse)]
pub struct ExecuteSelectToneDialing {}

// do NOT implement 2.2.25 ATV TA Response Format because atat cannot handle the other mode

#[derive(Clone, AtatCmd)]
#[at_cmd("X", NoResponse, value_sep = false)]
pub struct ExecuteSetConnectResultCodeFormat {
    #[at_arg(position = 0)]
    pub value: ConnectResultCodeFormat,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("Z0", NoResponse)]
pub struct ExecuteResetDefaultConfiguration {}

#[derive(Clone, AtatCmd)]
#[at_cmd("&C", NoResponse, value_sep = false)]
pub struct ExecuteSetDcdFunctionMode {
    #[at_arg(position = 0)]
    pub value: DcdOn,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("&D", NoResponse, value_sep = false)]
pub struct ExecuteSetDtrFunctionMode {
    #[at_arg(position = 0)]
    pub value: DtrMode,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("&F0", NoResponse)]
pub struct ExecuteFactoryDefinedConfiguration {}

// TODO: 2.2.31 AT&V Display Current Configuration

#[derive(Clone, AtatCmd)]
#[at_cmd("&W0", NoResponse)]
pub struct ExecuteStoreActiveProfile {}

// TODO: 2.2.33 AT+GCAP Request Complete TA Capabilities List
// TODO: 2.2.34 AT+GMI Request Manufacturer Identification
// TODO: 2.2.35 AT+GMM Request TA Model Identification
// TODO: 2.2.36 AT+GMR Request TA Revision Identification of Software Release
// TODO: 2.2.37 AT+GOI Request Global Object Identification
// TODO: 2.2.38 AT+GSN Request TA Serial Number Identification (IMEI)
// TODO: 2.2.39 AT+ICF Set TE-TA Control Character Framing
// TODO: 2.2.40 AT+IFC Set TE-TA Local Data Flow Control
// TODO: 2.2.41 AT+IPR Set TE-TA Fixed Local Rate

#[derive(Clone, AtatCmd)]
#[at_cmd("+HVOIC", NoResponse)]
pub struct ExecuteDisconnectVoiceCallOnly {}
