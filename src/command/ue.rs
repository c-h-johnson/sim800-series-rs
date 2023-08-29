//! AT Commands According to 3GPP TS 27.007
//!
//! The 3GPP TS 27.007 commands are for User Equipment (UE).

pub mod data;
pub mod response;

use atat::atat_derive::AtatCmd;
use data::*;
use response::*;

use crate::command::common::response::NoResponse;

// TODO: 3.2.1 AT+CACM Accumulated Call Meter (ACM) Reset or Query
// TODO: 3.2.2 AT+CAMM Accumulated Call Meter Maximum (ACM max) Set or Query
// TODO: 3.2.3 AT+CAOC Advice of Charge
// TODO: 3.2.4 AT+CBST Select Bearer Service Type
// TODO: 3.2.5 AT+CCFC Call Forwarding Number and Conditions Control
// TODO: 3.2.6 AT+CCWA Call Waiting Control
// TODO: 3.2.7 AT+CEER Extended Error Report
// TODO: 3.2.8 AT+CGMI Request Manufacturer Identification
// TODO: 3.2.9 AT+CGMM Request Model Identification
// TODO: 3.2.10 AT+CGMR Request TA Revision Identification of Software Release
// TODO: 3.2.11 AT+CGSN Request Product Serial Number Identification (Identical with +GSN)

#[derive(Clone, AtatCmd)]
#[at_cmd("+CSCS=?", TestSelectTeCharacterSetResponse)]
pub struct TestSelectTeCharacterSet {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CSCS?", ReadSelectTeCharacterSetResponse)]
pub struct ReadSelectTeCharacterSet {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CSCS", NoResponse)]
pub struct WriteSelectTeCharacterSet {
    #[at_arg(position = 0)]
    pub chset: CharacterSet,
}

// TODO: 3.2.13 AT+CSTA Select Type of Address
// TODO: 3.2.14 AT+CHLD Call Hold and Multiparty
// TODO: 3.2.15 AT+CIMI Request International Mobile Subscriber Identity
// TODO: 3.2.16 AT+CLCC List Current Calls of ME
// TODO: 3.2.17 AT+CLCK Facility Lock
// TODO: 3.2.18 AT+CLIP Calling Line Identification Presentation
// TODO: 3.2.19 AT+CLIR Calling Line Identification Restriction
// TODO: 3.2.20 AT+CMEE Report Mobile Equipment Error
// TODO: 3.2.21 AT+COLP Connected Line Identification Presentation

#[derive(Clone, AtatCmd)]
#[at_cmd("+COPS=?", TestOperatorSelectionResponse, timeout_ms = 45000)]
pub struct TestOperatorSelection {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+COPS?", ReadOperatorSelectionResponse)]
pub struct ReadOperatorSelection {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+COPS", NoResponse, timeout_ms = 120000)]
pub struct WriteOperatorSelection {
    #[at_arg(position = 0)]
    pub mode: OperatorMode,
    #[at_arg(position = 1)]
    pub format: Option<OperatorFormat>,
    #[at_arg(position = 2)]
    pub operator: Option<Operator>,
}

// TODO: 3.2.23 AT+CPAS Phone Activity Status
// TODO: 3.2.24 AT+CPBF Find Phonebook Entries
// TODO: 3.2.25 AT+CPBR Read Current Phonebook Entries
// TODO: 3.2.26 AT+CPBS Select Phonebook Memory Storage
// TODO: 3.2.27 AT+CPBW Write Phonebook Entry
// TODO: 3.2.28 AT+CPIN Enter PIN
// TODO: 3.2.29 AT+CPWD Change Password
// TODO: 3.2.30 AT+CR Service Reporting Control
// TODO: 3.2.31 AT+CRC Set Cellular Result Codes for Incoming Call Indication

#[derive(Clone, AtatCmd)]
#[at_cmd("+CREG=?", TestNetworkRegistrationResponse)]
pub struct TestNetworkRegistration {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CREG?", ReadNetworkRegistrationResponse)]
pub struct ReadNetworkRegistration {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CREG", NoResponse)]
pub struct WriteNetworkRegistration {
    #[at_arg(position = 0)]
    pub urc_mode: NetworkRegistrationUrcMode,
}

// TODO: 3.2.33 AT+CRLP Select Radio Link Protocol Parameters
// TODO: 3.2.34 AT+CRSM Restricted SIM Access

#[derive(Clone, AtatCmd)]
#[at_cmd("+CSQ=?", TestSignalQualityReportResponse)]
pub struct TestSignalQualityReport {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CSQ", ExecuteSignalQualityReportResponse)]
pub struct ExecuteSignalQualityReport {}

// TODO: 3.2.36 AT+FCLASS FAX: Select, Read or Test Service Class
// TODO: 3.2.37 AT+FMI FAX: Report Manufactured ID
// TODO: 3.2.38 AT+FMM FAX: Report Model ID
// TODO: 3.2.39 AT+FMR FAX: Report Revision ID
// TODO: 3.2.40 AT+VTD Tone Duration
// TODO: 3.2.41 AT+VTS DTMF and Tone Generation
// TODO: 3.2.42 AT+CMUX Multiplexer Control
// TODO: 3.2.43 AT+CNUM Subscriber Number
// TODO: 3.2.44 AT+CPOL Preferred Operator List

#[derive(Clone, AtatCmd)]
#[at_cmd("+COPN=?", NoResponse)]
pub struct TestReadOperatorNames {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+COPN", ExecuteReadOperatorNamesResponse)]
pub struct ExecuteReadOperatorNames {}

// TODO: 3.2.46 AT+CFUN Set Phone Functionality
// TODO: 3.2.47 AT+CCLK Clock
// TODO: 3.2.48 AT+CSIM Generic SIM Access
// TODO: 3.2.49 AT+CALM Alert Sound Mode
// TODO: 3.2.50 AT+CALS Alert Sound Select
// TODO: 3.2.51 AT+CRSL Ringer Sound Level
// TODO: 3.2.52 AT+CLVL Loud Speaker Volume Level
// TODO: 3.2.53 AT+CMUT Mute Control
// TODO: 3.2.54 AT+CPUC Price Per Unit and Currency Table
// TODO: 3.2.55 AT+CCWE Call Meter Maximum Event

#[derive(Clone, AtatCmd)]
#[at_cmd("+CBC=?", TestBatteryChargeResponse)]
pub struct TestBatteryCharge {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CBC", ExecuteBatteryChargeResponse)]
pub struct ExecuteBatteryCharge {}

// TODO: 3.2.57 AT+CUSD Unstructured Supplementary Service Data
// TODO: 3.2.58 AT+CSSN Supplementary Services Notification
